#include <ActiveObject.h>
#include "ActiveObjectScheduler.h"

#include <cstdio>

class TestActiveObject : public ActiveObject {
    public:
        TestActiveObject() : ActiveObject(10) { 
            printf("TestActiveObject created\n");
        }
        ~TestActiveObject() {
            printf("TestActiveObject deleted\n");
            this->stop();
        }

    protected:
        stateRtn initialPsuedoState(const AOEvent_t *event) override {
            (void)event;
            printf("TestActiveObject psuedo\n");

            return this->transitionTo(&TestActiveObject::initial);
        }

        stateRtn initial(const AOEvent_t *event) {
            printf("TestActiveObject initial\n");
            stateRtn rtn;
            switch (event->sig) {
                case AOSignal_t::AO_ENTER_SIG: {
                    printf("TestActiveObject initial AO_ENTER_SIG\n");
                    AOEvent_t event(AOSignal_t::AO_TEST_SIG);
                    this->post(event);
                    rtn = handled();
                    break;
                }
                case AOSignal_t::AO_EXIT_SIG: {
                    printf("TestActiveObject initial AO_EXIT_SIG\n");
                    rtn = handled();
                    break;
                }
                case AOSignal_t::AO_TEST_SIG: {
                    printf("TestActiveObject initial AO_TEST_SIG\n");
                    rtn = transitionTo(&TestActiveObject::update);
                    break;
                }
                default: {
                    rtn = handled();
                    break;
                }
            }

            return rtn;
        }

        stateRtn update(const AOEvent_t *event) {
            printf("TestActiveObject update\n");
            stateRtn rtn;
            switch (event->sig) {
                case AOSignal_t::AO_ENTER_SIG: {
                    printf("TestActiveObject update AO_ENTER_SIG\n");
                    AOEvent_t evt(AOSignal_t::AO_TEST2_SIG);
                    this->publish(evt);
                    rtn = handled();
                    break;
                }
                case AOSignal_t::AO_EXIT_SIG: {
                    printf("TestActiveObject update AO_ENTER_SIG\n");
                    rtn = handled();
                    break;
                }
                case AOSignal_t::AO_TEST2_SIG: {
                    printf("should not happen\n");
                    rtn = handled();
                    break;
                }
                default: {
                    rtn = handled();
                    break;
                }
            }

            return rtn;
        }
    private:
};

class TestActiveObject2 : public ActiveObject {
    public:
        TestActiveObject2() : ActiveObject(10) { 
            printf("TestActiveObject2 created\n");
        }
        ~TestActiveObject2() {
            printf("TestActiveObject2 deleted\n");
            this->stop();
        }

    protected:
        stateRtn initialPsuedoState(const AOEvent_t *event) override {
            (void)event;
            printf("TestActiveObject2 psuedo\n");

            return this->transitionTo(&TestActiveObject2::initial);
        }

        stateRtn initial(const AOEvent_t *event) {
            printf("TestActiveObject2 initial\n");
            stateRtn rtn;
            switch (event->sig) {
                case AOSignal_t::AO_ENTER_SIG: {
                    printf("TestActiveObject2 initial AO_ENTER_SIG\n");
                    rtn = handled();
                    break;
                }
                case AOSignal_t::AO_EXIT_SIG: {
                    printf("TestActiveObject2 initial AO_EXIT_SIG\n");
                    rtn = handled();
                    break;
                }
                case AOSignal_t::AO_TEST2_SIG: {
                    printf("TestActiveObject2 initial AO_TEST2_SIG\n");
                    rtn = transitionTo(&TestActiveObject2::update);
                    break;
                }
                default: {
                    rtn = handled();
                    break;
                }
            }

            return rtn;
        }

        stateRtn update(const AOEvent_t *event) {
            printf("TestActiveObject2 update\n");
            stateRtn rtn;
            switch (event->sig) {
                case AOSignal_t::AO_ENTER_SIG: {
                    printf("TestActiveObject2 update AO_ENTER_SIG\n");
                    rtn = handled();
                    break;
                }
                case AOSignal_t::AO_EXIT_SIG: {
                    printf("TestActiveObject2 update AO_ENTER_SIG\n");
                    rtn = handled();
                    break;
                }
                default: {
                    rtn = handled();
                    break;
                }
            }

            return rtn;
        }
    private:
};

int main(int argc, char **argv) {
    (void)argc;
    (void)argv;

    TestActiveObject ao;
    TestActiveObject2 ao2;

    ActiveObjectScheduler *scheduler = ActiveObjectScheduler::scheduler();
    scheduler->registerActiveObject(&ao);
    scheduler->registerActiveObject(&ao2);
    scheduler->subscribe(&ao2, AOSignal_t::AO_TEST2_SIG);

    ao.start();
    ao2.start();

    return scheduler->run();
}