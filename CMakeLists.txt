cmake_minimum_required(VERSION 3.31)
project(mouse C)

set(CMAKE_C_STANDARD 17)

enable_testing()

add_subdirectory(src)
add_subdirectory(test)

configure_file(test/resources/maze maze COPYONLY)