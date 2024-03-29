using HLCD.ChessAppExampleWithDSL.ChessApp_Internals;

namespace HLCD.ChessAppExampleWithDSL.Components.ChessApp_Internals.Components
{
    [Component("CA-AP")]
    class AiPlayer : IPlayer
    {
        #region Dependencies

        [Dependency]
        private RulesEngine RulesEngine { get; }

        #endregion

        #region Construction

        public AiPlayer(RulesEngine rulesEngine)
        {
            RulesEngine = CheckArg.NotNull(rulesEngine);
        }

        #endregion

        public Task<Turn> NextTurn(BoardState state)
            // Just get the first possible turn yet
            => Task.FromResult(RulesEngine.GetPossibleTurns(state, Color.Black).First());
    }
}
