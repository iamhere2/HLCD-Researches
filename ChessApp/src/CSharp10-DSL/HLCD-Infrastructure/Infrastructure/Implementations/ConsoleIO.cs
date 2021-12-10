using System;

namespace HLCD.Infrastructure.Implementations
{
#pragma warning disable IDE0040 // Instantiated by Scrutor
#pragma warning disable CA1812  // Instantiated by Scrutor
    class ConsoleIO : IConsoleIO
#pragma warning restore IDE0040 
#pragma warning restore CA1812 
    {
        public string ReadLine() => Console.ReadLine() ?? string.Empty;
        public void SetBackgroundColor(ConsoleColor color) => Console.BackgroundColor = color;
        public void SetForegroundColor(ConsoleColor color) => Console.ForegroundColor = color;
        public void Write(string s) => Console.Write(s);
        public void WriteLine(string s) => Console.WriteLine(s);
    }
}
