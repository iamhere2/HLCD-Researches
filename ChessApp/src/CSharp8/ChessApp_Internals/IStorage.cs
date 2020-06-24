using System.Collections.Generic;

namespace ChessApp.ChessApp_Internals
{
    public interface IStorage
    {
        (GameHistory, Color) Load(string name);
        void Save(string name, GameHistory gameHistory, Color uiPlayerColor);
        void Delete(string name);
        IReadOnlyCollection<string> GetNames();
    }
}
