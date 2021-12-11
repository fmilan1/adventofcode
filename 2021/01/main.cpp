#include <iostream>
#include <fstream>
#include <conio.h>
#include <string>
#include <vector>


int A()
{
    std::string line;
    std::vector<std::string> input;
    std::ifstream read("../input.txt");

    int c = 0;
    while (std::getline(read, line))
    {
        input.push_back(line);
    }

    for (int i = 1; i < input.size(); i++)
    {
        if (std::stoi(input[i]) > std::stoi(input[i - 1])) { c++; }
    }
    
    return c;
}

int B()
{
    std::string line;
    std::vector<std::string> input;
    std::ifstream read("../input.txt");

    int c = 0;
    while (std::getline(read, line))
    {
        input.push_back(line);
    }

    std::vector<int> trios;
    for (int i = 0; i < input.size() - 2; i++)
    {
         trios.push_back(std::stoi(input[i]) + std::stoi(input[i + 1]) + std::stoi(input[i + 2]));
    }

    for (int i = 1; i < trios.size(); i++)
    {
        if (trios[i] > trios[i - 1]) c++;
    }

    return c;
}

int main()
{
    
    std::cout << A() << "\n";
    std::cout << B() << "\n";
    
    
    
    getch();
}
