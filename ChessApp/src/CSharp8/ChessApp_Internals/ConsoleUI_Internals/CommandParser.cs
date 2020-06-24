using System;
using System.Linq;
using ChessApp.ConsoleUI_Internals.Commands;
using Pidgin;
using static Pidgin.Parser;
using static Pidgin.Parser<char>;

namespace ChessApp.ConsoleUI_Internals
{
    class CommandParser
    {
        public CommandParser()
        {
            Parser = CreateCommandParser();
        }

        public string GetHelp() =>
@"
Game commands:
    save <game name>
    load <game name>
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
        private Parser<char, Command> CreateCommandParser()
        {
            var ws = Whitespaces;
            var color = CreateEnumParser<Color>();
            var figure = CreateEnumParser<Figure>();
            var name = from first in Token(char.IsLetter)
                       from rest in Token(char.IsLetterOrDigit).ManyString()
                       select first + rest;

            var exit = Try(String("exit")).ThenReturn(Command.Exit).Cast<Command>();
            var list = Try(String("list")).ThenReturn(Command.List).Cast<Command>();

            var newGame = Try(String("new")).Then(ws).Then(color).Select(Command.NewGame).Cast<Command>();
            var saveGame = Try(String("save")).Then(ws).Then(name).Select(Command.Save).Cast<Command>();
            var loadGame = Try(String("load")).Then(ws).Then(name).Select(Command.Load).Cast<Command>();

            return
                OneOf(exit, list, newGame, saveGame, loadGame);
        }

        private Parser<char, TEnum> CreateEnumParser<TEnum>() where TEnum : Enum
            => OneOf(
                Enum.GetValues(typeof(TEnum))
                    .Cast<TEnum>()
                    .Select(v => String(Enum.GetName(typeof(TEnum), v)!).ThenReturn(v))
                    .ToArray());

        private Parser<char, Command> Parser { get; }

        public Command Parse(string cmdStr)
        {
            cmdStr = cmdStr.Trim();
            var result = Parser.Parse(cmdStr);
            return result.Success
                ? result.Value
                : throw new UserError($"Unknown command: {cmdStr}");
        }
    }
}
