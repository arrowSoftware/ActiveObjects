#ifndef _ACTIVE_OBJECT_SIGNALS_H_
#define _ACTIVE_OBJECT_SIGNALS_H_

typedef enum class AOSignal {
    AO_PROBE_SIG,
    AO_ENTER_SIG,
    AO_EXIT_SIG,
    // Unit test signals
    AO_TEST_1_SIG,
    AO_TEST_2_SIG,
    AO_TEST_3_SIG,
    AO_BEGIN_USER_SIGNALS,
    AO_TEST_SIG,
    AO_TEST2_SIG,
    AO_MAX_SIG
} AOSignal_t;

#endif // _ACTIVE_OBJECT_SIGNALS_H_