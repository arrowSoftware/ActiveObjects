use crate::state::StateT;

/**
 * Return enum for each State.
 */
pub enum Action {
    Handled, /* Does nothing, just keeps the current state the same. */
    TransitionTo(StateT) /* Sets the state machines current state to the new StateT State. */
}
