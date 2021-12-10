namespace HLCD.ChessAppExampleWithDSL.ChessApp_Internals.ConsoleUI_Internals.Commands
{
    [Value("CA-CUI")]
    abstract record Command
    {
        public static Exit Exit = new();
        public static ListGames List = new();
        public static NewGame NewGame(Color uiPlayerColor) => new(uiPlayerColor);
        public static SaveGame Save(string name) => new(name);
        public static LoadGame Load(string name) => new(name);
        public static DeleteGame Delete(string name) => new(name);
        public static Move Move(Cell from, Cell to) => new(from, to);
    }

    record TurnCommand(Turn Turn) : Command;

    abstract record GameCommand : Command;

    record Exit : Command;

    record NewGame(Color UIPlayerColor) : GameCommand;

    record ListGames : GameCommand;

    record LoadGame(string Name) : GameCommand;

    record DeleteGame(string Name) : GameCommand;

    record SaveGame(string Name) : GameCommand;
}
