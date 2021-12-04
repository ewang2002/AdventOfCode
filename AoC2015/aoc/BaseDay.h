#pragma once
#ifndef AOC2015_BASEDAY_H
#define AOC2015_BASEDAY_H

#include <string>
#include <vector>
#include <fstream>
#include <iostream>
#include <chrono>
#include "helper.h"

class BaseDay {
public:
    virtual std::string part1() = 0;
    virtual std::string part2() = 0;
    explicit BaseDay(std::string file);
    void execute();
protected:
    std::vector<std::string> _input;
};

#endif //AOC2015_BASEDAY_H
