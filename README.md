# State Machine

A generic **finite state machine (FSM)** library in Rust — define states, events, and transitions in a declarative table, then process events with compile-time type safety and runtime validation.

## Why It Matters

State machines are the most reliable way to model any system with discrete modes: traffic lights, TCP connections, vending machines, UI flows, parser states, and protocol handlers. By making states and transitions **explicit**, you eliminate impossible states (a connection that's simultaneously "connecting" and "closed") and undefined behavior ("what happens if I receive data while idle?"). The type system enforces valid transitions at compile time (states and events are generic type parameters), and invalid transitions return errors at runtime. This is the pattern behind Erlang's `gen_statem`, Rust's `statig` crate, and XState (JavaScript).

## How It Works

### Transition Table

The FSM stores a `HashMap<(State, Event), State>`. Each entry is one rule: "when in state S and event E fires, transition to state S'."

```
transitions = {
    (Idle,    Start)  → Running,
    (Running, Pause)  → Paused,
    (Paused,  Resume) → Running,
    (Running, Stop)   → Idle,
    (Paused,  Stop)   → Idle,
}
```

Any `(state, event)` pair not in the table is an error.

### Processing Events

```
process(event):
    next = transitions.get((current, event))
    if next is None:
        return Err(InvalidTransition { from: current, event })
    current = next
    return Ok(next)
```

### Type Safety

The generic parameters `S: Clone + Hash + Eq + Debug` and `E: Clone + Hash + Eq + Debug` mean you can use any type for states and events — enums, strings, integers. This enables exhaustive pattern matching in match expressions.

### Complexity

| Operation | Time |
|-----------|------|
| Add transition | O(1) |
| Process event | O(1) |
| Check current state | O(1) |
| Space | O(t) where t = number of transitions |

### Compared to Alternatives

- **State pattern (GoF)**: OOP approach with polymorphic state objects. More flexible but more boilerplate.
- **Match on enum**: Rust idiom — `match` on (state, event) pairs. Simple but doesn't scale to many states.
- **Typestate pattern**: Compile-time FSM using Rust's type system. Zero runtime cost but inflexible.
- This crate: Runtime FSM with dynamic transitions — flexible and simple.

## Quick Start

```rust
use state_machine::StateMachine;

fn main() {
    let mut sm: StateMachine<&str, &str> = StateMachine::new("idle");
    sm.add_transition("idle", "start", "running")
      .add_transition("running", "pause", "paused")
      .add_transition("paused", "resume", "running")
      .add_transition("running", "stop", "idle")
      .add_transition("paused", "stop", "idle");

    assert_eq!(sm.current_state(), &"idle");

    sm.process("start").unwrap();
    assert_eq!(sm.current_state(), &"running");

    sm.process("pause").unwrap();
    assert_eq!(sm.current_state(), &"paused");

    sm.process("resume").unwrap();
    assert_eq!(sm.current_state(), &"running");

    sm.process("stop").unwrap();
    assert_eq!(sm.current_state(), &"idle");
}
```

## API

### `StateMachine<S, E>`
- `new(initial: S)` — create FSM with starting state
- `add_transition(from, event, to) -> &mut Self` — builder pattern
- `process(event) -> Result<S, TransitionError<S, E>>` — fire an event
- `current_state() -> &S` — inspect current state

### `TransitionError`
- `InvalidTransition { from, event }` — no rule for this pair
- `NoRuleDefined` — generic error

## Architecture Notes

In SuperInstance, state machines govern fleet ship lifecycles: `Idle → Booting → Healthy → Degraded → Recovering → Healthy`. Transitions are triggered by conservation law violations: a ship exceeding its γ + η = C budget transitions from `Healthy` to `Degraded`. The Cocapn watches these transitions across the fleet. See [Architecture](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## References

- Wagner, F. & Wolstenholme, P. (2004). *Misunderstandings about State Machines*. embedded.com
- Samek, M. (2002). *Practical Statecharts in C/C++*. CMP Books. (Hierarchical state machines.)
- Russell, S. & Norvig, P. (2020). *AI: A Modern Approach*, 4th ed., §3.3 (search in state spaces).

## License

MIT
