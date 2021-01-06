using System;
using System.Linq;
using ChessApp;
using ChessApp.ChessApp_Internals.FileStorage_Internals;
using Xunit;

namespace ChessAppTests
{
    public class UnitTests
    {
        [Fact]
        public void GameCanBeSerializedAndDeserialized()
        {
            var gs = new GameSerializer();
            var h = new GameHistory(new[] { BoardState.ClassicInitialState }, Array.Empty<Turn>());

            string s = gs.Serialize(h, Color.White);

            Assert.Contains("King", s);
            Assert.Contains("White", s);

            var (dh, _) = gs.Deserialize(s);

            Assert.Equal((Figure.Pawn, Color.White), dh.States.Single()[Cell.At('E', 2)]);
        }
    }
}
