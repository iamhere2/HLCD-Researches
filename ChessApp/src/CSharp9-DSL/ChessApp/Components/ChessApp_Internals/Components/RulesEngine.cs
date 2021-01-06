using System;
using System.Collections.Generic;
using HLCD.ChessAppExampleWithDSL.Data;
using HLCD.ChessAppExampleWithDSL.Errors;
using HLCD.Infrastructure;

namespace HLCD.ChessAppExampleWithDSL.ChessApp_Internals
{
    [Component("CA-RE")]
    public sealed class RulesEngine
    {
        public RuleViolation? Check(BoardState state, Turn turn)
        {
            if (state is null)
            {
                throw new ArgumentNullException(nameof(state));
            }

            if (turn is null)
            {
                throw new ArgumentNullException(nameof(turn));
            }

            return null;
        }

        public BoardState Apply(BoardState state, Turn turn)
        {
            if (state is null)
            {
                throw new ArgumentNullException(nameof(state));
            }

            if (turn is null)
            {
                throw new ArgumentNullException(nameof(turn));
            }

            if (turn is Move move)
            {
                var (f, c) = state[move.From] ?? throw new UserErrorException($"There are no figure at {move.From}");

                if (move.From == move.To)
                    throw new RuleViolationError(new RuleViolation("Figure can't move to the same cell"));

                state = state.Without(move.From);

                if (state[move.To].HasValue)
                {
                    state = state.Without(move.To);
                }

                return state.With(f, c, move.To);
            }

            throw new NotSupportedException();
        }

        public IEnumerable<Turn> GetPossibleTurns(BoardState state, Color playerColor)
        {
            if (state is null)
            {
                throw new ArgumentNullException(nameof(state));
            }

            foreach (var (cell, (_, color)) in state.Figures)
            {
                if (color == playerColor)
                {
                    foreach (var t in GetPossibleTurns(state, cell))
                        yield return t;
                }
            }
        }

        private IEnumerable<Turn> GetPossibleTurns(BoardState state, Cell cell)
        {
            var (figure, color) = state[cell]
                ?? throw new InvalidOperationException($"There are no figure at cell {cell}");

            yield return new Move(cell, Cell.At(cell.H, cell.V - 1));
        }
    }
}
