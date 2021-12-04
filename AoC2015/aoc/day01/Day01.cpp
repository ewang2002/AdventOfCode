#pragma once
#include "Day01.h"
#include <string>

std::string Day01::part1() {
    auto line = _input[0];
    auto floor = 0;
    for (char paren : line) {
        floor += paren == '(' ? 1 : -1;
    }

    return std::to_string(floor);
}

std::string Day01::part2() {
    auto floor = 0;
    for (auto i = 0; i < _input[0].length(); i++) {
        floor += _input[0][i] == '(' ? 1 : -1;
        if (floor == -1)
            return std::to_string(i + 1);
    }

    return std::to_string(-1);
}

Day01::Day01(std::string file) : BaseDay(file) {}