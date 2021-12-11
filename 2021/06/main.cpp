#include <iostream>
#include <fstream>
#include "../utils.h"

int A(int asd)
{
    std::string a;
    std::getline(std::fstream("../i.txt"), a);
    std::vector<std::string> fishes = string_split(a, ",");
    int day = 0;
    while (++day <= asd)
    {
        int c = 0;
        for (int i = 0; i < fishes.size(); ++i)
        {
            if (fishes[i] != "0")
            {
                fishes[i] = std::to_string(std::stoi(fishes[i]) - 1);
            }
            else
            {
                c++;
                fishes[i] = "6";
            }
        }
        for (int i = 0; i < c; ++i)
        {
            fishes.push_back("8");
        }
    }
    return fishes.size();
}

unsigned long long B(int asd)
{
    std::string a;
    std::getline(std::fstream("../i.txt"), a);
    std::vector<std::string> tmp = string_split(a, ",");
    std::vector<unsigned long long> fish(9);
    for (int i = 0; i < tmp.size(); ++i)
    {
        fish[std::stoull(tmp[i])]++;
    }

    int day = 0;
    while (++day <= asd)
    {
        unsigned long long n = fish[0];
        for (int j = 0; j < fish.size() - 1; j++)
        {
            fish[j] = fish[j + 1];
        }
        fish[8] = n;
        fish[6] += n;

        // for (int i = 0; i < fish.size(); ++i)
        // {
        //     std::cout << fish[i] << " ";
        // }
        // std::cout << std::endl;
    }
    unsigned long long c = 0;
    for (int i = 0; i < fish.size(); i++)
    {
        c += fish[i];
    }
    return c;
}

int main()
{
    int asd = 256;
    // std::cout << A(asd) << std::endl;
    std::cout << B(asd) << std::endl;
    return 0;
}