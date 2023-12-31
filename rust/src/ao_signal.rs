/* TODO
 * Find a way to extend this enum so the user can create custom signals that extend this enum
 * That way this enum doesnt need to contain all the user signals. We would just extend and refernce.
 */
#[derive(Debug, Copy, Clone)]
pub enum AoSignal {
    AoProbeSig,         /* Used to shutdown an active object. */
    AoTimerTickSig,     /* Signal used to trigger the tick function of all timers. */
    AoEnterSig,         /* The enter event for states. */
    AoExitSig,          /* The exit event for states. */
    AoStdTimeoutSig,    /* Standard timer signal. */
    AoBeginUserSignals, /* Dummy value, not used. Only here to seperate user signals. */
    AoTestSig,           /* User signal for testing. */
    AoHungrySig, /* TODO user example signal, shouldnt be defined here */
    AoEatSig,
    AoDoneSig,
}
