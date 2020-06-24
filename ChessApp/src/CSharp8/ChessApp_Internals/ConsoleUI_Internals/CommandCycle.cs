using System;
using ChessApp.ConsoleUI_Internals.Commands;
using Infrastructure;

namespace ChessApp.ConsoleUI_Internals
{
    class CommandCycle : IConsoleApplication
    {
        private IConsoleIO Console { get; }
        private GameCmdHandler GameCmdHandler { get; }
        private TurnCmdHandler TurnCmdHandler { get; }
        private BoardPrinter BoardPrinter { get; }
        private CommandParser CommandParser { get; }

        public CommandCycle(
            IConsoleIO consoleIO,
            GameCmdHandler gameCmdHandler,
            TurnCmdHandler turnCmdHandler,
            BoardPrinter boardPrinter,
            CommandParser commandParser)
        {
            Console = consoleIO;
            GameCmdHandler = gameCmdHandler;
            TurnCmdHandler = turnCmdHandler;
            BoardPrinter = boardPrinter;
            CommandParser = commandParser;
        }

        int IConsoleApplication.Run(string[] args)
        {
            PrintWelcome();

            while (true)
            {
                BoardPrinter.PrintCurrentGameState();

                Console.Write("> ");

                string cmdStr = Console.ReadLine();

                try
                {
                    var cmd = CommandParser.Parse(cmdStr);
                    if (cmd is TurnCommand tc)
                        MakeTurn(tc.Turn);
                    else if (cmd is GameCommand gc)
                        GameCmdHandler.Execute(gc);
                    else if (cmd is Exit)
                        return 0;
                    else
                        throw new NotImplementedException($"Not implemented command: {cmd}");
                }
                catch (UserError ue)
                {
                    PrintError(ue);
                    PrintHelp();
                }
            }
        }

        private void PrintWelcome()
        {
            Console.WriteLine("Welcome to ChessApp!");
        }

        private void PrintHelp()
        {
            Console.WriteLine(
                CommandParser.GetHelp());
        }

        private void MakeTurn(Turn turn)
        {
            TurnError? e =  TurnCmdHandler.MakeTurn(turn);
            if (e != null)
            {
                PrintError(e);
            }
        }

        private void PrintError(UserError e)
        {
            Console.WriteLine($"Error: {e.Message}");
        }
    }
}
