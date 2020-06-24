using System;
using static MoreLinq.Extensions.ForEachExtension;
using ChessApp.ChessApp_Internals;
using Infrastructure;
using ChessApp.ConsoleUI_Internals.Commands;

namespace ChessApp.ConsoleUI_Internals
{
    public class GameCmdHandler
    {
        private GameFlow GameFlow { get; }
        private IStorage Storage { get; }
        private IConsoleIO Console { get; }

        public GameCmdHandler(GameFlow gameFlow, IStorage storage, IConsoleIO consoleIO)
        {
            GameFlow = gameFlow;
            Storage = storage;
            Console = consoleIO;
        }

        internal void Execute(GameCommand gc)
        {
            if (gc is NewGame ng)
            {
                GameFlow.NewGame(ng.UIPlayerColor);
            }
            else if (gc is LoadGame lc)
            {
                var (h, c) = Storage.Load(lc.Name);
                GameFlow.StartFrom(h, c);
            }
            else if (gc is ListGames)
            {
                Console.WriteLine("Saved games:");
                Storage.GetNames().ForEach(n => Console.WriteLine(n));
            }
            else if (gc is SaveGame sc)
            {
                var history = GameFlow.History;
                if (history == null)
                    Console.WriteLine("Nothing to save yet");
                else
                    Storage.Save(sc.Name, history, GameFlow.PlayerAColor!.Value);
            }
            else if (gc is DeleteGame dc)
            {
                Storage.Delete(dc.Name);
            }
            else
            {
                throw new ArgumentOutOfRangeException(nameof(gc));
            }
        }
    }
}