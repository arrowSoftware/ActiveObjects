
cmake_minimum_required(VERSION 3.16)
project(ActiveObject)

set(CMAKE_CXX_STANDARD 17)

add_compile_options(-Wall -Wextra -Werror)

set(APP_NAME ActiveObjectApp)
set(APP_LIB_NAME ActiveObjectAppLib)

# (2) Include application build instructions
add_subdirectory(src)

# (3) include tests build instructions   
option(COMPILE_TESTS "Compile the tests" OFF)
if(COMPILE_TESTS)
  add_subdirectory(tests)
endif(COMPILE_TESTS)
