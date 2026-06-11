use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub enum TransitionError<S, E> {
    InvalidTransition { from: S, event: E },
    NoRuleDefined,
}

/// A generic finite state machine.
pub struct StateMachine<S, E>
where
    S: Clone + Hash + Eq + Debug,
    E: Clone + Hash + Eq + Debug,
{
    transitions: HashMap<(S, E), S>,
    current: S,
}

impl<S, E> StateMachine<S, E>
where
    S: Clone + Hash + Eq + Debug,
    E: Clone + Hash + Eq + Debug,
{
    pub fn new(initial: S) -> Self {
        Self {
            transitions: HashMap::new(),
            current: initial,
        }
    }

    pub fn add_transition(&mut self, from: S, event: E, to: S) -> &mut Self {
        self.transitions.insert((from, event), to);
        self
    }

    pub fn current_state(&self) -> &S {
        &self.current
    }

    pub fn process(&mut self, event: E) -> Result<S, TransitionError<S, E>> {
        let next = self
            .transitions
            .get(&(self.current.clone(), event.clone()))
            .cloned()
            .ok_or(TransitionError::InvalidTransition {
                from: self.current.clone(),
                event,
            })?;
        self.current = next.clone();
        Ok(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_transition() {
        let mut sm: StateMachine<&str, &str> = StateMachine::new("idle");
        sm.add_transition("idle", "start", "running")
          .add_transition("running", "stop", "idle");

        assert_eq!(sm.current_state(), &"idle");
        sm.process("start").unwrap();
        assert_eq!(sm.current_state(), &"running");
        sm.process("stop").unwrap();
        assert_eq!(sm.current_state(), &"idle");
    }
}
