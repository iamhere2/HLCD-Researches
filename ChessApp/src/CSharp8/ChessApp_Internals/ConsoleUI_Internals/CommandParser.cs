using System;

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
            => cmdStr switch
            {
                "new black" => Command.NewGame(Color.Black),
                "new white" => Command.NewGame(Color.White),
                "exit" => Command.Exit,
                _ => throw new UserError($"Unknown command: \"{cmdStr}\"")
            };
    }
}
