using System.Threading.Tasks;

namespace ChessApp.Internals
{
    public interface IPlayer
    {
        TaskCompletionSource<Turn> NextTurn(BoardState state);
    }
}
