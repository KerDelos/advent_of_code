var lines = File.ReadLines("input.txt").ToList();
var pb1 = lines.Zip(lines.Skip(1), (first, second) => Int32.Parse(second) > Int32.Parse(first) ? 1 : 0).Sum();
Console.WriteLine($"Problem one : {pb1}");