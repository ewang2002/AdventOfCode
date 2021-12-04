#include "Day07.h"
#include <map>
#include <queue>


std::string Day07::part1() {
    std::map<std::string, unsigned short> dict;
    // get initial value
    std::queue<std::string> toRemove;
    for (auto line : _input) {
        auto info = split(line, " -> ");
        auto initInfo = split(info[0], " ");

        if (initInfo.size() == 1 && isInteger(initInfo[0])) {
            dict[info[1]] = std::stoi(initInfo[0]);
            toRemove.emplace(line);
        }
    }

    while (!toRemove.empty()) {
        _input.erase(std::remove(_input.begin(), _input.end(), toRemove.front()));
        toRemove.pop();
    }

    for (const auto& line : _input) {
        auto info = split(line, " -> ");
        auto initInfo = split(info[0], " ");

        if (initInfo.size() == 1 && !isInteger(initInfo[0])) {
            dict[info[1]] = dict[initInfo[0]];
            continue;
        }

        // must be NOT
        if (initInfo.size() == 2) {
            dict[info[1]] = ~dict[initInfo[1]];
            continue;
        }

        if (initInfo.size() == 3) {
            if (initInfo[1] == "AND")
                dict[info[1]] = dict[initInfo[0]] & dict[initInfo[2]];
            else if (initInfo[1] == "OR")
                dict[info[1]] = dict[initInfo[0]] | dict[initInfo[2]];
            else if (initInfo[1] == "LSHIFT")
                dict[info[1]] = dict[initInfo[0]] << std::stoi(initInfo[2]);
            else if (initInfo[1] == "RSHIFT")
                dict[info[1]] = dict[initInfo[0]] >> std::stoi(initInfo[2]);
        }
    }

    return std::to_string(dict["a"]);
}

std::string Day07::part2() {
    return std::string();
}

Day07::Day07(std::string file) : BaseDay(file) {}
