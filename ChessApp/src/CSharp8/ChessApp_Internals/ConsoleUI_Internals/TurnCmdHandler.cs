using System;
using System.Threading.Tasks;

namespace ChessApp.ChessApp_Internals.ConsoleUI_Internals
{
    class TurnCmdHandler : IPlayer
    {
        #region Dependencies

        private RulesEngine RulesEngine { get; }

        #endregion

        #region Construction

        public TurnCmdHandler(RulesEngine rulesEngine)
        {
            RulesEngine = rulesEngine ?? throw new ArgumentNullException(nameof(rulesEngine));
        }

        #endregion

        #region Mutable state

        private TaskCompletionSource<Turn>? TurnPromise;

        private BoardState? BoardState;

        #endregion

        internal TurnError? MakeTurn(Turn turn)
        {
            if (TurnPromise is null)
                throw new UserError("Turn is not expected");

            var ruleViolation = RulesEngine.Check(BoardState!, turn);

            if (ruleViolation is null)
            {
                TurnPromise.SetResult(turn);
                return null;
            }
            else
            {
                return new RuleViolationError(ruleViolation);
            }
        }

        TaskCompletionSource<Turn> IPlayer.NextTurn(BoardState state)
        {
            TurnPromise = new TaskCompletionSource<Turn>();
            BoardState = state;
            return TurnPromise;
        }
    }
}
