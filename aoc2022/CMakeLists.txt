cmake_minimum_required(VERSION 3.20)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
project(aoc2022 VERSION 1.0 DESCRIPTION "aoc2022" LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED YES)

include_directories(include)
link_directories(include)

file(GLOB_RECURSE SOURCES "src/*.cpp")

add_compile_options("-Wall" "-Wextra" "-Wpedantic" "-Werror" "-Wdouble-promotion"
        # "$<$<CONFIG:Debug>:-gdwarf-4>"
        "$<$<CONFIG:Release>:-O3>")
#set(CMAKE_CXX_FLAGS_DEBUG "-g3 -fsanitize=address,undefined,leak -fsanitize-undefined-trap-on-error")
#add_compile_options("$<$<CONFIG:Debug>:-g3>" "-fdiagnostics-format=clang" "-ftemplate-backtrace-limit=1024" "-ftemplate-backtrace-limit=32")

add_executable(aoc2022 ${SOURCES})

if(CMAKE_BUILD_TYPE MATCHES "Debug")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fsanitize=undefined -fsanitize=address")
    target_link_options(aoc2022 BEFORE PUBLIC -fsanitize=undefined PUBLIC -fsanitize=address)
endif()

# Debug build
# cmake -S . -B build/ -D CMAKE_BUILD_TYPE=Debug
# cmake --build build/

# Release build
# cmake -S . -B build/ -D CMAKE_BUILD_TYPE=Release
# cmake --build build/
