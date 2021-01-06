using System;
using System.Linq;
using System.Threading.Tasks;

namespace ChessApp.ChessApp_Internals
{
    internal class AiPlayer : IPlayer
    {
        #region Dependencies

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
