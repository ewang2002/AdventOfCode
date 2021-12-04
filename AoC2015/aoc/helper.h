#pragma once
#ifndef AOC2015_HELPER_H
#define AOC2015_HELPER_H

#include <vector>
#include <string>

std::vector<std::string> parseInputFile(std::string file);

std::vector<std::string> split(const std::string &i_str, const std::string &i_delim);

int parseInt(std::string input);

bool isInteger(std::string input);

#endif //AOC2015_HELPER_H
