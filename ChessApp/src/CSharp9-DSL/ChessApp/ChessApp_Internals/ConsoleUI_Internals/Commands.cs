using HLCD.Infrastructure.Attributes;

namespace ChessApp.ChessApp_Internals.ConsoleUI_Internals.Commands
{
    [Value]
    abstract record Command
    {
        public static Exit Exit = new Exit();
        public static ListGames List = new ListGames();
        public static NewGame NewGame(Color uiPlayerColor) => new NewGame(uiPlayerColor);
        public static SaveGame Save(string name) => new SaveGame(name);
        public static LoadGame Load(string name) => new LoadGame(name);
        public static DeleteGame Delete(string name) => new DeleteGame(name);
        public static Move Move(Cell from, Cell to) => new Move(from, to);
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
