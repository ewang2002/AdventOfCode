#pragma once
#include <map>
#include "Day05.h"

std::string Day05::part1() {
    auto niceStr = 0;
    for (auto line : _input) {
        // Find three vowels
        auto numVowels = 0;
        for (auto c : line) {
            if (c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u')
                numVowels++;
        }

        if (numVowels < 3)
            continue;

        // appears twice in row
        auto appearsTwice = false;
        for (auto i = 1; i < line.size(); i++) {
            if (line[i] == line[i - 1]) {
                appearsTwice = true;
                break;
            }
        }

        if (!appearsTwice)
            continue;

        // std::string::npos = -1 I think
        auto invalidStrings = {"ab", "cd", "pq", "xy"};
        auto containsInvalidStr = false;
        for (auto s : invalidStrings) {
            if (line.find(s) != std::string::npos) {
                containsInvalidStr = true;
                break;
            }
        }

        if (containsInvalidStr)
            continue;

        niceStr++;
    }

    return std::to_string(niceStr);
}

// Not working :(
// 69, 41, 57, 48
std::string Day05::part2() {
    auto niceStr = 0;
    for (auto line : _input) {
        // Check pairs
        std::vector<std::string> pairs;
        auto hasPair = false;
        auto sizeOfStr = line.size();
        for (auto i = 0; i < sizeOfStr - 3; i++) {
            auto currChar = line[i];
            auto nextChar = line[i + 1];

            for (auto j = i + 2; j < sizeOfStr - 1; j++) {
                auto secondCurr = line[j];
                auto secondNext = line[j + 1];
                if (currChar == secondCurr && nextChar == secondNext) {
                    hasPair = true;
                    goto outLoop;
                }
            }
        }

        outLoop:
        if (!hasPair)
            continue;

        // Check repeats
        auto hasRepeat = false;
        for (auto i = 2; i < line.size(); i++) {
            if (line[i - 2] == line[i]) {
                hasRepeat = true;
                break;
            }
        }

        if (!hasRepeat)
            continue;

        niceStr++;
    }

    return std::to_string(niceStr);
}

Day05::Day05(std::string file) : BaseDay(file) {}
