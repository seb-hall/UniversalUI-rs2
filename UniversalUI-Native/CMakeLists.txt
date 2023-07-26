# CMakeLists.txt

cmake_minimum_required(VERSION 3.7)
project(UniversalUI-Native)

enable_testing()

add_subdirectory(UniversalUI-Native-rs)
add_subdirectory(src)
