using System;
using System.Collections.Generic;
using System.Linq;

namespace ChessApp
{
    public class Turn
    {
    }

    public enum Color
    {
        Black,
        White
    }

    public enum Figure
    {
        Knight,
        King,
        Rook,
        Bishop,
        Queen,
        Pawn
    }

    public class GameHistory
    {
        public GameHistory(IReadOnlyCollection<BoardState> states, IReadOnlyCollection<Turn> turns)
        {
            States = states;
            Turns = turns;
        }

        public static GameHistory ClassicInitialGameHistory { get; }
            = new GameHistory(new[] { BoardState.ClassicInitialState }, Array.Empty<Turn>());

        public bool IsFinished { get => false; }

        public IReadOnlyCollection<BoardState> States { get; }

        public IReadOnlyCollection<Turn> Turns { get; }

        public GameHistory With(Turn turn, BoardState nextState)
            => new GameHistory(
                States.Append(nextState).ToArray(),
                Turns.Append(turn).ToArray());
    }

    public class BoardState
    {
        private BoardState()
        {
            Figures = new Dictionary<Cell, (Figure, Color)>();
        }

        private BoardState(Dictionary<Cell, (Figure, Color)> figures)
        {
            Figures = figures;
        }

        private readonly Dictionary<Cell, (Figure, Color)> Figures;

        public static BoardState Empty { get; } = new BoardState();

        public BoardState With(Figure f, Color c, Cell cell)
        {
            var figures =
                new Dictionary<Cell, (Figure, Color)>(Figures)
                {
                    { cell, (f, c) }
                };

            return new BoardState(figures);
        }

        public static BoardState ClassicInitialState { get; }
            = Empty
                .With(Figure.Rook,   Color.White, Cell.At('A', 1))
                .With(Figure.Knight, Color.White, Cell.At('B', 1))
                .With(Figure.Bishop, Color.White, Cell.At('C', 1))
                .With(Figure.Queen,  Color.White, Cell.At('D', 1))
                .With(Figure.King,   Color.White, Cell.At('E', 1))
                .With(Figure.Bishop, Color.White, Cell.At('F', 1))
                .With(Figure.Knight, Color.White, Cell.At('G', 1))
                .With(Figure.Rook,   Color.White, Cell.At('H', 1))
                .With(Figure.Pawn,   Color.White, Cell.At('A', 2))
                .With(Figure.Pawn,   Color.White, Cell.At('B', 2))
                .With(Figure.Pawn,   Color.White, Cell.At('C', 2))
                .With(Figure.Pawn,   Color.White, Cell.At('D', 2))
                .With(Figure.Pawn,   Color.White, Cell.At('E', 2))
                .With(Figure.Pawn,   Color.White, Cell.At('F', 2))
                .With(Figure.Pawn,   Color.White, Cell.At('G', 2))
                .With(Figure.Pawn,   Color.White, Cell.At('H', 2))
                .With(Figure.Rook,   Color.Black, Cell.At('A', 8))
                .With(Figure.Knight, Color.Black, Cell.At('B', 8))
                .With(Figure.Bishop, Color.Black, Cell.At('C', 8))
                .With(Figure.Queen,  Color.Black, Cell.At('D', 8))
                .With(Figure.King,   Color.Black, Cell.At('E', 8))
                .With(Figure.Bishop, Color.Black, Cell.At('F', 8))
                .With(Figure.Knight, Color.Black, Cell.At('G', 8))
                .With(Figure.Rook,   Color.Black, Cell.At('H', 8))
                .With(Figure.Pawn,   Color.Black, Cell.At('A', 7))
                .With(Figure.Pawn,   Color.Black, Cell.At('B', 7))
                .With(Figure.Pawn,   Color.Black, Cell.At('C', 7))
                .With(Figure.Pawn,   Color.Black, Cell.At('D', 7))
                .With(Figure.Pawn,   Color.Black, Cell.At('E', 7))
                .With(Figure.Pawn,   Color.Black, Cell.At('F', 7))
                .With(Figure.Pawn,   Color.Black, Cell.At('G', 7))
                .With(Figure.Pawn,   Color.Black, Cell.At('H', 7));

        public (Figure f, Color c)? GetFigureAt(Cell cell)
        {
            if (Figures.TryGetValue(cell, out var fc))
                return fc;
            else
                return null;
        }

        public BoardState Apply(Turn turn) => throw new NotImplementedException();
    }

    public static class Board
    {
        public const int Low = 1;
        public const int High = 8;
        public const char Left = 'A';
        public const char Right = 'H';

        public static Color ColorOf(Cell cell)
            => (cell.H - Left + cell.V - Low) % 2 == 0 ? Color.Black : Color.White;
    }

    public readonly struct Cell : IEquatable<Cell>
    {
        public char H { get; }
        public int V { get; }

        public Cell(char h, int v)
        {
            H = h;
            V = v;
            Validate();
        }

        public static Cell Parse(string s)
        {
            if (s is null)
                throw new ArgumentNullException(nameof(s));

            if (s.Length != 2)
                throw new ArgumentOutOfRangeException(nameof(s));

            char h = s[0];
            int v = int.Parse(s[1].ToString());

            if (!IsValid(h, v))
                throw new ArgumentOutOfRangeException("Invalid cell value");

            return At(h, v);
        }

        public override string ToString() => $"{H}{V}";

        private void Validate()
        {
            if (!IsValid(H, V))
                throw new InvalidOperationException($"Invalid cell value: ({H}, {V})");
        }

        public static bool IsValid(char h, int v) =>
               h >= Board.Left && h <= Board.Right
            && v >= Board.Low  && v <= Board.High;

        public static Cell At(char h, int v) => new Cell(h, v);

        public override bool Equals(object? obj) => obj is Cell other && Equals(other);

        public override int GetHashCode() => (H, V).GetHashCode();

        public bool Equals(Cell other) => (H, V) == (other.H, other.V);

        public static bool operator ==(Cell left, Cell right)
        {
            return left.Equals(right);
        }

        public static bool operator !=(Cell left, Cell right)
        {
            return !(left == right);
        }
    }

    public class RuleViolation
    {
        public RuleViolation(string description)
        {
            Description = description;
        }

        public string Description { get; }
    }
}
