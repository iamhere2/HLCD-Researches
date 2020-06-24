using System;
using System.Linq;

namespace ChessApp.ConsoleUI_Internals
{
    class CommandParser
    {
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

        public Command Parse(string cmdStr)
        {
            cmdStr = cmdStr.Trim();
            var first = cmdStr;
            var second = "";

            if (cmdStr.Contains(' '))
            {
                var words = cmdStr.Split(' ', StringSplitOptions.RemoveEmptyEntries);
                if (words.Length > 2)
                    throw new UserError($"Unknown command: \"{cmdStr}\"");

                first = words.First();
                second = words.Skip(1).First();
            }

            return (first, second) switch
            {
                ("new", var colorStr) => Command.NewGame(Enum.Parse<Color>(colorStr)),
                ("exit", "") => Command.Exit,
                ("save", var name) => Command.Save(name),
                ("load", var name) => Command.Load(name),
                ("list", "") => Command.List,
                _ => throw new UserError($"Unknown command: \"{cmdStr}\"")
            };
        }
    }
}
