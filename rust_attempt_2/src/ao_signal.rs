/* TODO
 * Find a way to extend this enum so the user can create custom signals that extend this enum
 * That way this enum doesnt need to contain all the user signals. We would just extend and refernce.
 */
#[derive(Debug, Copy, Clone)]
pub enum AoSignal {
    AoProbeSig,         /* Used to shutdown an active object. */
    AoEnterSig,         /* The enter event for states. */
    AoExitSig,          /* The exit event for stats. */
    AoBeginUserSignals, /* Dummy value, not used. Only here to seperate user signals. */
    AoTestSig           /* User signal for testing. */
}
