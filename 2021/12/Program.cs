using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Numerics;

namespace asd
{
    class Program
    {
        static private HashSet<string> seen = new HashSet<string>();
        static private HashSet<List<string>> steps = new HashSet<List<string>>();

        static void foo(Dictionary<string, List<string>> path, List<string> n, string hol)
        {
            // Console.Write(hol + " ");
            n.Add(hol);
            if (hol == "end")
            {
                seen.Add(hol);
                steps.Add(n);
                foreach (var i in n)
                {
                    Console.Write(i + " ");
                }
                return;
                // foo(path, n, hol);
            }


            foreach (var p in path[hol])
            {
                if (seen.Contains(p))
                {
                    // if (n.Count > 0)
                    // {
                    //     n.RemoveAt(n.Count - 1);
                    // }
                    continue;
                }
                if (hol.ToLower() == hol)
                {
                    seen.Add(hol);
                }

                if (!seen.Contains("end"))
                {
                    foo(path, n, p);
                }
                
            }
            // Console.WriteLine();
            // if (hol != "end")
            // {
            // }
        }

        
        static void feladat()
        {
            StreamReader sr = new StreamReader("../../../i.txt");
            Dictionary<string, List<string>> path = new Dictionary<string, List<string>>();
            while (!sr.EndOfStream)
            {
                string line = sr.ReadLine();
                string honnan = line.Split("-")[0];
                string hova = line.Split("-")[1];


                if (path.ContainsKey(honnan) && hova != "start")
                {
                    path[honnan].Add(hova);
                }
                else if (hova != "start")
                {
                    path.Add(honnan, new List<string>(){ hova });
                }

                
                    
                if (path.ContainsKey(hova))
                {
                    path[hova].Add(honnan);
                }
                else if (honnan != "start" && hova != "end")
                {
                    path.Add(hova, new List<string>(){ honnan });
                }
                    
                
                
            }

            


            foreach (var p in path)
            {
                Console.Write(p.Key + " -> ");
                foreach (var h in p.Value)
                {
                    Console.Write(h + " ");
                }
                Console.WriteLine();
            }
            Console.WriteLine("----------");

            
            // foo(path, new List<string>(){}, "start");
            string hol = "start";
            foreach (var p in path[hol])
            {
                Console.Write("start ");
                seen.Clear();
                foo(path, new List<string>() { }, p);
                Console.WriteLine();
            }
        }
        
        
        static void Main(string[] args)
        {
            feladat();
            Console.ReadKey();
        }
    }
}