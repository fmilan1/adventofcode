#pragma once

#include <vector>
#include <string>


std::vector<std::string> string_split(const std::string& str, const std::string& splitter)
{
    std::vector<std::string> splits;
    std::string split;
    for (int i = 0; i < str.size(); i++)
    {
        split += str[i];
        bool splitting = true;
        for (int j = 0; j < splitter.size(); j++)
        {
            if (str[i + j + 1] == splitter[j] || i == str.size() - 1)
            {
                if (splitting && j == splitter.size() - 1)
                {
                    splits.push_back(split);
                    i += splitter.size();
                    split = "";
                }
            }
            else
            {
                splitting = false;
            }
        }
    }
    return splits;
}
