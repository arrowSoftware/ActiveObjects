#ifndef _ACTIVE_OBJECT_EVENT_H_
#define _ACTIVE_OBJECT_EVENT_H_

#include "ActiveObjectSignals.h"

typedef struct AOEvent {
    constexpr explicit AOEvent(void) : sig(AOSignal_t::AO_ENTER_SIG) {}
    constexpr explicit AOEvent(AOSignal_t signal) : sig(signal) {}
    AOSignal_t sig;
} AOEvent_t;

#endif // _ACTIVE_OBJECT_EVENT_H_