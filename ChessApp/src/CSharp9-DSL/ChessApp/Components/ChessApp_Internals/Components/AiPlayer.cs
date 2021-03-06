using System;
using System.Linq;
using System.Threading.Tasks;
using HLCD.ChessAppExampleWithDSL.ChessApp_Internals;
using HLCD.ChessAppExampleWithDSL.Data;
using HLCD.Infrastructure;

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
            RulesEngine = rulesEngine ?? throw new ArgumentNullException(nameof(rulesEngine));
        }

        #endregion

        public TaskCompletionSource<Turn> NextTurn(BoardState state)
        {
            var result = new TaskCompletionSource<Turn>();
            result.SetResult(
                RulesEngine.GetPossibleTurns(state, Color.Black).First());
            return result;
        }
    }
}
