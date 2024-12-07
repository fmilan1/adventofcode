#include <iostream>
#include <fstream>
#include <map>
#include "../utils.h"
#include <numeric>
#include <set>
#include <algorithm>
#include <string>

int A()
{
    std::fstream read("../i.txt");
    std::string line;

    int count = 0;
    while (getline(read, line))
    {
        std::string a = string_split(line, " | ")[1];
        std::string b = string_split(a, "\r")[0];
        std::vector<std::string> c = string_split(b, " ");
        for (int i = 0; i < c.size(); ++i)
        {
            if (c[i].size() == 2 || c[i].size() == 3 || c[i].size() == 4 || c[i].size() == 7) count++;
        }
    }
    return count;
}




std::vector<char> asd(std::vector<char> d, std::string  s)
{
    d.clear();
    for (int j = 0; j < s.size(); ++j)
    {
        d.push_back(s[j]);
    }
    std::sort(d.begin(), d.end());
    return d;
}

std::string vector_merge_elements_to_string(std::vector<char> d1, std::vector<char> d2)
{
    std::string tmp = "";
    for (int i = 0; i < d1.size(); ++i)
    {
        tmp.push_back(d1[i]);
    }
    std::sort(tmp.begin(), tmp.end());
    for (int i = 0; i < d2.size(); ++i)
    {
        if (!std::count(tmp.begin(), tmp.end(), d2[i]))
        {
            tmp.push_back(d2[i]);
        }
    }
    std::sort(tmp.begin(), tmp.end());
    return tmp;
}

int count_difference(std::string str1, std::string str2)
{
    int c = 0;
    for (int i = 0; i < str2.size(); ++i)
    {
        if (!str1.find(str2[i])) c++;
    }
    return c;
}

char get_difference(std::string str1, std::string str2)
{
    char tmp;
    for (int i = 0; i < str1.size(); ++i)
    {
        for (int j = i; j < str2.size(); ++j)
        {
            if (str2[j] != str1[i]) tmp = str1[i];
            else break;
        }
    }
    return tmp;
}

int B()
{
    std::fstream read("../i.txt");
    std::string line;
    
    std::vector<std::set<int>> digitslist;
    
    digitslist.push_back({0, 1, 2, 4, 5, 6});
    digitslist.push_back({2, 5});
    digitslist.push_back({0, 2, 3, 4, 6});
    digitslist.push_back({0, 2, 3, 5, 6});
    digitslist.push_back({1, 2, 3, 5});
    digitslist.push_back({0, 1, 3, 5, 6});
    digitslist.push_back({0, 1, 3, 4, 5, 6});
    digitslist.push_back({0, 2, 5});
    digitslist.push_back({0, 1, 2, 3, 4, 5, 6});
    digitslist.push_back({0, 1, 2, 3, 5, 6});
    char top, tleft, tright, center, bleft, bright, bottom;
    int sum = 0;
    while (getline(read, line))
    {
        std::string digits = "0123456789";
        std::vector<std::string> patterns = string_split(string_split(line, " | ")[0], " ");
        for (std::string& pattern : patterns)
        {
            std::sort(pattern.begin(), pattern.end());
        }
        do
        {
            bool good_length = true;
            for (int i = 0; i < digits.size(); ++i)
            {
                std::string tmp = "";
                tmp += digits[i];
                int d = std::stoi(tmp);
                // std::cout << i << " " << patterns[i].size() << " " << digitslist[i].size() << " " << d << "\n";
                if (patterns[i].size() != digitslist[d].size())
                {
                    good_length = false;
                    break;
                }
                
                
            }
            if (good_length)
            {
                std::string segments = "abcdefg";
                do
                {
                    bool good_segment = true;
                    for (int i = 0; i < digits.size(); ++i)
                    {
                        int d = std::stoi(std::to_string(digits[i])) - '0';
                        // std::cout << d << std::endl;
                        std::string pattern = patterns[i];
                        
                        // std::cout << pattern << std::endl;
                        
                        for (int idx : digitslist[d])
                        {
                            // std::cout << "idx: " << idx << std::endl;
                            if (pattern.find(segments[idx]) == std::string::npos)
                            {
                                good_segment = false;
                                break;
                            }
                        }
                        if (!good_segment) break;
                    }
                    if (good_segment)
                    {
                        std::cout << digits << '\n';
                        goto loop;
                    }
                } while (std::next_permutation(segments.begin(), segments.end()));
            // acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf
            }
        } while(std::next_permutation(digits.begin(), digits.end()));
 loop:       std::vector<std::string> values = string_split(string_split(line, " | ")[1], " ");
        values[values.size() - 1].pop_back();
        for (std::string& value : values)
        {
            std::sort(value.begin(), value.end());
        }
        int d = 1000;
        int s = 0;
        for (std::string& value : values)
        {
            for (int i = 0; i < patterns.size(); ++i)
            {
                if (value == patterns[i])
                {
                    // std::cout << i << " " << digits[i] << std::endl;
                    s += d * (digits[i] - '0');
                    d /= 10;
                }
            }
        }
        // std::cout << s << std::endl;
        sum += s;
    }
    return sum ;
}





int main()
{
    // std::cout << A() << std::endl;
    std::cout << B() << std::endl;

   
    
    return 0;
}
