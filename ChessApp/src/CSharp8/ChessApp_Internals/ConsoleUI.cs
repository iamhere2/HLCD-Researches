using System.Threading.Tasks;
using ChessApp.ConsoleUI_Internals;
using ChessApp.Internals;
using Infrastructure;

namespace ChessApp
{
    class ConsoleUI : IConsoleApplication, IPlayer
    {
        private CommandCycle CommandCycle { get; }
        private TurnCmdHandler TurnCmdHandler { get; }

        public ConsoleUI(CommandCycle commandCycle, TurnCmdHandler turnCmdHandler)
        {
            CommandCycle = commandCycle;
            TurnCmdHandler = turnCmdHandler;
        }

        int IConsoleApplication.Run(string[] args) => ((IConsoleApplication)CommandCycle).Run(args);
        TaskCompletionSource<Turn> IPlayer.NextTurn(BoardState state) => ((IPlayer)TurnCmdHandler).NextTurn(state);
    }
}
