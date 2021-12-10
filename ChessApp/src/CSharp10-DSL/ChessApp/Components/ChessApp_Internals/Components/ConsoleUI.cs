using HLCD.ChessAppExampleWithDSL.ChessApp_Internals.ConsoleUI_Internals;
using HLCD.ChessAppExampleWithDSL.Components.ChessApp_Internals.Components.ConsoleUI_Internals.Components;

namespace HLCD.ChessAppExampleWithDSL.ChessApp_Internals
{
    [Component("CA-CUI")]
    class ConsoleUI : IConsoleApplication, IPlayer
    {
        #region Internals

        [Child]
        [DelegatingImplementation<IConsoleApplication>]
        private CommandCycle CommandCycle { get; }

        [Child]
        [DelegatingImplementation<IPlayer>]
        private TurnCmdHandler TurnCmdHandler { get; }

        [Child]
        private GameCmdHandler GameCmdHandler { get; }

        [Child]
        private CommandParser CommandParser { get; }

        [Child]
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

        Task<Turn> IPlayer.NextTurn(BoardState state) => ((IPlayer)TurnCmdHandler).NextTurn(state);

        #endregion
    }
}
