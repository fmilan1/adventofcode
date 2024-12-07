#include <iostream>
#include <fstream>
#include "../utils.h"
#include <chrono>
#include <map>
std::map<int, int> dp;

int A()
{
    std::fstream read("../i.txt");
    std::string line;
    getline(read, line);
    std::vector<std::string> tmp = string_split(line, ",");
    std::vector<int> crabs;
    for (int i = 0; i < tmp.size(); ++i)
    {
        crabs.push_back(std::stoi(tmp[i]));
    }
    int a = *std::max_element(crabs.begin(), crabs.end()) + 1;
    std::vector<int> cost(a);
    for (int i = 0; i < crabs.size(); ++i)
    {
        for (int j = 0; j < cost.size(); ++j)
        {
            cost[j] += abs(crabs[i] - j);
        }
    }
    return *std::min_element(cost.begin(), cost.end());
}

int sum1(int b)
{
    return b * (b + 1) / 2;
}

int sum2(int b)
{
    int sum = 0;
    for (int i = 1; i <= b; ++i)
    {
        sum += i;
    }
    return sum;
}

int sum3(int b)
{
    if (dp.find(b) == dp.end())
    {
        int sum = 0;
        for (int i = 1; i <= b; ++i)
        {
            sum += i;
        }
        dp[b] = sum;
        return sum;
    }
    return dp.at(b);
}

int B()
{
    std::fstream read("../i.txt");
    std::string line;
    getline(read, line);
    auto start = std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::system_clock::now().time_since_epoch()).count();
    std::vector<std::string> tmp = string_split(line, ",");
    std::vector<int> crabs;
    for (int i = 0; i < tmp.size(); ++i)
    {
        crabs.push_back(std::stoi(tmp[i]));
    }
    int max = *std::max_element(crabs.begin(), crabs.end());
    int min = 100000000;
    for (int i = 0; i <= max; ++i)
    {
        int sum = 0;
        for (int j = 0; j < crabs.size(); ++j)
        {
            int b = abs(crabs[j] - i);
            sum += sum1(b);
            
        }
        if (sum < min)
        {
            min = sum;
        }
    }
    auto end = std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::system_clock::now().time_since_epoch()).count();
    std::cout << end - start << " ms\n";
    return min;
}



int main()
{
    std::cout << A() << std::endl;
    std::cout << B() << std::endl;
    return 0;
}
