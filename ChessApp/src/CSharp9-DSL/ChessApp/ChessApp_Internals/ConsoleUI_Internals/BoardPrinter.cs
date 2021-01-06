using System;
using System.Collections.Generic;
using System.Linq;
using HLCD.Infrastructure;

namespace ChessApp.ChessApp_Internals.ConsoleUI_Internals
{
    [Component("CA-CUI-BP")]
    public class BoardPrinter
    {
        #region Dependencies

        private IConsoleIO Console { get; }
        private GameFlow GameFlow { get; }

        #endregion

        #region Construction

        public BoardPrinter(IConsoleIO consoleIO, GameFlow gameFlow)
        {
            Console = consoleIO;
            GameFlow = gameFlow;
        }

        #endregion

        public void PrintCurrentGameState()
        {
            var history = GameFlow.History;
            if (history == null)
                PrintEmpty();
            else
                Print(history.States.Last());
        }

        private void PrintEmpty() => Console.WriteLine("(board is empty)");

        private void Print(BoardState boardState)
        {
            foreach (var v in Range(Board.High, Board.Low))
            {
                SetDefaultColors();
                Console.Write($" {v}");

                foreach (var h in Range(Board.Left, Board.Right))
                {
                    var cell = Cell.At((char)h, v);
                    PrintCell(Board.ColorOf(cell), boardState[cell]);
                }

                SetDefaultColors();
                Console.WriteLine("");
            }

            Console.Write("  ");

            foreach (var h in Range(Board.Left, Board.Right))
                Console.Write($" {(char)h}");

            Console.WriteLine("");
        }

        private static IEnumerable<int> Range(int a, int b)
            => a < b
                ? Enumerable.Range(a, b - a + 1)
                : Enumerable.Range(b, a - b + 1).Reverse();

        private void SetDefaultColors()
        {
            Console.SetForegroundColor(ConsoleColor.Gray);
            Console.SetBackgroundColor(ConsoleColor.Black);
        }

        private void PrintCell(Color cellColor, (Figure f, Color c)? figure)
        {
            Console.SetBackgroundColor(TranslateBackgroundColor(cellColor));

            if (figure == null)
            {
                Console.Write("  ");
            }
            else
            {
                var (f, c) = figure.Value;
                Console.SetForegroundColor(TranslateForegroundColor(c));
                Console.Write(TranslateFigure(f));
            }
        }

        private ConsoleColor TranslateBackgroundColor(Color color)
            => color switch
            {
                Color.Black => ConsoleColor.DarkRed,
                Color.White => ConsoleColor.DarkYellow,
                _ => throw new ArgumentOutOfRangeException()
            };

        private ConsoleColor TranslateForegroundColor(Color color)
            => color switch
            {
                Color.Black => ConsoleColor.Black,
                Color.White => ConsoleColor.White,
                _ => throw new ArgumentOutOfRangeException()
            };

        private string TranslateFigure(Figure figure)
            => figure switch
            {
                Figure.Knight => "Kn",
                Figure.Rook => "Rk",
                Figure.King => "Kg",
                Figure.Queen => "Qn",
                Figure.Bishop => "Bs",
                Figure.Pawn => "pw",
                _ => throw new ArgumentOutOfRangeException()
            };
    }
}
