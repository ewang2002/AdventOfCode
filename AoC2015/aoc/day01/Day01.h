#pragma once
#ifndef AOC2015_DAY01_H
#define AOC2015_DAY01_H

#include <string>
#include <vector>
#include "../BaseDay.h"

class Day01 : public BaseDay {
public:
    std::string part1() override;
    std::string part2() override;
    explicit Day01(std::string file);
};

#endif //AOC2015_DAY01_H
