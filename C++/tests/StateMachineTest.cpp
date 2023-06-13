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
                    mock("TestStateMachine").actualCall("C-T3-HANDLED").onObject(this);
                    ret = handled();                    
                    break;
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


TEST_GROUP(StateMachineTests) {
    TestStateMachine *mUnderTest;

    void setup() {
        mUnderTest = new TestStateMachine();
    }

    void teardown() {
        delete mUnderTest;
        mock().clear();
    }

    void Init() {
        mUnderTest->initialize();
    }

    void InitToA() {
        mock("TestStateMachine").expectOneCall("initialPsuedoState").onObject(mUnderTest);
        mock("TestStateMachine").expectOneCall("A-ENTER").onObject(mUnderTest);
        Init();
        mock().checkExpectations();
    }

    void InitToB() {
        AOEvent_t event(AOSignal_t::AO_TEST_2_SIG);
        InitToA();
        mock("TestStateMachine").expectOneCall("A-EXIT").onObject(mUnderTest);
        mock("TestStateMachine").expectOneCall("B-ENTER").onObject(mUnderTest);
        mUnderTest->processEvent(&event);
        mock().checkExpectations();
    }

    void InitToC() {
        AOEvent_t event(AOSignal_t::AO_TEST_3_SIG);
        InitToA();
        mock("TestStateMachine").expectOneCall("A-EXIT").onObject(mUnderTest);
        mock("TestStateMachine").expectOneCall("C-ENTER").onObject(mUnderTest);
        mUnderTest->processEvent(&event);
        mock().checkExpectations();
    }
};

TEST(StateMachineTests, create_does_not_crash)
{
    mock("TestStateMachine").ignoreOtherCalls();
    Init();
}

TEST(StateMachineTests, init_behaves_as_expected)
{
    InitToA();
}

TEST(StateMachineTests, a_handled_event)
{
    AOEvent_t event(AOSignal_t::AO_TEST_1_SIG);
    InitToA();
    mock("TestStateMachine").expectOneCall("A-T1-HANDLED").onObject(mUnderTest);
    mUnderTest->processEvent(&event);
    mock().checkExpectations();
}

TEST(StateMachineTests, tran_to_a_to_b_behaves_as_expected)
{
    InitToB();
}

TEST(StateMachineTests, b_handled_event)
{
    AOEvent_t event(AOSignal_t::AO_TEST_2_SIG);

    InitToB();
    mock("TestStateMachine").expectOneCall("B-T2-HANDLED").onObject(mUnderTest);
    mUnderTest->processEvent(&event);
    mock().checkExpectations();
}

TEST(StateMachineTests, tran_to_a_to_c_behaves_as_expected)
{
    InitToC();
}

TEST(StateMachineTests, c_handled_event)
{
    AOEvent_t event(AOSignal_t::AO_TEST_3_SIG);

    InitToC();
    mock("TestStateMachine").expectOneCall("C-T3-HANDLED").onObject(mUnderTest);
    mUnderTest->processEvent(&event);
    mock().checkExpectations();
}