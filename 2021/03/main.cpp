#include <iostream>
#include <vector>
#include <string>
#include <fstream>
#include <cmath>

int bintodec(long long n)
{
    std::string bin = std::to_string(n);
    int dec = 0;
    for (int i = bin.size() - 1; i >= 0; i--)
    {
        std::string tmp = "";
        tmp += bin[i];
        dec += std::stoi(tmp) * pow(2, bin.size() - i - 1);
        
    }
    return dec;
}

int A()
{
    std::string line;
    std::ifstream read("../input.txt");

    std::vector<std::string> input;
    long long gamma = 0, epsilon = 0;
    while (std::getline(read, line))
    {
        input.push_back(line);
    }
    
    


    for (int i = 0; i < input[0].size(); i++)
    {
        int ones = 0, zeros = 0;
        for (int j = 0; j < input.size(); j++)
        {
            if (input[j][i] == '1') ones++;
            else zeros++;
        }
        if (ones > zeros)
        {
            gamma *= 10;
            gamma += 1;
            epsilon *= 10;
        }
        else
        {
            gamma *= 10;
            epsilon *= 10;
            epsilon += 1;
        }
    }
    
    gamma = bintodec(gamma);
    epsilon = bintodec(epsilon);
    
    
    return gamma * epsilon;
}


int B()
{
    std::string line;
    std::ifstream read("../input.txt");

    std::vector<std::string> input;
    
    while (std::getline(read, line))
    {
        input.push_back(line);
    }



    std::vector<std::string> save = input;
    for (int i = 0; i < input[0].size(); i++)
    {
        std::vector<std::string> o;
        if (input.size() == 1) break;
        int ones = 0, zeros = 0;
        for (int j = 0; j < input.size(); j++)
        {
            if (input[j][i] == '1') ones++;
            else zeros++;
        }
        if (ones > zeros || ones == zeros)
        {
            for (int j = 0; j < input.size(); j++)
            {
                if (input[j][i] == '1')
                {
                    o.push_back(input[j]);
                }
            }
        }
        else if (zeros > ones)
        {
            for (int j = 0; j < input.size(); j++)
            {
                if (input[j][i] == '0')
                {
                    o.push_back(input[j]);
                }
            }
        }
        input = o;
    }
    
    int oxigen = bintodec(std::stoll(input[0]));

    input = save;
    for (int i = 0; i < input[0].size(); i++)
    {
        std::vector<std::string> c;
        if (input.size() == 1) break;
        int ones = 0, zeros = 0;
        for (int j = 0; j < input.size(); j++)
        {
            if (input[j][i] == '1') ones++;
            else zeros++;
        }
        if (ones > zeros || ones == zeros)
        {
            for (int j = 0; j < input.size(); j++)
            {
                if (input[j][i] == '0')
                {
                    c.push_back(input[j]);
                }
            }
        }
        else if (zeros > ones)
        {
            for (int j = 0; j < input.size(); j++)
            {
                if (input[j][i] == '1')
                {
                    c.push_back(input[j]);
                }
            }
        }
        input = c;
    }
    int co2 = bintodec(std::stoll(input[0]));
    return oxigen * co2;
}



int main()
{
    std::cout << A() << "\n";
    std::cout << B() << "\n";
    
    return 0;
}
