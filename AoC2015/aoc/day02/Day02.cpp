#pragma once

#include "Day02.h"
#include <cassert>

std::string Day02::part1() {
    auto totalArea = 0;
    for (auto & i : _input) {
        auto dimensions = split(i, "x");
        std::vector<int> nums;
        for (const auto& dimension : dimensions)
            nums.emplace_back(stoi(dimension));

        assert(dimensions.size() == 3);
        // [0] = l
        // [1] = w
        // [2] = h
        totalArea += (2 * nums[0] * nums[1])
                + (2 * nums[1] * nums[2])
                + (2 * nums[0] * nums[2])
                + std::min(nums[0] * nums[1], std::min(nums[1] * nums[2], nums[2] * nums[0]));
    }

    return std::to_string(totalArea);
}


std::string Day02::part2() {
    auto totalFeetRibbon = 0;
    for (auto dimensions : _input) {
        std::vector<int> nums;
        auto dim = split(dimensions, "x");
        for (auto d : dim)
            nums.emplace_back(stoi(d));

        auto p1 = nums[0] + nums[2] + nums[2] + nums[0];
        auto p2 = nums[0] + nums[0] + nums[1] + nums[1];
        auto p3 = nums[1] + nums[2] + nums[2] + nums[1];

        totalFeetRibbon += std::min(p1, std::min(p2, p3)) + nums[0] * nums[1] * nums[2];
    }

    return std::to_string(totalFeetRibbon);
}

Day02::Day02(std::string file) : BaseDay(file) {}