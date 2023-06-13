#include "StateMachine.h"

#include <cassert>

StateMachine::StateMachine(void) :
    mCurrentState(nullptr),
    mPreviousState(nullptr) {}

StateMachine::~StateMachine(void) {}

void StateMachine::initialize(void) {
    assert(nullptr == this->mCurrentState);

    // Set the state to the initial psuedo state.
    this->mCurrentState = &StateMachine::initialPsuedoState;
    // reference: Built-in pointer-to-member access operators
    // execute the enter event on the psuedo state.
    StateMachine::stateRtn ret = ((this)->*(mCurrentState))(&enterEvent);

    // Enter the real initial state with the enter event.
    this->mCurrentState = ret.mMethod;
    ((this)->*(mCurrentState))(&enterEvent);
}

void StateMachine::processEvent(const AOEvent *event) {
    StateMachine::stateRtn ret = ((this)->*(mCurrentState))(event);
    if (ret.mMethod != this->mCurrentState) {
        // execute the exit event on the current state.
        ((this)->*( this->mCurrentState))(&this->exitEvent);

        // Enter the new state with the enter event.
         this->mCurrentState = ret.mMethod;
        ((this)->*( this->mCurrentState))(&this->enterEvent);
    }
}

StateMachine::stateRtn StateMachine::handled(void) {
    return stateRtn(this->mCurrentState);
}