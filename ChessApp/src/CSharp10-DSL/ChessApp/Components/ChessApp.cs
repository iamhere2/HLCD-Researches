using System;
using HLCD.ChessAppExampleWithDSL.ChessApp_Internals;
using HLCD.ChessAppExampleWithDSL.Components.ChessApp_Internals.Components;
using HLCD.Infrastructure;

namespace HLCD.ChessAppExampleWithDSL
{
    [Component("CA")]
    public sealed class ChessApp : IConsoleApplication
    {
        #region Dependencies

        [Dependency]
        private IConsoleIO Console { get; }

        [Dependency]
        private IFileIO FileIO { get; }

        #endregion

        #region Internals

        [Child]
        [DelegatingImplementation(typeof(IConsoleApplication))]
        private ConsoleUI ConsoleUI { get; }

        [Child]
        private GameFlow GameFlow { get; }

        [Child]
        private AiPlayer AiPlayer { get; }

        [Child]
        private RulesEngine RulesEngine { get; }

        [Child]
        private FileStorage FileStorage { get; }

        #endregion

        #region Construction

        public ChessApp(IConsoleIO console, IFileIO fileIO)
        {
            // TODO: How to replace this boilerplate code with auto-generated?
            // Maybe, classical DI infrastructure could be used here in a local scope?

            Console = console ?? throw new ArgumentNullException(nameof(console));
            FileIO = fileIO ?? throw new ArgumentNullException(nameof(fileIO));

            RulesEngine = new RulesEngine();
            AiPlayer = new AiPlayer(RulesEngine);
            FileStorage = new FileStorage(FileIO);

            var lazyConsolePlayer = new LateBindingPlayerAdapter();

            GameFlow = new GameFlow(lazyConsolePlayer, AiPlayer, RulesEngine);
            ConsoleUI = new ConsoleUI(GameFlow, Console, FileStorage, RulesEngine);

            lazyConsolePlayer.Bind(ConsoleUI);
        }

        #endregion

        #region Interface delegation

        // TODO: How to replace this boilerplate code with auto-generated?
        int IConsoleApplication.Run(string[] args) => ((IConsoleApplication)ConsoleUI).Run(args);

        #endregion
    }
}
