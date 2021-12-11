#include <iostream>
#include <fstream>
#include <vector>
#include <cstring>

using namespace std;

int N;
int M;
int energy[100][100];

int foo(int i, int j)
{
    energy[i][j]++;
    if (energy[i][j] > 9)
    {
        energy[i][j] = -10;
        return 1 + foo(i, j + 1) + foo(i, j - 1) + foo(i + 1, j) + foo(i - 1, j) + foo(i + 1, j + 1) + foo(i + 1, j - 1) + foo(i - 1, j + 1) + foo(i - 1, j - 1);
    }
    return 0;


}

void a()
{
    fstream read("../i.txt");
    string line;
    vector<string> tmp;

    while (getline(read, line))
    {
        line.pop_back();
        tmp.push_back(line);
    }


    N = tmp.size() + 2;
    M = tmp[0].size() + 2;

    for (int i = 0; i < N; ++i)
    {
        for (int j = 0; j < M; ++j)
        {
            energy[i][j] = -1000000;
        }
    }

    for (int i = 0; i < tmp.size(); ++i)
    {
        for (int j = 0; j < tmp[i].size(); ++j)
        {
            energy[i + 1][j + 1] = (int)(tmp[i][j] - '0');
        }
    }

    int flash = 0;
    int a = 0;
    int b = 0;
    for (int k = 1; k <= 2000; ++k)
    {
        for (int i = 1; i < N - 1; ++i)
        {
            for (int j = 1; j < M - 1; ++j)
            {
                flash += foo(i, j);
            }
        }
        for (int i = 1; i < N - 1; ++i)
        {
            for (int j = 1; j < M - 1; ++j)
            {
                if (energy[i][j] < 0) energy[i][j] = 0;
                // cout << energy[i][j];
            }
            // cout << "\n";
        }
        if (k <= 100)
        {
            // cout << "\n\nafter step: " << k << " --> " << flash << "\n";
            a = flash;
        }
        for (int i = 1; i < N - 1; ++i)
        {
            for (int j = 1; j < M - 1; ++j)
            {
                if (energy[i][j] != 0)
                {
                    goto ide;
                }
            }
        }
        b = k;
        break;
        ide:;
    }


    cout << "\n1.:" << a << "\n";
    cout << "\n2.:" << b << "\n";



}

int main()
{
    a();
    return 0;
}
