namespace ChessApp.ConsoleUI_Internals
{
    abstract class Command
    {
        public static Exit Exit = new Exit();
        public static NewGame NewGame(Color uiPlayerColor) => new NewGame(uiPlayerColor);
    }

    class TurnCommand : Command
    {
        public TurnCommand(Turn turn)
        {
            Turn = turn;
        }

        public Turn Turn { get; }
    }

    abstract class GameCommand : Command
    {
    }

    class Exit : Command
    {
    }

    class NewGame : GameCommand
    {
        public NewGame(Color uiPlayerColor)
        {
            UIPlayerColor = uiPlayerColor;
        }

        public Color UIPlayerColor { get; }
    }

    class ListGames : GameCommand
    {
    }

    class LoadGame : GameCommand
    {
        public LoadGame(string name)
        {
            Name = name;
        }

        public string Name { get; }
    }

    class DeleteGame : GameCommand
    {
        public DeleteGame(string name)
        {
            Name = name;
        }

        public string Name { get; }
    }

    class SaveGame : GameCommand
    {
        public SaveGame(string name)
        {
            Name = name;
        }

        public string Name { get; }
    }
}
