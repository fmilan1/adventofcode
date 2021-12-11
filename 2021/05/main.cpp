#include <iostream>
#include <vector>
#include <fstream>
#include <cstdlib>
#include "../utils.h"

int A()
{
    std::ifstream read("../i.txt");
    std::string line;
    std::vector<std::string> coords;
    std::vector<std::vector<int>> matrix(1000,std::vector<int>(1000));
    int x1 = 0, x2 = 0, y1 = 0, y2 = 0;
    while (getline(read, line))
    {
        coords = string_split(line, " -> ");
        x1 = std::stoi(string_split(coords[0], ",")[0]);
        y1 = std::stoi(string_split(coords[0], ",")[1]);
        x2 = std::stoi(string_split(coords[1], ",")[0]);
        y2 = std::stoi(string_split(coords[1], ",")[1]);
        if (x1 == x2 || y1 == y2)
        {
            for (int i = std::min(y1, y2); i <= std::max(y1, y2); i++)
            {
                for (int j = std::min(x1, x2); j <= std::max(x1, x2); j++)
                {
                    matrix[i][j]++;
                }
            }
        }

    }
    int c = 0;
    for (int i = 0; i < matrix.size(); i++)
    {
        for (int j = 0; j < matrix[i].size(); j++)
        {
            if (matrix[i][j] >= 2) c++;
        }
    }


    return c;
}

int B()
{
    std::ifstream read("../i.txt");
    std::string line;
    std::vector<std::string> coords;
    std::vector<std::vector<int>> matrix(1000,std::vector<int>(1000));
    int x1 = 0, x2 = 0, y1 = 0, y2 = 0;
    while (getline(read, line))
    {
        coords = string_split(line, " -> ");
        x1 = std::stoi(string_split(coords[0], ",")[0]);
        y1 = std::stoi(string_split(coords[0], ",")[1]);
        x2 = std::stoi(string_split(coords[1], ",")[0]);
        y2 = std::stoi(string_split(coords[1], ",")[1]);
        if (x1 == x2 || y1 == y2)
        {
            for (int i = std::min(y1, y2); i <= std::max(y1, y2); i++)
            {
                for (int j = std::min(x1, x2); j <= std::max(x1, x2); j++)
                {
                    matrix[i][j]++;
                }
            }
        }
        
        if (abs(x2 - x1) == abs(y2 - y1))
        {
            if (x1 < x2)
            {
                if (y1 < y2)
                {
                    for (int i = 0; i <= x2 - x1; ++i)
                    {
                        matrix[y2 - i][x2 - i]++;
                    }
                }
                else if (y1 > y2)
                {
                    for (int i = 0; i <= x2 - x1; ++i)
                    {
                        matrix[y1 - i][x1 + i]++;
                    }
                }
            }
            else if (x1 > x2)
            {
                if (y1 < y2)
                {
                    for (int i = 0; i <= x1 - x2; ++i)
                    {
                        matrix[y2 - i][x2 + i]++;
                    }
                }
                else if (y1 > y2)
                {
                    for (int i = 0; i <= x1 - x2; ++i)
                    {
                        matrix[y2 + i][x2 + i]++;
                    }
                }
                
            }
        }
    }
    int c = 0;
    for (int i = 0; i < matrix.size(); i++)
    {
        for (int j = 0; j < matrix[i].size(); j++)
        {
            if (matrix[i][j] >= 2) c++;
        }
    }


    return c;
}

int main()
{

    std::cout << A() << "\n";
    std::cout << B() << "\n";
    return 0;
}
