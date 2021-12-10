using System.Collections.Generic;
using HLCD.ChessAppExampleWithDSL.Data;
using HLCD.Infrastructure.Attributes;

namespace HLCD.ChessAppExampleWithDSL.ChessApp_Internals
{
    [Interface("CA")]
    public interface IStorage
    {
        (GameHistory, Color) Load(string name);
        void Save(string name, GameHistory gameHistory, Color uiPlayerColor);
        void Delete(string name);
        IReadOnlyCollection<string> GetNames();
    }
}
