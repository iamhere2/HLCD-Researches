using System;

namespace HLCD.Infrastructure
{
    public interface IConsoleIO
    {
        void Write(string s);
        void WriteLine(string s);
        string ReadLine();
        void SetBackgroundColor(ConsoleColor color);
        void SetForegroundColor(ConsoleColor color);
    }
}
