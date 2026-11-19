# Scoped API Provider Execution Context

## Status

Design accepted for implementation; no code integration or test verification is claimed yet.

## Problem

DreamMod providers currently receive an `ApiRequest` only. A provider cannot safely use a host object that exists only during an engine callback (such as a Horizon hook's typed Level host). Global mutable host state, raw pointers, fabricated snapshots, and storing callback handles beyond their lifetime are prohibited.

## Decision

Pass an **operation-scoped host context** into context-aware provider invocation. The context is bound by the engine/runtime to the already-authorized operation being dispatched. It is borrowed only for that invocation and cannot be retained by a provider through the safe Rust API.

The interface is engine-neutral. Horizon-specific code implements the context by translating the operation into the current Horizon hook's typed host calls; the base runtime has no Horizon SDK dependency.

## Contract (conceptual)

```rust
pub trait ApiOperationContext {
    fn execute(&mut self, input: &serde_json::Value)
        -> Result<serde_json::Value, String>;
}

pub trait ApiProvider: Send {
    fn descriptor(&self) -> &ApiProviderDescriptor;
    fn invoke(&mut self, request: &ApiRequest) -> Result<Value, String>;

    // New context-aware entrypoint, with a compatibility default:
    fn invoke_with_context(
        &mut self,
        request: &ApiRequest,
        context: &mut dyn ApiOperationContext,
    ) -> Result<Value, String> {
        self.invoke(request)
    }
}
```

The provider router adds a context-aware call path. The existing context-free call remains available for providers that are self-contained. Context-required adapters must fail clearly if invoked without a valid scoped context; they must not invent or cache a host context.

The real code may use a different signature if object-safety, borrowing, or ergonomics require it, but must preserve these invariants.

## Safety and routing invariants

1. The runtime validates the operation and capability before provider invocation.
2. Context is bound to the selected operation and its safety classification; it cannot dispatch a different operation.
3. Context implementations validate input schemas, bounds, ownership and engine state before accessing/mutating authoritative state.
4. Read-only operations must not mutate; mutating operations follow the engine's transaction/atomicity rules.
5. Provider and context errors are returned to the caller. Do not automatically retry another provider after failure (side effects may have occurred).
6. Context borrowing is limited to the invocation; no globals, raw host pointers, unsafe escape, or stale snapshots.
7. Providers remain hot-swappable by routing. Stateful provider replacement still requires explicit migration/snapshot/rollback contracts.
8. Context and adapter code must not require Horizon in the base runtime. The optional Horizon adapter is version-pinned and advertises only verified operations.
9. Existing providers using `invoke` remain compatible via the default context-aware method.
10. The context is not a general privilege escalation interface; it is the host implementation for the exact authorized request.

## Runtime integration points

- `ApiRouter::call_with_context`: selects provider exactly as normal routing and passes the borrowed context to that provider.
- Runtime host-call API: add a context-aware variant, retaining current context-free behavior.
- Script event dispatch: add a context-aware dispatch entrypoint for events whose host can provide operation contexts. The current deferred script request flow resolves providers during event dispatch, so this scoped context must be supplied there.
- Context-free providers/tests continue to work. Tests must cover context propagation, absence of context, operation binding, errors, no context reuse, and provider selection without Horizon dependencies.

## First vertical slice

Implement a fake scoped context that performs a deterministic in-memory block lookup for `world.block_get`. A context-aware fake provider must call the context and return its result. Verify that the same API request cannot use a context bound to a different operation. This is a contract test only; it does not connect real world/chunk storage.

## Out of scope for this slice

- A concrete Horizon adapter or Horizon SDK dependency.
- Real world/chunk/entity storage.
- Async provider execution.
- Stateful-provider migration.
- Any claim of runtime CI verification until Actions reports successful checks.
