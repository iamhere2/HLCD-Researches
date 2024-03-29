using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using HLCD.Infrastructure.Attributes;

namespace HLCD.ChessAppExampleWithDSL.Data
{
    [Value("CA")]
    public abstract record Turn
    {
    }

    public enum Color
    {
        Black,
        White
    }

    public static class ColorExtensions
    {
        public static Color Invert(this Color color)
            => color switch
            {
                Color.Black => Color.White,
                Color.White => Color.Black,
                _ => throw new ArgumentOutOfRangeException(nameof(color))
            };
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

    [Value("CA")]
    public sealed class GameHistory
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

    [Value("CA")]
    public sealed class BoardState
    {
        private static readonly ReadOnlyDictionary<Cell, (Figure, Color)> None =
            new ReadOnlyDictionary<Cell, (Figure, Color)>(
                new Dictionary<Cell, (Figure, Color)>());

        private BoardState()
        {
            Figures = None;
        }

        private BoardState(IDictionary<Cell, (Figure, Color)> figures)
        {
            Figures = new ReadOnlyDictionary<Cell, (Figure, Color)>(figures);
        }

        public IReadOnlyDictionary<Cell, (Figure, Color)> Figures { get; }

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

        public BoardState Without(Cell cell)
        {
            if (Figures.ContainsKey(cell))
            {
                var figures = new Dictionary<Cell, (Figure, Color)>(
                    Figures.Where(kv => kv.Key != cell));

                return new BoardState(figures);
            }
            else
            {
                return this;
            }
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

        public (Figure f, Color c)? this[Cell cell]
        {
            get
            {
                if (Figures.TryGetValue(cell, out var fc))
                    return fc;
                else
                    return null;
            }
        }
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
            if (!IsValid(h, v))
                throw new ArgumentException($"Invalid cell value: ({h}, {v})");

            H = h;
            V = v;
        }

        // TODO: SRE violation and logic duplication with CommandParser! CellParser should be extracted as well as Move/Turn Parser
        public static Cell Parse(string s)
        {
            if (s is null)
                throw new ArgumentNullException(nameof(s));

            if (s.Length != 2)
                throw new ArgumentOutOfRangeException(nameof(s));

            char h = s[0];
            int v = int.Parse(s[1].ToString(), System.Globalization.NumberStyles.Integer, System.Globalization.CultureInfo.InvariantCulture);

            if (!IsValid(h, v))
                throw new ArgumentOutOfRangeException(nameof(s), s, "Invalid cell value");

            return At(h, v);
        }

        public override string ToString() => $"{H}{V}";

        public static bool IsValid(char h, int v) =>
               h >= Board.Left && h <= Board.Right
            && v >= Board.Low  && v <= Board.High;

        public static Cell At(char h, int v) => new Cell(h, v);

        public override bool Equals(object? obj) => obj is Cell other && Equals(other);

        public override int GetHashCode() => (H, V).GetHashCode();

        public bool Equals(Cell other) => (H, V) == (other.H, other.V);

        public static bool operator ==(Cell left, Cell right) => left.Equals(right);

        public static bool operator !=(Cell left, Cell right) => !(left == right);
    }

    [Value("CA")]
    public record Move(Cell From, Cell To) : Turn;

    [Value("CA")]
    public record RuleViolation(string Description);
}
