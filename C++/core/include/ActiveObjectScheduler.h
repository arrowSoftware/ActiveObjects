#ifndef _ACTIVE_OBJECT_SCHEDULER_H_
#define _ACTIVE_OBJECT_SCHEDULER_H_

#include <vector>
#include <deque>
#include <mutex>
#include <condition_variable>
#include <map>

#include "ActiveObject.h"

class ActiveObjectScheduler {
    public:
        using lockGuard = std::unique_lock<std::mutex>;

        ActiveObjectScheduler(ActiveObjectScheduler &other) = delete;
        void operator=(const ActiveObjectScheduler &) = delete;

        static ActiveObjectScheduler* scheduler();

        int run(processOption_t processOption = processOption_t::NORMAL);

        void registerActiveObject(ActiveObject *ao);
        void subscribe(ActiveObject *ao, AOSignal_t signal);
        bool publish(const AOEvent_t &event);
        bool publishUrgent(const AOEvent_t &event);

    protected:
        ActiveObjectScheduler(void);
        bool processOneEvent(processOption_t processOption = processOption_t::NORMAL);

        static ActiveObjectScheduler *mInstance;

    private:
        std::vector<ActiveObject*> mRegisteredActiveObjects;
        std::deque<AOEvent_t> mQueue;
        std::condition_variable mCondVar;
        std::mutex mMutex;
        uint32_t mQueueDepth;
        typedef std::vector<ActiveObject*> activeObjectList_t;
        typedef std::map<AOSignal_t, activeObjectList_t> activeObjectSignalMap_t;
        activeObjectSignalMap_t mSubscriberMap;
};

#endif // _ACTIVE_OBJECT_SCHEDULER_H_