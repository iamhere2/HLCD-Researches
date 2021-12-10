namespace HLCD.ChessAppExampleWithDSL.ChessApp_Internals
{
    [Interface("CA")]
    public interface IPlayer
    {
        Task<Turn> NextTurn(BoardState state);
    }
}
