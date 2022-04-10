cmake_minimum_required(VERSION 3.10)

# set the project name
project(LZW)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# add the executable
add_executable(lzw main.c compress.c dictionary.c string.c buffer.c)
