using System;
using System.Threading.Tasks;
using ChessApp.Internals;

namespace ChessApp.ConsoleUI_Internals
{
    class TurnCmdHandler : IPlayer
    {
        #region Mutable state

        private TaskCompletionSource<Turn>? TurnPromise;

        private BoardState? BoardState;

        #endregion

        internal TurnError? MakeTurn(Turn turn) => throw new NotImplementedException();

        TaskCompletionSource<Turn> IPlayer.NextTurn(BoardState state)
        {
            TurnPromise = new TaskCompletionSource<Turn>();
            BoardState = state;
            return TurnPromise;
        }
    }
}
