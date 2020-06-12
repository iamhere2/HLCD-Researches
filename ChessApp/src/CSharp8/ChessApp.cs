using Infrastructure;

namespace ChessApp
{
    class ChessApp : IConsoleApplication
    {
        private ConsoleUI ConsoleUI { get; }

        public ChessApp(ConsoleUI consoleUI)
        {
            ConsoleUI = consoleUI;
        }

        int IConsoleApplication.Run(string[] args) => ((IConsoleApplication)ConsoleUI).Run(args);
    }
}
