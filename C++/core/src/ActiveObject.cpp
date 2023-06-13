#include <ActiveObject.h>

#include "ActiveObjectScheduler.h"

#include <cassert>


ActiveObject::ActiveObject(uint32_t queueDepth) :
    StateMachine(),
    mExit(false),
    mThread(nullptr),
    mQueueDepth(queueDepth) {

}

ActiveObject::~ActiveObject(void) {
    assert(mThread == nullptr);
}

bool ActiveObject::start(processOption_t processOption) {
    if (processOption_t::NORMAL == processOption) {
         if (nullptr == this->mThread) {
            this->mThread = std::make_unique<std::thread>([=]() { 
                this->task(); 
            });
         }
    } else {
        this->initialize();
    }

    return true;
}

bool ActiveObject::post(const AOEvent_t &event) {
    lockGuard lockQueue(this->mMutex);

    size_t queueSize = this->mQueue.size();
    if (queueSize < this->mQueueDepth) {
        mQueue.push_back(event);
        lockQueue.unlock();

        if (0 == queueSize) {
            this->mCondVar.notify_one();
        }
        return true;
    }
    return false;
}

bool ActiveObject::postUrgent(const AOEvent_t &event) {
    lockGuard lockQueue(this->mMutex);

    size_t queueSize = this->mQueue.size();
    if (queueSize < this->mQueueDepth) {
        mQueue.push_front(event);
        lockQueue.unlock();

        if (0 == queueSize) {
            this->mCondVar.notify_one();
        }
        return true;
    }
    return false;
}

bool ActiveObject::publish(const AOEvent_t &event) {
    (void)event;
    ActiveObjectScheduler *scheduler = ActiveObjectScheduler::scheduler();
    scheduler->publish(event);
    return true;
}

bool ActiveObject::publishUrgent(const AOEvent_t &event) {
    (void)event;
    ActiveObjectScheduler *scheduler = ActiveObjectScheduler::scheduler();
    scheduler->publishUrgent(event);
    return true;
}

//bool ActiveObject::setPriority(threadPriority_t prio) {
//    (void)prio;
//}

void ActiveObject::stop(void) {
    if (this->mThread) {
        this->exitTask();
        this->postUrgent(AOEvent_t(AOSignal_t::AO_PROBE_SIG));
        this->mThread->join();
        this->mThread.reset(nullptr);
    }
}

bool ActiveObject::processOneEvent(processOption_t processOption) {
    lockGuard lockQueue(this->mMutex);

    AOEvent_t event;
    if (processOption_t::NORMAL == processOption) {
        while (this->mQueue.empty()) {
            this->mCondVar.wait(lockQueue);
        }
    } else {
        if (0 == this->mQueue.size()) {
            return false;
        }
    }

    event = this->mQueue.front();
    this->mQueue.pop_front();

    lockQueue.unlock();
    StateMachine::processEvent(&event);
    return true;
}


void ActiveObject::exitTask(void) {
    this->mExit = true;
}

void ActiveObject::task(void) {
    StateMachine::initialize();

    while (!this->mExit) {
        this->processOneEvent(processOption_t::NORMAL);
    }
}