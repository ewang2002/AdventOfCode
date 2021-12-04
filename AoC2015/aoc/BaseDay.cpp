#pragma once
#include "BaseDay.h"

#include <string>
#include <fstream>
#include <iostream>
#include <chrono>

void BaseDay::execute() {
    auto t1 = std::chrono::high_resolution_clock::now();
    auto res1 = part1();
    auto t2 = std::chrono::high_resolution_clock::now();
    auto res2 = part2();
    auto t2end = std::chrono::high_resolution_clock::now();

    auto timePart1 = t2 - t1;
    auto timePart2 = t2end - t2;

    std::cout << "Part 1: " << res1 << std::endl;
    std::cout << "\tTime Taken: " << duration_cast<std::chrono::milliseconds>(timePart1).count() << " MS" << std::endl;
    std::cout << "\n" << std::endl;
    std::cout << "Part 2: " << res2 << std::endl;
    std::cout << "\tTime Taken: " << duration_cast<std::chrono::milliseconds>(timePart2).count() << " MS" << std::endl;
}

BaseDay::BaseDay(std::string file) {
    _input = parseInputFile(file);
}