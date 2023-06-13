# (1) Build a library with my application sources
set(APP_LIB_SOURCE
    src/ActiveObject.cpp
    src/ActiveObjectScheduler.cpp
    src/StateMachine.cpp
)

include_directories(include)

add_library(${APP_LIB_NAME} ${APP_LIB_SOURCE})

# (2) Add main(..) to the application library to have something we can run
add_executable(${APP_NAME} main.cpp)
target_link_libraries(${APP_NAME} ${APP_LIB_NAME})
