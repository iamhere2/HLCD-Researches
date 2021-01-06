using System;
using System.Linq;
using ChessApp.ChessApp_Internals.ConsoleUI_Internals.Commands;
using HLCD.Infrastructure;
using Pidgin;
using static Pidgin.Parser;
using static Pidgin.Parser<char>;

namespace ChessApp.ChessApp_Internals.ConsoleUI_Internals
{
    [Component("CA-CUI-CP")]
    class CommandParser
    {
        public CommandParser()
        {
            Parser = CreateParser();
        }

        public string GetHelp() =>
@"
Game commands:
    save <game name>
    load <game name>
    del  <game name>
    list            - lists saved games
    new <color>     - starts new game, playing with black or white
    exit            - exit
    
Turns:
    <from cell> - <to cell>   - move or eat
        e.g.: E2 - E4
    
    <from cell> - <to figure> - pawn transformation
        e.g.: D7 - Queen
    
    castle - <to cell>        - castle
        e.g.: castle - A7
    
    draw                      - offer/accept draw
    reject                    - reject draw
";
        private Parser<char, Command> Parser { get; }

        private Parser<char, Command> CreateParser()
        {
            var ws = Whitespaces;
            var color = TryEnum<Color>();
            var figure = TryEnum<Figure>();

            var name =
                from first in Token(char.IsLetter)
                from rest in Token(char.IsLetterOrDigit).ManyString()
                select first + rest;

            var cell =
                from h in Token(c => c >= Board.Left && c <= Board.Right)
                from v in Token(c => c >= '1' && c <= '8')
                select Cell.At(h, int.Parse($"{v}"));

            var exit = Try(String("exit")).ThenReturn(Command.Exit).Cast<Command>();
            var list = Try(String("list")).ThenReturn(Command.List).Cast<Command>();
            var newGame = Try(String("new")).Then(ws).Then(color).Select(Command.NewGame).Cast<Command>();
            var saveGame = Try(String("save")).Then(ws).Then(name).Select(Command.Save).Cast<Command>();
            var loadGame = Try(String("load")).Then(ws).Then(name).Select(Command.Load).Cast<Command>();
            var delGame = Try(String("del")).Then(ws).Then(name).Select(Command.Delete).Cast<Command>();

            var move = Try(
                from fromCell in cell
                from toCell in ws.Then(Char('-')).Then(ws).Then(cell)
                select new Move(fromCell, toCell))
                .Cast<Turn>();

            var turn = OneOf(move).Select(t => new TurnCommand(t)).Cast<Command>();

            return OneOf(exit, list, newGame, saveGame, loadGame, delGame, turn);
        }

        private Parser<char, TEnum> TryEnum<TEnum>() where TEnum : Enum
            => OneOf(
                Enum.GetValues(typeof(TEnum))
                    .Cast<TEnum>()
                    .Select(v => Try(String(Enum.GetName(typeof(TEnum), v)!)).ThenReturn(v))
                    .ToArray());

        public Command Parse(string cmdStr)
        {
            cmdStr = cmdStr.Trim();
            var result = Parser.Parse(cmdStr);
            return result.Success
                ? result.Value
                : throw new UserError($"Bad command: {cmdStr}, {result.Error!}");
        }
    }
}
