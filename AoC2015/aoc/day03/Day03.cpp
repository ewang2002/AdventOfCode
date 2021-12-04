#pragma once
#include "Day03.h"
#include <map>
#include <unordered_set>

// & => reference
// Note that arguments in C++ are passed by value, not reference
// In C#/Java, arguments are passed by "references."
void add_or_edit_map(std::map<std::tuple<int, int>, int> &presents, int x, int y) {
    auto houseCoords = std::make_tuple(x, y);
    if (presents.contains(houseCoords))
        presents[houseCoords]++;
    else
        presents[houseCoords] = 1;
}

// Using a map<tuple<...>, int> takes 2x less time than using an unordered set.
std::string Day03::part1() {
    std::map<std::tuple<int, int>, int> presents;

    auto x = 0;
    auto y = 0;
    std::vector<char> characters(_input[0].begin(), _input[0].end());
    for (auto c : characters) {
        add_or_edit_map(presents, x, y);
        switch (c) {
            case '^':
                y++;
                break;
            case 'v':
                y--;
                break;
            case '>':
                x++;
                break;
            case '<':
                x--;
                break;
            default:
                throw "Character " + std::to_string(c) + " Not Recognized.";
        }

        // Account for last house
        add_or_edit_map(presents, x, y);
    }

    auto numWithOne = 0;
    for (const auto& houseCoord : presents)
        numWithOne += houseCoord.second >= 1 ? 1 : 0;

    return std::to_string(presents.size());

}

std::string Day03::part2() {
    std::map<std::tuple<int, int>, int> presents;
    auto x = 0;
    auto y = 0;
    auto roboX = 0;
    auto roboY = 0;
    auto i = 0;

    std::vector<char> characters(_input[0].begin(), _input[0].end());
    for (; i < characters.size(); i++) {
        auto santasTurn = i % 2 == 0;
        add_or_edit_map(presents, santasTurn ? x : roboX, santasTurn ? y : roboY);

        switch (characters[i]) {
            case '^':
                santasTurn ? y++ : roboY++;
                break;
            case 'v':
                santasTurn ? y-- : roboY--;
                break;
            case '>':
                santasTurn ? x++ : roboX++;
                break;
            case '<':
                santasTurn ? x-- : roboX--;
                break;
            default:
                throw "Character " + std::to_string(characters[i]) + " Not Recognized.";
        }

        add_or_edit_map(presents, santasTurn ? x : roboX, santasTurn ? y : roboY);
    }

    auto numWithOne = 0;
    for (const auto& houseCoord : presents)
        numWithOne += houseCoord.second >= 1 ? 1 : 0;

    return std::to_string(numWithOne);
}

Day03::Day03(std::string file) : BaseDay(file) {}
