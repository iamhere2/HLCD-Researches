using System.Threading.Tasks;

namespace ChessApp.ChessApp_Internals
{
    public interface IPlayer
    {
        TaskCompletionSource<Turn> NextTurn(BoardState state);
    }
}
