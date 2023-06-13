#ifndef _ACTIVE_OBJECT_H_
#define _ACTIVE_OBJECT_H_

#include <atomic>
#include <thread>
#include <mutex>
#include <condition_variable>
#include <deque>
#include <stdint.h>

#include "StateMachine.h"
#include "ActiveObjectSignals.h"
#include "ActiveObjectEvent.h"

typedef enum class processOption {
    NORMAL,
    UNIT_TEST
} processOption_t;
/*
typedef enum class threadPriority {
    IDLE,
    LOWEST,
    LOW,
    NORMAL,
    HIGH,
    HIGHEST,
    TIME_CRITICAL
} threadPriority_t;
*/
class ActiveObject : public StateMachine {
    public:
        using lockGuard = std::unique_lock<std::mutex>;

        ActiveObject(uint32_t queueDepth);
        virtual ~ActiveObject();
        ActiveObject(const ActiveObject&) = delete;
        ActiveObject(ActiveObject&&) = delete;
        ActiveObject& operator=(ActiveObject&&) = delete;
        ActiveObject& operator=(const ActiveObject&) = delete;

        bool start(processOption_t processOption = processOption_t::NORMAL);
        bool post(const AOEvent_t &event);
        bool postUrgent(const AOEvent_t &event);
        bool publish(const AOEvent_t &event);
        bool publishUrgent(const AOEvent_t &event);
        //bool setPriority(threadPriority_t prio);
        void stop(void);
        bool processOneEvent(processOption_t processOption = processOption_t::NORMAL);

    protected:
        void exitTask(void);
        void task(void);

    private:
        std::atomic_bool mExit;
        std::unique_ptr<std::thread> mThread;
        std::condition_variable mCondVar;
        std::mutex mMutex;
        std::deque<AOEvent_t> mQueue;
        uint32_t mQueueDepth;
};

#endif // _ACTIVE_OBJECT_H_