#pragma once

#include <iostream>
#include <fstream>
#include "helper.h"

std::vector<std::string> parseInputFile(const std::string file) {
    std::vector<std::string> input;
    std::ifstream filestream(file.c_str());


    if (!filestream) {
        std::cerr << "Could not open file: " << file << std::endl;
        return input;
    }

    std::string str;
    while (std::getline(filestream, str)) {
        if (str.size() > 0)
            input.emplace_back(str);
    }

    filestream.close();
    return input;
}

// Taken from: https://stackoverflow.com/a/57346888
std::vector<std::string> split(const std::string &i_str, const std::string &i_delim) {
    std::vector<std::string> result;

    size_t found = i_str.find(i_delim);
    size_t startIndex = 0;

    while (found != std::string::npos) {
        result.emplace_back(i_str.begin() + startIndex, i_str.begin() + found);
        startIndex = found + i_delim.size();
        found = i_str.find(i_delim, startIndex);
    }
    if (startIndex != i_str.size())
        result.emplace_back(i_str.begin() + startIndex, i_str.end());
    return result;
}

int parseInt(std::string input) {
    // trim the string
    auto startIdx = 0;
    for (auto i = 0; i < input.size(); i++) {
        if (std::isspace(input[i]))
            continue;
        startIdx = i;
        break;
    }

    unsigned int endIdx = 0;
    for (auto i = input.size() - 1; i >= 0; i--) {
        if (std::isspace(input[i]))
            continue;
        endIdx = i;
        break;
    }

    input = input.substr(startIdx, endIdx + 1 - startIdx);
    if (input.empty())
        throw std::exception("Invalid number: the number is empty.");

    auto startSearchAt = (input[0] == '+' || input[0] == '-') ? 1 : 0;
    for (auto i = startSearchAt; i < input.size(); i++) {
        if (std::isdigit(input[i]))
            continue;

        throw std::exception("Invalid number given");
    }

    auto num = std::stoi(input.substr(startSearchAt, input.size() - startSearchAt));
    return num * (input[0] == '-' ? -1 : 1);
}

bool isInteger(std::string input) {
    // trim the string
    auto startIdx = 0;
    for (auto i = 0; i < input.size(); i++) {
        if (std::isspace(input[i]))
            continue;
        startIdx = i;
        break;
    }

    auto endIdx = 0;
    for (auto i = input.size() - 1; i >= 0; i--) {
        if (std::isspace(input[i]))
            continue;
        endIdx = i;
        break;
    }

    input = input.substr(startIdx, endIdx + 1 - startIdx);
    if (input.empty())
        return false;

    auto startSearchAt = (input[0] == '+' || input[0] == '-') ? 1 : 0;
    for (auto i = startSearchAt; i < input.size(); i++) {
        if (std::isdigit(input[i]))
            continue;

        return false;
    }

    return true;
}