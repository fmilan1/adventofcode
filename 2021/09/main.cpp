#include <cstdio>
#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <cstring>
#include "../utils.h"

int N = 102;
int M = 102;
int arr[102][102];

bool canBe(int i, int j)
{
    int currnumber = (int)(arr[i][j]);
    
    if (currnumber >= (int)(arr[i][j + 1])) return false;

    if (currnumber >= (int)(arr[i][j - 1])) return false;

    if (currnumber >= (int)(arr[i + 1][j])) return false;

    if (currnumber >= (int)(arr[i - 1][j])) return false;

    return true;

}

int A()
{
    std::fstream read("../i.txt");
    std::string line;
    std::vector<std::string> m;
    std::vector<int> lowpoints;
    while (getline(read, line))
    {
        line.pop_back();
        m.push_back(line);
    }
    int N = m.size() + 2;
    int M = m[0].size() + 2;
    for (int i = 0; i < N; ++i)
    {
        for (int j = 0; j < M; ++j)
        {
            arr[i][j] = 9;
        }
    }

    
    
    for (int i = 0; i < m.size(); ++i)
    {
        for (int j = 0; j < m[0].size(); ++j)
        {
            arr[i + 1][j + 1] = m[i][j]-'0';
        }
    }
    int sum = 0;
    for (int i = 0; i < N; ++i)
    {
        for (int j = 0; j < M; ++j)
        {
            if (canBe(i, j))
            {
                // std::cout << i << " " << j << " " << arr[i][j] << std::endl;
                sum += arr[i][j] + 1;
            }
        }
    }
    
    for (int i = 0; i < N; ++i)
    {
        for (int j = 0; j < M; ++j)
        {
            std::cout << arr[i][j] << " ";
            
        }
        std::cout << "\n";
    }

    
    
    return sum;
}



std::vector<std::vector<bool>> usedIdx(N,std::vector<bool>(M, false));
int size = 0;

int foo(int i, int j)
{
    if (arr[i][j] == 9) return 0;
    // size++;
    arr[i][j] = 9;
    return 1 + foo(i - 1, j) + foo(i + 1, j) + foo(i, j + 1) + foo(i, j - 1);
        
}

int getBasinSize(int i, int j)
{
    if (arr[i][j] != 9)
    {
        // if (j > 0 && j < M - 2 && i > 0 && i < N - 2)
        // {
        if (!usedIdx[i][j])
        {
            size++;
            if (arr[i + 1][j] != 9)
            {
                usedIdx[i][j] = true;
                std::cout << i << " " << j << " --> " << size << "\n";
                return getBasinSize(i + 1, j);
            }
            // else if (arr[i][j + 1] != 9)
            // {
            //     usedIdx[i][j] = true;
            //     std::cout << i << " " << j << " --> " << size << "\n";
            //     return getBasinSize(i, j + 1);
            // }
            else if (arr[i - 1][j] != 9)
            {
                usedIdx[i][j] = true;
                // size--;
                std::cout << i << " " << j << " --> " << size << "\n";
                return getBasinSize(i - 1, j);
            }
                std::cout << i << " " << j << " --> " << size << "\n";
            // else return size;
        }
        else
        {
            return getBasinSize(i, j + 1);
        }
    }
    return size;
}

int B()
{
    size = 0;
    // return getBasinSize(1, 6);
    std::vector<int> results;
    for (int i = 2; i < N - 1; ++i)
    {
        for (int j = 1; j < M - 1; ++j)
        {
            int tmp = foo(i, j);
            // std::cout << tmp << " " << i << " " << j << std::endl;
            results.push_back(tmp);
        }
    }
    sort(results.begin(), results.end(), std::greater<int>());
    return results[0] * results[1] * results[2];
    
}

int main()
{
    std::cout << A() << "\n";
    std::cout << B() << "\n";
    
    std::string a;
    std::cin >> a;
}
