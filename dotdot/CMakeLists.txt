# CMakeList.txt : CMake project for dotdot, include source and define
# project specific logic here.
#
cmake_minimum_required(VERSION 3.8)

set(CMAKE_CXX_STANDARD 20)

# Add source to this project's executable.
file(GLOB SRC *.h *.cpp *.hpp)

add_executable(dotdot ${SRC})

add_subdirectory(yaml-cpp-0.6.3)
target_include_directories(dotdot PRIVATE yaml-cpp-0.6.3)
target_link_libraries(dotdot yaml-cpp)

#find_package(yaml-cpp CONFIG REQUIRED)


# TODO: Add tests and install targets if needed.
