using System;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;

namespace ChessApp.ChessApp_Internals
{
    public class GameFlow
    {
        #region Dependencies

        private IPlayer PlayerA { get; }
        private IPlayer PlayerB { get; }
        private RulesEngine RulesEngine { get; }

        #endregion

        #region Construction

        public GameFlow(IPlayer playerA, IPlayer playerB, RulesEngine rulesEngine)
        {
            PlayerA = playerA;
            PlayerB = playerB;
            RulesEngine = rulesEngine;
        }

        #endregion

        #region Mutable state

        public GameHistory? History { get; private set; }

        public Color? PlayerAColor { get; private set; }
        public Color? PlayerBColor { get; private set; }

        private CancellationTokenSource? CancellationTokenSource { get; set; }

        #endregion

        public void NewGame(Color playerAColor)
            => StartFrom(GameHistory.ClassicInitialGameHistory, playerAColor);

        public void StartFrom(GameHistory history, Color playerAColor)
        {
            History = history;
            PlayerAColor = playerAColor;
            PlayerBColor = playerAColor.Invert();

            if (CancellationTokenSource != null)
            {
                CancellationTokenSource.Cancel();
                CancellationTokenSource.Dispose();
            }

            CancellationTokenSource = new CancellationTokenSource();

            new Task(() => TurnCycle(playerAColor, CancellationTokenSource.Token)).Start();
        }

        private async void TurnCycle(Color playerAColor, CancellationToken cancellationToken)
        {
            var player = playerAColor switch
            {
                Color.White => PlayerA,
                Color.Black => PlayerB,
                _ => throw new ArgumentOutOfRangeException(nameof(playerAColor))
            };

            while (!History!.IsFinished && !cancellationToken.IsCancellationRequested)
            {
                var state = History!.States.Last();

                var turn = await player.NextTurn(state).Task.ConfigureAwait(false);

                if (cancellationToken.IsCancellationRequested)
                    break;

                var violation = RulesEngine.Check(state, turn);

                if (violation != null)
                    throw new RuleViolationError(violation);

                var nextState = RulesEngine.Apply(state, turn);

                History = History.With(turn, nextState);

                player = player == PlayerA ? PlayerB : PlayerA;
            }
        }
    }
}
