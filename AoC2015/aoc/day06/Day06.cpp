#include "Day06.h"
#include <cassert>

struct StartEndCoords {
    int startX, startY;
    int endX, endY;
};

StartEndCoords getStartEndCoords(std::vector<std::string> args) {
    auto startArgs = split(args[1 + (args.size() == 5 ? 1 : 0)], ",");
    auto sX = std::stoi(startArgs[0]);
    auto sY = std::stoi(startArgs[1]);

    auto endArgs = split(args[3 + (args.size() == 5 ? 1 : 0)], ",");
    auto eX = std::stoi(endArgs[0]);
    auto eY = std::stoi(endArgs[1]);

    return {sX, sY, eX, eY};
}

std::string Day06::part1() {
    auto lightGrid = new bool[1000][1000];

    for (auto i = 0; i < 1000; i++)
        for (auto j = 0; j < 1000; j++)
            lightGrid[i][j] = false;

    for (auto line : _input) {
        auto args = split(line, " ");
        auto coords = getStartEndCoords(args);

        // toggle  ...
        if (args.size() == 4) {
            for (auto x = coords.startX; x <= coords.endX; x++) {
                for (auto y = coords.startY; y <= coords.endY; y++) {
                    lightGrid[x][y] = !lightGrid[x][y];
                }
            }

            continue;
        }

        // turn ...
        for (auto x = coords.startX; x <= coords.endX; x++) {
            for (auto y = coords.startY; y <= coords.endY; y++) {
                lightGrid[x][y] = args[1] == "on";
            }
        }
    }

    auto ct = 0;
    for (auto i = 0; i < 1000; i++) {
        for (auto j = 0; j < 1000; j++) {
            ct += lightGrid[i][j] ? 1 : 0;
        }
    }

    delete[] lightGrid;
    return std::to_string(ct);
}

std::string Day06::part2() {
    auto lightGrid = new int[1000][1000];

    for (auto i = 0; i < 1000; i++)
        for (auto j = 0; j < 1000; j++)
            lightGrid[i][j] = 0;

    for (const auto& line : _input) {
        auto args = split(line, " ");
        auto coords = getStartEndCoords(args);


        // toggle  ...
        if (args.size() == 4) {
            for (auto x = coords.startX; x <= coords.endX; x++) {
                for (auto y = coords.startY; y <= coords.endY; y++) {
                    lightGrid[x][y] += 2;
                }
            }

            continue;
        }

        // turn ...
        for (auto x = coords.startX; x <= coords.endX; x++) {
            for (auto y = coords.startY; y <= coords.endY; y++) {
                if (args[1] == "on") {
                    lightGrid[x][y]++;
                    continue;
                }

                if (lightGrid[x][y] - 1 < 0)
                    continue;

                lightGrid[x][y]--;
            }
        }
    }

    auto ct = 0;
    for (auto i = 0; i < 1000; i++) {
        for (auto j = 0; j < 1000; j++) {
            ct += lightGrid[i][j];
        }
    }

    delete[] lightGrid;
    return std::to_string(ct);
}

Day06::Day06(std::string file) : BaseDay(file) {}
