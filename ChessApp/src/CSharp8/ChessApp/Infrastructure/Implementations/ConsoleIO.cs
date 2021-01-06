using System;

namespace Infrastructure.Implementations
{
    class ConsoleIO : IConsoleIO
    {
        public string ReadLine() => Console.ReadLine();
        public void SetBackgroundColor(ConsoleColor color) => Console.BackgroundColor = color;
        public void SetForegroundColor(ConsoleColor color) => Console.ForegroundColor = color;
        public void Write(string s) => Console.Write(s);
        public void WriteLine(string s) => Console.WriteLine(s);
    }
}
