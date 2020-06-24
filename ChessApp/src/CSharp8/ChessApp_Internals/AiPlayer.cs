using System;
using System.Threading.Tasks;

namespace ChessApp.ChessApp_Internals
{
    internal class AiPlayer : IPlayer
    {
        public TaskCompletionSource<Turn> NextTurn(BoardState state) => throw new NotImplementedException();
    }
}
