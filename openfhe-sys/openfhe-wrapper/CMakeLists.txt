cmake_minimum_required(VERSION 3.5.1)

project(openfhe-wrapper C CXX)

set(CMAKE_CXX_STANDARD 17)
set(CXX_STANDARD_REQUIRED ON)


add_library(openfhe-wrapper STATIC
    src/wrapper.cpp
    src/wrapper.hpp
)