using System;
using HLCD.ChessAppExampleWithDSL.ChessApp_Internals.ConsoleUI_Internals.Commands;
using HLCD.ChessAppExampleWithDSL.Components.ChessApp_Internals.Components.ConsoleUI_Internals.Components;
using HLCD.ChessAppExampleWithDSL.Data;
using HLCD.ChessAppExampleWithDSL.Errors;
using HLCD.Infrastructure;

namespace HLCD.ChessAppExampleWithDSL.ChessApp_Internals.ConsoleUI_Internals
{
    [Component("CA-CUI-CC")]
    class CommandCycle : IConsoleApplication
    {
        #region Dependencies

        [Dependency]
        private IConsoleIO Console { get; }

        [Dependency]
        private GameCmdHandler GameCmdHandler { get; }

        [Dependency]
        private TurnCmdHandler TurnCmdHandler { get; }

        [Dependency]
        private BoardPrinter BoardPrinter { get; }

        [Dependency]
        private CommandParser CommandParser { get; }

        #endregion

        #region Construction

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

        #endregion

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
                catch (UserErrorException ue)
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
            TurnError? e = TurnCmdHandler.MakeTurn(turn);
            if (e != null)
            {
                PrintError(e);
            }
        }

        private void PrintError(UserErrorException e)
        {
            Console.SetForegroundColor(ConsoleColor.Red);
            Console.WriteLine($"Error: {e.Message}");
            Console.SetForegroundColor(ConsoleColor.Gray);
        }
    }
}
