# CMakeList.txt : CMake project for dotdot, include source and define
# project specific logic here.
#
cmake_minimum_required (VERSION 3.8)

set(CMAKE_CXX_STANDARD 20)

# Add source to this project's executable.
file(GLOB SRC *.h *.cpp *.hpp)

add_executable (dotdot ${SRC})

# TODO: Add tests and install targets if needed.
