//! Candidate runtime prototype. This is an evaluation harness, not the final
//! DreamMod language decision or production sandbox.
use rhai::{AST, Array, Engine, EvalAltResult, ParseError, Scope};

const MAX_PHYSICS_BATCH_ITEMS: usize = 100_000;

fn batch_error(message: &str) -> Box<EvalAltResult> {
    EvalAltResult::ErrorRuntime(message.into(), rhai::Position::NONE).into()
}

#[derive(Clone, Copy, Debug)]
pub struct RuntimeLimits {
    pub max_operations: u64,
    pub max_call_levels: usize,
    pub max_array_items: usize,
    pub max_map_entries: usize,
    pub max_string_bytes: usize,
    pub max_variables: usize,
    pub max_functions: usize,
}

impl Default for RuntimeLimits {
    fn default() -> Self {
        Self {
            max_operations: 250_000,
            max_call_levels: 32,
            max_array_items: 100_000,
            max_map_entries: 16_384,
            max_string_bytes: 1_048_576,
            max_variables: 1_024,
            max_functions: 256,
        }
    }
}

/// A source script parsed to an AST and executed by Rhai's AST evaluator.
/// The optional Grain bytecode feature is deliberately not enabled.
pub struct RhaiPrototype {
    engine: Engine,
    ast: AST,
}

impl RhaiPrototype {
    pub fn parse(source: &str) -> Result<Self, ParseError> {
        Self::parse_with_limits(source, RuntimeLimits::default())
    }

    pub fn parse_with_limits(source: &str, limits: RuntimeLimits) -> Result<Self, ParseError> {
        let mut engine = Engine::new();
        engine.set_max_operations(limits.max_operations);
        engine.set_max_call_levels(limits.max_call_levels);
        engine.set_max_array_size(limits.max_array_items);
        engine.set_max_map_size(limits.max_map_entries);
        engine.set_max_string_size(limits.max_string_bytes);
        engine.set_max_variables(limits.max_variables);
        engine.set_max_functions(limits.max_functions);

        // No filesystem, network, process or native-library functions are registered.
        // Production DreamMod will expose only capability-gated host functions.
        let ast = engine.compile(source)?;
        Ok(Self { engine, ast })
    }

    pub fn call_i64(&self, function: &str, argument: i64) -> Result<i64, Box<EvalAltResult>> {
        self.engine
            .call_fn(&mut Scope::new(), &self.ast, function, (argument,))
    }

    pub fn call_no_args_i64(&self, function: &str) -> Result<i64, Box<EvalAltResult>> {
        self.engine
            .call_fn(&mut Scope::new(), &self.ast, function, ())
    }

    pub fn call_physics_batch(
        &self,
        positions: Vec<f64>,
        velocities: Vec<f64>,
        delta_seconds: f64,
    ) -> Result<Vec<f64>, Box<EvalAltResult>> {
        if positions.len() != velocities.len() {
            return Err(batch_error("positions and velocities must have equal lengths"));
        }
        if positions.len() > MAX_PHYSICS_BATCH_ITEMS {
            return Err(batch_error("physics batch exceeds the configured item limit"));
        }
        if !delta_seconds.is_finite()
            || positions.iter().any(|value| !value.is_finite())
            || velocities.iter().any(|value| !value.is_finite())
        {
            return Err(batch_error("physics inputs must contain only finite numbers"));
        }
        let expected_len = positions.len();
        let positions = positions.into_iter().map(Into::into).collect::<Array>();
        let velocities = velocities.into_iter().map(Into::into).collect::<Array>();
        let result: Array = self.engine.call_fn(
            &mut Scope::new(),
            &self.ast,
            "physics_step",
            (positions, velocities, delta_seconds),
        )?;
        if result.len() != expected_len {
            return Err(batch_error("physics provider returned an unexpected result length"));
        }
        result
            .into_iter()
            .map(|value| {
                let number = value.as_float().map_err(|_| {
                    EvalAltResult::ErrorMismatchDataType(
                        "f64".into(),
                        value.type_name().into(),
                        rhai::Position::NONE,
                    )
                    .into()
                })?;
                if !number.is_finite() {
                    return Err(batch_error("physics provider returned a non-finite number"));
                }
                Ok(number)
            })
            .collect()
    }

    pub fn run(&self) -> Result<(), Box<EvalAltResult>> {
        self.engine.run_ast(&self.ast)
    }
}

/// Construct an engine with a deliberately narrow, batch-oriented host surface.
/// Used by the tests to model a single coarse-grained world query.
pub fn engine_with_test_world_api() -> Engine {
    let mut engine = Engine::new();
    engine.set_max_operations(250_000);
    engine.set_max_call_levels(32);
    engine.set_max_array_size(100_000);
    engine.set_max_map_size(16_384);
    engine.set_max_string_size(1_048_576);
    engine.set_max_variables(1_024);
    engine.set_max_functions(256);
    engine.register_fn("world_health_batch", || {
        vec![10_i64, 20, 30, 40]
            .into_iter()
            .map(Into::into)
            .collect::<Array>()
    });
    engine
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interprets_source_and_dispatches_a_game_event() {
        let program = RhaiPrototype::parse("fn on_player_damage(amount) { amount - 3 }").unwrap();

        assert_eq!(program.call_i64("on_player_damage", 12).unwrap(), 9);
    }

    #[test]
    fn host_queries_are_coarse_grained_batches() {
        let engine = engine_with_test_world_api();
        let ast = engine
            .compile(
                r#"
                fn total_health() {
                    let total = 0;
                    for health in world_health_batch() {
                        total += health;
                    }
                    total
                }
                "#,
            )
            .unwrap();

        let total = engine
            .call_fn::<i64>(&mut Scope::new(), &ast, "total_health", ())
            .unwrap();
        assert_eq!(total, 100);
    }

    #[test]
    fn one_script_call_can_replace_a_batch_algorithm() {
        let program = RhaiPrototype::parse(
            r#"
            fn physics_step(positions, velocities, delta_seconds) {
                let next = [];
                for i in 0..positions.len() {
                    next.push(positions[i] + velocities[i] * delta_seconds);
                }
                next
            }
            "#,
        )
        .unwrap();

        let result = program
            .call_physics_batch(vec![0.0, 10.0, -2.0], vec![2.0, -1.0, 4.0], 0.5)
            .unwrap();

        assert_eq!(result, vec![1.0, 9.5, 0.0]);
    }

    #[test]
    fn array_budget_stops_unbounded_script_allocation() {
        let program = RhaiPrototype::parse_with_limits(
            "let values = []; for i in 0..32 { values.push(i); }",
            RuntimeLimits {
                max_array_items: 8,
                ..RuntimeLimits::default()
            },
        )
        .unwrap();

        assert!(program.run().is_err());
    }

    #[test]
    fn execution_budget_stops_an_infinite_loop() {
        let program = RhaiPrototype::parse_with_limits(
            "loop { }",
            RuntimeLimits {
                max_operations: 128,
                max_call_levels: 8,
                ..RuntimeLimits::default()
            },
        )
        .unwrap();

        assert!(program.run().is_err());
    }
}
