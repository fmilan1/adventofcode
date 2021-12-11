#include <iostream>
#include <fstream>
#include <vector>
#include <stack>

using namespace std;

void A()
{
    fstream read("../i.txt");
    string line;
    vector<string> chunks;
    while (getline(read, line))
    {
        chunks.push_back(line);
    }
    vector<char> open = {'(', '[', '{', '<'};
    vector<char> close = {')', ']', '}', '>'};
    vector<int> values1 = {3, 57, 1197, 25137};
    vector<int> values2 = {1, 2, 3, 4};
    stack<char> s;
    vector<char> bads;
    long sum1 = 0, sum2 = 0;
    vector<long> sum2s;
    for (int i = 0; i < chunks.size(); ++i)
    {
        sum2 = 0;
        // cout << "\ni: " << i << " ";
        while (s.size() > 0)
        {
            s.pop();
        }
        chunks[i].pop_back();
        for (int j = 0; j < chunks[i].size(); j++)
        {
            char c = chunks[i][j];
            vector<char>::iterator o;
            o = find(open.begin(), open.end(), c);
            s.push(c);
            if (o == open.end()) //nincs benne
            {
                vector<char>::iterator cl;
                cl = find(close.begin(), close.end(), c);
                s.pop();
                o = find(open.begin(), open.end(), s.top());
                if (o - open.begin() == cl - close.begin())
                {
                    s.pop();
                } else
                {
                    bads.push_back(c);
                    goto nextline;
                }
            }
        }
        while (s.size() > 0)
        {
            vector<char>::iterator it = find(open.begin(), open.end(), s.top());
            int v = values2[it - open.begin()];
            sum2 = sum2 * 5 + v;
            s.pop();
        }
        sum2s.push_back(sum2);
        nextline:
        continue;
    }
    for (int i = 0; i < bads.size(); ++i)
    {
        vector<char>::iterator it = find(close.begin(), close.end(), bads[i]);
        sum1 += values1[it - close.begin()];
    }

    std::sort(sum2s.begin(), sum2s.end());
    cout << sum1;
    cout << endl << sum2s.at(sum2s.size() / 2) << "\n";
}

int main()
{
    A();
    return 0;
}
