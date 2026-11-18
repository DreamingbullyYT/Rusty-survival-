use std::{collections::BTreeSet, sync::Arc};

use rhai::{AST, Dynamic, Engine, Scope};
use serde_json::Value;
use thiserror::Error;

#[derive(Clone, Copy, Debug)]
pub struct ExecutionLimits {
    pub max_operations: u64,
    pub max_call_levels: usize,
    pub max_array_size: usize,
    pub max_map_size: usize,
    pub max_string_size: usize,
    pub max_variables: usize,
    pub max_functions: usize,
    pub max_expr_depth: usize,
    pub max_function_expr_depth: usize,
}

impl Default for ExecutionLimits {
    fn default() -> Self {
        Self {
            max_operations: 100_000,
            max_call_levels: 32,
            max_array_size: 4_096,
            max_map_size: 512,
            max_string_size: 64 * 1024,
            max_variables: 256,
            max_functions: 128,
            max_expr_depth: 64,
            max_function_expr_depth: 32,
        }
    }
}

#[derive(Debug, Error)]
pub enum ScriptError {
    #[error("script parse failed: {0}")]
    Parse(String),
    #[error("script execution failed: {0}")]
    Execution(String),
    #[error("script value conversion failed: {0}")]
    Conversion(String),
}

pub trait ScriptEngine {
    type Program: Send + Sync + 'static;
    type Instance: Send + 'static;

    fn parse_source(&self, source: &str) -> Result<Arc<Self::Program>, ScriptError>;
    fn instantiate(&self, program: Arc<Self::Program>) -> Self::Instance;
    fn call(
        &self,
        instance: &mut Self::Instance,
        function: &str,
        event: &Value,
    ) -> Result<Option<Value>, ScriptError>;
}

/// Provisional source interpreter. Rhai evaluates its AST directly; this
/// adapter deliberately does not emit bytecode or machine code.
#[derive(Clone, Copy, Debug, Default)]
pub struct RhaiEngine {
    limits: ExecutionLimits,
}

impl RhaiEngine {
    pub fn new(limits: ExecutionLimits) -> Self {
        Self { limits }
    }

    pub fn limits(&self) -> ExecutionLimits {
        self.limits
    }
}

pub struct RhaiProgram {
    engine: Engine,
    ast: AST,
    functions: BTreeSet<String>,
}

pub struct RhaiInstance {
    program: Arc<RhaiProgram>,
    scope: Scope<'static>,
}

impl ScriptEngine for RhaiEngine {
    type Program = RhaiProgram;
    type Instance = RhaiInstance;

    fn parse_source(&self, source: &str) -> Result<Arc<Self::Program>, ScriptError> {
        let mut engine = Engine::new();
        engine.set_max_operations(self.limits.max_operations);
        engine.set_max_call_levels(self.limits.max_call_levels);
        engine.set_max_array_size(self.limits.max_array_size);
        engine.set_max_map_size(self.limits.max_map_size);
        engine.set_max_string_size(self.limits.max_string_size);
        engine.set_max_variables(self.limits.max_variables);
        engine.set_max_functions(self.limits.max_functions);
        engine.set_max_expr_depths(
            self.limits.max_expr_depth,
            self.limits.max_function_expr_depth,
        );

        // Scripts receive only event data and return command data. They do not
        // get a filesystem module resolver, dynamic eval, or console functions.
        engine.disable_symbol("eval");
        engine.disable_symbol("import");
        engine.disable_symbol("export");
        engine.disable_symbol("print");
        engine.disable_symbol("debug");

        let ast = engine
            .compile(source)
            .map_err(|error| ScriptError::Parse(error.to_string()))?;
        let functions = ast
            .iter_functions()
            .map(|function| function.name.to_string())
            .collect();

        Ok(Arc::new(RhaiProgram {
            engine,
            ast,
            functions,
        }))
    }

    fn instantiate(&self, program: Arc<Self::Program>) -> Self::Instance {
        RhaiInstance {
            program,
            scope: Scope::new(),
        }
    }

    fn call(
        &self,
        instance: &mut Self::Instance,
        function: &str,
        event: &Value,
    ) -> Result<Option<Value>, ScriptError> {
        if !instance.program.functions.contains(function) {
            return Ok(None);
        }

        let event = rhai::serde::to_dynamic(event)
            .map_err(|error| ScriptError::Conversion(error.to_string()))?;
        let result = instance
            .program
            .engine
            .call_fn::<Dynamic>(
                &mut instance.scope,
                &instance.program.ast,
                function,
                (event,),
            )
            .map_err(|error| ScriptError::Execution(error.to_string()))?;

        if result.is_unit() {
            return Ok(None);
        }

        let value = rhai::serde::from_dynamic::<Value>(&result)
            .map_err(|error| ScriptError::Conversion(error.to_string()))?;
        Ok(Some(value))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn evaluates_source_ast_and_returns_json_commands() {
        let engine = RhaiEngine::default();
        let program = engine
            .parse_source(
                r#"
                fn on_server_tick(event) {
                    #{
                        commands: [
                            #{
                                namespace: "world",
                                name: "set_block",
                                payload: #{ x: event.x, y: 64 }
                            }
                        ]
                    }
                }
                "#,
            )
            .unwrap();
        let mut instance = engine.instantiate(program);

        let result = engine
            .call(&mut instance, "on_server_tick", &json!({ "x": 12 }))
            .unwrap()
            .unwrap();

        assert_eq!(result["commands"][0]["payload"]["x"], 12);
        assert_eq!(result["commands"][0]["payload"]["y"], 64);
    }

    #[test]
    fn missing_handler_is_not_an_execution_error() {
        let engine = RhaiEngine::default();
        let program = engine.parse_source("fn on_load(event) { () }").unwrap();
        let mut instance = engine.instantiate(program);

        assert!(
            engine
                .call(&mut instance, "on_server_tick", &json!({}))
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn operation_budget_stops_unbounded_script() {
        let engine = RhaiEngine::new(ExecutionLimits {
            max_operations: 128,
            ..ExecutionLimits::default()
        });
        let program = engine
            .parse_source("fn on_server_tick(event) { loop { } }")
            .unwrap();
        let mut instance = engine.instantiate(program);

        assert!(
            engine
                .call(&mut instance, "on_server_tick", &json!({}))
                .is_err()
        );
    }

    #[test]
    fn eval_symbol_is_rejected_during_parsing() {
        let engine = RhaiEngine::default();
        assert!(
            engine
                .parse_source(r#"fn on_load(event) { eval("1 + 1") }"#)
                .is_err()
        );
    }
}
