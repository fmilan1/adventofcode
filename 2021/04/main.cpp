#include <iostream>
#include <vector>
#include <string>
#include <fstream>
#include <sstream>
#include <cstring>

std::vector<int> splitByChar(std::string str, char c)
{
    std::vector<int> splits;
    std::string split;
    for (int i = 0; i < str.size(); i++)
    {
        if (str[i] != c /*&& i < str.size()-1*/)
        {
            split += str[i];
        }
        if ((str[i] == c || i == str.size() - 1) && split != "")
        {
            splits.push_back(std::stoi(split));
            split = "";
        }
    }
    return splits;
}

std::vector<int> split(std::string str, std::string splitter)
{
    std::vector<int> splits;
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
                    splits.push_back(std::stoi(split));
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



std::vector<int> readnums()
{
    std::fstream read ("../input.txt");
    std::vector<int> nums;
    std::string line;

    std::getline(read, line);
    nums = split(line, ",");
    
    
    return nums;
}

std::vector<std::vector<int>> readboards()
{
    std::fstream read ("../input.txt");
    std::vector<int> nums;
    std::string line;

    std::vector<std::vector<int>> boards;
    
    std::getline(read, line);
    while (getline(read, line))
    {
        if (line != "\r")
        {
            std::vector<int> tmp = split(line, " ");
            nums.insert(nums.end(), tmp.begin(), tmp.end());
        }
        if (nums.size() == 25)
        {
            boards.push_back(nums);
            nums = {};
        }
    }


    return boards;
}


int A()
{
    std::vector<int> nums = readnums();
    std::vector<std::vector<int>> boards = readboards();

    int c = 0;
    for (int i = 0; i < nums.size(); i++)
    {
        int last;
        for (int j = 0; j < boards.size(); j++)
        {
            if (std::count(boards[j].begin(), boards[j].end(), nums[i]))
            {
                int idx = std::distance(boards[j].begin(), std::find(boards[j].begin(), boards[j].end(), nums[i]));
                last = boards[j][idx];
                boards[j][idx] -= 1000;
            }
        }

        for (int j = 0; j < boards.size(); j++)
        {
            bool bingo = false;
            for (int k = 0; k < 5; k++)
            {
                if (boards[j][k] < 0 && boards[j][k + 5] < 0 && boards[j][k + 10] < 0 && boards[j][k + 15] < 0 && boards[j][k + 20] < 0)
                {
                    bingo = true;
                    goto check;
                }
            }

            for (int k = 0; k < boards[j].size(); k += 5)
            {
                if (boards[j][k] < 0 && boards[j][k + 1] < 0 && boards[j][k + 2] < 0 && boards[j][k + 3] < 0  && boards[j][k + 4] < 0)
                {
                    bingo = true;
                    goto check;
                }
            }
            check:
            if (bingo)
            {
                for (int k = 0; k < boards[j].size(); k++)
                {
                    if (boards[j][k] >= 0) c += boards[j][k];
                }
                c *= last;
                goto bingo;
            }
        }
    }
    bingo:
    return c;
}


int B()
{
    std::vector<int> nums = readnums();
    std::vector<std::vector<int>> boards = readboards();

    int c = 0;
    for (int i = 0; i < nums.size(); i++)
    {
        int lastnum;
        std::vector<int> lastboard;
        for (int j = 0; j < boards.size(); j++)
        {
            if (std::count(boards[j].begin(), boards[j].end(), nums[i]))
            {
                int idx = std::distance(boards[j].begin(), std::find(boards[j].begin(), boards[j].end(), nums[i]));
                lastnum = boards[j][idx];
                boards[j][idx] -= 1000;
            }
        }

        for (int j = 0; j < boards.size(); j++)
        {
            bool bingo = false;
            for (int k = 0; k < 5; k++)
            {
                if (boards[j][k] < 0 && boards[j][k + 5] < 0 && boards[j][k + 10] < 0 && boards[j][k + 15] < 0 && boards[j][k + 20] < 0)
                {
                    lastboard = boards[j];
                    boards[j] = {};
                    goto check;
                }
            }

            for (int k = 0; k < boards[j].size(); k += 5)
            {
                if (boards[j][k] < 0 && boards[j][k + 1] < 0 && boards[j][k + 2] < 0 && boards[j][k + 3] < 0  && boards[j][k + 4] < 0)
                {
                    lastboard = boards[j];
                    boards[j] = {};
                    goto check;
                }
            }
            check:
            int bingos = 0;
            for (int l = 0; l < boards.size(); l++)
            {
                if (boards[l].size() == 0)
                {
                    bingos++;
                }
            }
            if (bingos == boards.size())
            {
                for (int k = 0; k < lastboard.size(); k++)
                {
                    if (lastboard[k] >= 0) c += lastboard[k];
                }
                c *= lastnum;
                goto bingo;
            }
        }
    }
    bingo:
    return c;
}

int main()
{
    std::cout << A() << "\n";
    std::cout << B() << "\n";
    
    return 0;
}
