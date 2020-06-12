using System;
using System.Collections.Generic;

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
        Tower
    }

    public class GameHistory
    {
        public GameHistory(IReadOnlyCollection<BoardState> states)
        {
            States = states;
        }

        public static GameHistory ClassicInitialGameHistory { get; }
            = new GameHistory(new[] { BoardState.ClassicInitialState });

        public bool IsFinished { get => false; }

        public IReadOnlyCollection<BoardState> States { get; }

        public GameHistory With(Turn turn, BoardState nextState)
            => throw new NotImplementedException();
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
            var figures = new Dictionary<Cell, (Figure, Color)>(Figures);
            figures.Add(cell, (f, c));
            return new BoardState(figures);
        }

        public static BoardState ClassicInitialState { get; }
            = Empty
                .With(Figure.Tower, Color.White, Cell.At('A', 1))
                .With(Figure.Tower, Color.White, Cell.At('H', 1))
                .With(Figure.Tower, Color.Black, Cell.At('A', 8))
                .With(Figure.Tower, Color.Black, Cell.At('H', 8));

        public (Figure f, Color c)? GetFigureAt(Cell cell)
        {
            if (Figures.TryGetValue(cell, out var fc))
                return fc;
            else
                return null;
        }

        public BoardState Apply(Turn turn) => throw new NotImplementedException();
    }

    public class Board
    {
        public const int Low = 1;
        public const int High = 8;
        public const char Left = 'A';
        public const char Right = 'H';

        public static Color ColorOf(Cell cell)
            => ((cell.H - Left + 1) + (cell.V - Low + 1)) % 2 == 0 ? Color.White : Color.Black;
    }

    public readonly struct Cell
    {
        public char H { get; }
        public int V { get; }

        public Cell(char h, int v)
        {
            H = h;
            V = v;
        }

        public static Cell At(char h, int v) => new Cell(h, v);
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
