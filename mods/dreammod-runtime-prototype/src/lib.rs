//! Candidate runtime prototype. This is an evaluation harness, not the final
//! DreamMod language decision or production sandbox.
use rhai::{AST, Engine, EvalAltResult, ParseError, Scope};

#[derive(Clone, Copy, Debug)]
pub struct RuntimeLimits {
    pub max_operations: u64,
    pub max_call_levels: usize,
}

impl Default for RuntimeLimits {
    fn default() -> Self {
        Self {
            max_operations: 250_000,
            max_call_levels: 32,
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

    pub fn parse_with_limits(
        source: &str,
        limits: RuntimeLimits,
    ) -> Result<Self, ParseError> {
        let mut engine = Engine::new();
        engine.set_max_operations(limits.max_operations);
        engine.set_max_call_levels(limits.max_call_levels);

        // No filesystem, network, process or native-library functions are registered.
        // Production DreamMod will expose only capability-gated host functions.
        let ast = engine.compile(source)?;
        Ok(Self { engine, ast })
    }

    pub fn call_i64(
        &self,
        function: &str,
        argument: i64,
    ) -> Result<i64, Box<EvalAltResult>> {
        self.engine
            .call_fn(&mut Scope::new(), &self.ast, function, (argument,))
    }

    pub fn call_no_args_i64(
        &self,
        function: &str,
    ) -> Result<i64, Box<EvalAltResult>> {
        self.engine
            .call_fn(&mut Scope::new(), &self.ast, function, ())
    }

    pub fn call_physics_batch(
        &self,
        positions: Vec<f64>,
        velocities: Vec<f64>,
        delta_seconds: f64,
    ) -> Result<Vec<f64>, Box<EvalAltResult>> {
        self.engine.call_fn(
            &mut Scope::new(),
            &self.ast,
            "physics_step",
            (positions, velocities, delta_seconds),
        )
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
    engine.register_fn("world_health_batch", || vec![10_i64, 20, 30, 40]);
    engine
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interprets_source_and_dispatches_a_game_event() {
        let program = RhaiPrototype::parse(
            "fn on_player_damage(amount) { amount - 3 }",
        )
        .unwrap();

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
                for i in 0..positions.len {
                    next.push(positions[i] + velocities[i] * delta_seconds);
                }
                next
            }
            "#,
        )
        .unwrap();

        let result = program
            .call_physics_batch(
                vec![0.0, 10.0, -2.0],
                vec![2.0, -1.0, 4.0],
                0.5,
            )
            .unwrap();

        assert_eq!(result, vec![1.0, 9.5, 0.0]);
    }

    #[test]
    fn execution_budget_stops_an_infinite_loop() {
        let program = RhaiPrototype::parse_with_limits(
            "loop { }",
            RuntimeLimits {
                max_operations: 128,
                max_call_levels: 8,
            },
        )
        .unwrap();

        assert!(program.run().is_err());
    }
}
