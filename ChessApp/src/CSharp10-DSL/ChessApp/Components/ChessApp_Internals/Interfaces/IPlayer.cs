using System.Threading.Tasks;
using HLCD.ChessAppExampleWithDSL.Data;
using HLCD.Infrastructure.Attributes;

namespace HLCD.ChessAppExampleWithDSL.ChessApp_Internals
{
    [Interface("CA")]
    public interface IPlayer
    {
        Task<Turn> NextTurn(BoardState state);
    }
}
