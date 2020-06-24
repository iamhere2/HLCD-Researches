using System;
using ChessApp.ChessApp_Internals;
using Infrastructure;

namespace ChessApp
{
    class ChessApp : IConsoleApplication
    {
        #region Dependencies

        private IConsoleIO Console { get; }
        private IFileIO FileIO { get; }

        #endregion

        #region Internals

        private ConsoleUI ConsoleUI { get; }
        private GameFlow GameFlow { get; }
        private AiPlayer AiPlayer { get; }
        private RulesEngine RulesEngine { get; }
        private FileStorage FileStorage { get; }

        #endregion

        #region Construction

        public ChessApp(IConsoleIO console, IFileIO fileIO)
        {
            Console = console ?? throw new ArgumentNullException(nameof(console));
            FileIO = fileIO ?? throw new ArgumentNullException(nameof(fileIO));

            AiPlayer = new AiPlayer();
            RulesEngine = new RulesEngine();
            FileStorage = new FileStorage(FileIO);

            var lazyConsolePlayer = new LateBindingPlayerAdapter();

            GameFlow = new GameFlow(lazyConsolePlayer, AiPlayer, RulesEngine);
            ConsoleUI = new ConsoleUI(GameFlow, Console, FileStorage, RulesEngine);

            lazyConsolePlayer.Bind(ConsoleUI);
        }

        #endregion

        #region Interface delegation

        int IConsoleApplication.Run(string[] args) => ((IConsoleApplication)ConsoleUI).Run(args);

        #endregion
    }
}
