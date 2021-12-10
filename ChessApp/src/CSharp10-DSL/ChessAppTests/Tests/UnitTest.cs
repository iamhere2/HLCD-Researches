using System;
using System.Linq;
using HLCD.ChessAppExampleWithDSL;
using HLCD.ChessAppExampleWithDSL.Components.ChessApp_Internals.Components.FileStorage_Internals.Components;
using HLCD.ChessAppExampleWithDSL.Data;
using Xunit;

namespace ChessAppTests
{
    public sealed class UnitTests
    {
        [Fact]
        public void GameCanBeSerializedAndDeserialized()
        {
            var gs = new GameSerializer();
            var h = new GameHistory(new[] { BoardState.ClassicInitialState }, Array.Empty<Turn>());

            string s = gs.Serialize(h, Color.White);

            Assert.Contains("King", s, StringComparison.InvariantCulture);
            Assert.Contains("White", s, StringComparison.InvariantCulture);

            var (dh, _) = gs.Deserialize(s);

            Assert.Equal((Figure.Pawn, Color.White), dh.States.Single()[Cell.At('E', 2)]);
        }
    }
}
