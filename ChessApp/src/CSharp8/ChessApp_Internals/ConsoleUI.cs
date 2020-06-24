using System;
using System.Threading.Tasks;
using ChessApp.ChessApp_Internals.ConsoleUI_Internals;
using Infrastructure;

namespace ChessApp.ChessApp_Internals
{
    class ConsoleUI : IConsoleApplication, IPlayer
    {
        #region Internals

        private CommandCycle CommandCycle { get; }
        private TurnCmdHandler TurnCmdHandler { get; }
        private GameCmdHandler GameCmdHandler { get; }
        private CommandParser CommandParser { get; }
        private BoardPrinter BoardPrinter { get; }

        #endregion

        #region Construction

        public ConsoleUI(GameFlow gameFlow, IConsoleIO console, IStorage storage, RulesEngine rulesEngine)
        {
            if (gameFlow is null)
                throw new ArgumentNullException(nameof(gameFlow));

            if (console is null)
                throw new ArgumentNullException(nameof(console));

            if (storage is null)
                throw new ArgumentNullException(nameof(storage));

            if (rulesEngine is null)
                throw new ArgumentNullException(nameof(rulesEngine));

            GameCmdHandler = new GameCmdHandler(gameFlow, storage, console);
            TurnCmdHandler = new TurnCmdHandler(rulesEngine);
            BoardPrinter = new BoardPrinter(console, gameFlow);
            CommandParser = new CommandParser();
            CommandCycle = new CommandCycle(console, GameCmdHandler, TurnCmdHandler, BoardPrinter, CommandParser);
        }

        #endregion

        #region Interface delegation

        int IConsoleApplication.Run(string[] args) => ((IConsoleApplication)CommandCycle).Run(args);

        TaskCompletionSource<Turn> IPlayer.NextTurn(BoardState state) => ((IPlayer)TurnCmdHandler).NextTurn(state);

        #endregion
    }
}
