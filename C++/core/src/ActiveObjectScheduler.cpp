#include "ActiveObjectScheduler.h"

ActiveObjectScheduler* ActiveObjectScheduler::mInstance = nullptr;

ActiveObjectScheduler *ActiveObjectScheduler::scheduler() {
    if (nullptr == mInstance) {
        mInstance = new ActiveObjectScheduler();
    }

    return mInstance;
}

ActiveObjectScheduler::ActiveObjectScheduler() : mQueueDepth(100) {

}

void ActiveObjectScheduler::registerActiveObject(ActiveObject *ao) {
    mRegisteredActiveObjects.push_back(ao);
}

void ActiveObjectScheduler::subscribe(ActiveObject *ao, AOSignal_t signal) {
    if (mSubscriberMap.find(signal) == mSubscriberMap.end()) {
        // signal not in map yet, add it
        mSubscriberMap.insert(activeObjectSignalMap_t::value_type(signal, activeObjectList_t()));
        mSubscriberMap[signal].push_back(ao);
    } else {
        mSubscriberMap[signal].push_back(ao);
    }
}

bool ActiveObjectScheduler::processOneEvent(processOption_t processOption) {
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

    for (unsigned int i = 0; i < this->mSubscriberMap[event.sig].size(); i++) {
        this->mSubscriberMap[event.sig].at(i)->processEvent(&event);
    }

    return true;
}

bool ActiveObjectScheduler::publish(const AOEvent_t &event) {
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

bool ActiveObjectScheduler::publishUrgent(const AOEvent_t &event) {
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

int ActiveObjectScheduler::run(processOption_t processOption) {
    while (true) {
        processOneEvent(processOption);
    }

    return 0;
}