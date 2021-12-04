#include "Day08.h"

std::string Day08::part1() {
    size_t totalLength = 0;
    size_t characters = 0;

    for (const auto &line : _input) {
        totalLength += line.size();
        size_t len = 0;
        for (auto i = line.size() - 2; i >= 1; i--) {
            len++;
            while (line[i] == '\\') i--;
        }

        std::cout << line << ": " << line.size() << " - " << len << std::endl;
    }

    return "";
}

std::string Day08::part2() {
    return "";
}

Day08::Day08(std::string file) : BaseDay(file) {}
