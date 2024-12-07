#include <iostream>
#include <fstream>
#include <vector>

int A()
{
    std::ifstream read("../i.txt");
    std::string line;
    int x = 0, y = 0;
    while (std::getline(read, line))
    {
        std::vector<std::string> command = {line.substr(0, line.find(' ')), line.substr(line.find(' ') + 1, line.size())};
        if (command[0] == "forward")
        {
            x += std::stoi(command[1]);
        }
        else if (command[0] == "down")
        {
            y += std::stoi(command[1]);
        }
        else if (command[0] == "up")
        {
            y -= std::stoi(command[1]);
        }
    }
    
    return x * y;
}


int B()
{
    std::ifstream read("../i.txt");
    std::string line;
    int x = 0, y = 0, a = 0;
    while (std::getline(read, line))
    {
        std::vector<std::string> command = {line.substr(0, line.find(' ')), line.substr(line.find(' ') + 1, line.size())};
        if (command[0] == "forward")
        {
            x += std::stoi(command[1]);
            y += a * std::stoi(command[1]);
        }
        else if (command[0] == "down")
        {
            a += std::stoi(command[1]);
        }
        else if (command[0] == "up")
        {
            a -= std::stoi(command[1]);
        }
    }

    return x * y;
}

int main()
{
    std::vector<int> a = {3, 4, 5, 6, 7};
    std::string asd = "45678";
    std::cout << A() << "\n";
    std::cout << B() << "\n";
    return 0;
}
