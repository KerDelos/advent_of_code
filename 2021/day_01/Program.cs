var lines = File.ReadLines("input.txt").ToList();
var pb1 = lines.Zip(lines.Skip(1), (first, second) => Int32.Parse(second) > Int32.Parse(first) ? 1 : 0).Sum();
Console.WriteLine($"Problem one : {pb1}");

List<U> Windowed<T,U>(List<T> list, int windowSize, Func<List<T>,U> lambda)
{
    var result = new List<U>();
    for (int i = 0; i <= list.Count() - windowSize; i++)
    {   
        result.Add(lambda(list.GetRange(i,windowSize))); 
    }
    return result;
}

var sums = Windowed(lines,3, (l) => l.Select(x => Int32.Parse(x)).Sum());
var pb2 = Windowed<int,int>(sums, 2, (l) => l[1] > l[0] ? 1 : 0).Sum();

Console.WriteLine($"Problem two : {pb2}");