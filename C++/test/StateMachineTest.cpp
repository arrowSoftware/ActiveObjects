#include "CppUTestExt/MockSupport.h"

#include "StateMachine.h"

class TestStateMachine : public StateMachine
{
    public:
        TestStateMachine() = default;
        virtual ~TestStateMachine() = default;

    private:
        stateRtn initialPsuedoState(const AOEvent_t *event)  override {
            (void)event;
            mock("TestStateMachine").actualCall("initialPsuedoState").onObject(this);
            return transitionTo(&TestStateMachine::testStateA);
        }

        stateRtn testStateA(const AOEvent_t* const event) {
            stateRtn ret;

            switch (event->sig)
            {
                case AOSignal_t::AO_ENTER_SIG: {
                    mock("TestStateMachine").actualCall("A-ENTER").onObject(this);
                    ret = handled();
                    break;
                }
                case AOSignal_t::AO_EXIT_SIG: {
                    mock("TestStateMachine").actualCall("A-EXIT").onObject(this);
                    ret = handled();
                    break;
                }
                case AOSignal_t::AO_TEST_1_SIG: {
                    mock("TestStateMachine").actualCall("A-T1-HANDLED").onObject(this);
                    ret = handled();
                    break;
                }
                case AOSignal_t::AO_TEST_2_SIG: {
                    ret = transitionTo(&TestStateMachine::testStateB);
                    break;
                }
                case AOSignal_t::AO_TEST_3_SIG: {
                    ret = transitionTo(&TestStateMachine::testStateC);
                    break;
                }
                default: {
                    mock("TestStateMachine").actualCall("A-Default-HANDLED").onObject(this);
                    ret = handled();
                    break;
                }
            }
            return ret;
        }

        stateRtn testStateB(const AOEvent_t* const event) {
            stateRtn ret;

            switch (event->sig)
            {
                case AOSignal_t::AO_ENTER_SIG: {
                    mock("TestStateMachine").actualCall("B-ENTER").onObject(this);
                    ret = handled();
                    break;
                }
                case AOSignal_t::AO_EXIT_SIG: {
                    mock("TestStateMachine").actualCall("B-EXIT").onObject(this);
                    ret = handled();
                    break;
                }
                case AOSignal_t::AO_TEST_1_SIG: {
                    ret = transitionTo(&TestStateMachine::testStateA);
                    break;
                }
                case AOSignal_t::AO_TEST_2_SIG: {
                    mock("TestStateMachine").actualCall("B-T2-HANDLED").onObject(this);
                    ret = handled();
                    break;
                }
                case AOSignal_t::AO_TEST_3_SIG: {
                    ret = transitionTo(&TestStateMachine::testStateC);
                    break;
                }
                default: {
                    mock("TestStateMachine").actualCall("B-Default-HANDLED").onObject(this);
                    ret = handled();
                    break;
                }
            }
            return ret;
        }

        stateRtn testStateC(const AOEvent_t* const event) {
            stateRtn ret;

            switch (event->sig)
            {
                case AOSignal_t::AO_ENTER_SIG: {
                    mock("TestStateMachine").actualCall("C-ENTER").onObject(this);
                    ret = handled();
                    break;
                }
                case AOSignal_t::AO_EXIT_SIG: {
                    mock("TestStateMachine").actualCall("C-EXIT").onObject(this);
                    ret = handled();
                    break;
                }
                case AOSignal_t::AO_TEST_1_SIG: {
                    ret = transitionTo(&TestStateMachine::testStateA);
                    break;
                }
                case AOSignal_t::AO_TEST_2_SIG: {
                    ret = transitionTo(&TestStateMachine::testStateB);
                    break;
                }
                case AOSignal_t::AO_TEST_3_SIG: {
                    mock("TestStateMachine").actualCall("C-T2-HANDLED").onObject(this);
                    ret = handled();                    break;
                }
                default: {
                    mock("TestStateMachine").actualCall("C-Default-HANDLED").onObject(this);
                    ret = handled();
                    break;
                }
            }
            return ret;
        }
};