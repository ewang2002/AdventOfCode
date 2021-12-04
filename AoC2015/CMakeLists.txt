cmake_minimum_required(VERSION 3.17)
project(AoC2015)

set(CMAKE_CXX_STANDARD 20)

add_executable(AoC2015 main.cpp aoc/day01/Day01.cpp aoc/day01/Day01.h aoc/helper.cpp aoc/helper.h aoc/BaseDay.cpp aoc/BaseDay.h aoc/day02/Day02.cpp aoc/day02/Day02.h aoc/day02/Day02.cpp aoc/day02/Day02.h aoc/day02/Day02.cpp aoc/day02/Day02.h aoc/day03/Day03.cpp aoc/day03/Day03.h aoc/day05/Day05.cpp aoc/day05/Day05.h aoc/day06/Day06.cpp aoc/day06/Day06.h aoc/day07/Day07.cpp aoc/day07/Day07.h aoc/day08/Day08.cpp aoc/day08/Day08.h)

add_custom_command(TARGET ${PROJECT_NAME} POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy_directory
        ${CMAKE_SOURCE_DIR}/input $<TARGET_FILE_DIR:${PROJECT_NAME}>/input)