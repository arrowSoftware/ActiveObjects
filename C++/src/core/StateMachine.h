#ifndef _STATE_MACHINE_H_
#define _STATE_MACHINE_H_

#include "ActiveObjectEvent.h"

class StateMachine {
    public:
        StateMachine();
        virtual ~StateMachine();

        virtual void initialize(void);
        virtual void processEvent(const AOEvent_t *event);

    protected:
        struct stateRtn;
        using stateMethodHandler = stateRtn(StateMachine::*)(const AOEvent_t* event);

        struct stateRtn {
            stateRtn() = default;
            explicit stateRtn(stateMethodHandler method): mMethod(method) {}
            stateMethodHandler mMethod;
        };

        virtual stateRtn initialPsuedoState(const AOEvent_t *event) = 0;
        
        template<typename StateMethodT>
        stateRtn transitionTo(StateMethodT targetState) {
            return stateRtn(static_cast<stateMethodHandler>(targetState));
        }

        virtual stateRtn handled(void);

        stateMethodHandler currentState(void) { return mCurrentState; }

    private:
        const AOEvent_t enterEvent = AOEvent_t(AOSignal_t::AO_ENTER_SIG);
        const AOEvent_t exitEvent = AOEvent_t(AOSignal_t::AO_EXIT_SIG);
        stateMethodHandler mCurrentState = nullptr;
        stateMethodHandler mPreviousState = nullptr;
};

#endif // _STATE_MACHINE_H_