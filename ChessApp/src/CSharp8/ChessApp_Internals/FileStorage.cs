using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using ChessApp.ChessApp_Internals.FileStorage_Internals;
using Infrastructure;

namespace ChessApp.ChessApp_Internals
{
    internal class FileStorage : IStorage
    {
        private static string BuildFileName(string name) => $"{name}.chess";

        private IFileIO FileIO { get; }
        private GameSerializer Serializer { get; }

        public FileStorage(IFileIO fileIO, GameSerializer serializer)
        {
            FileIO = fileIO ?? throw new ArgumentNullException(nameof(fileIO));
            Serializer = serializer ?? throw new ArgumentNullException(nameof(serializer));
        }

        public void Delete(string name)
            => FileIO.Delete(BuildFileName(name));

        public IReadOnlyCollection<string> GetNames()
            => FileIO.GetFiles(FileIO.GetCurrentDirectory(), "*.chess")
                .Select(fn => Path.GetFileNameWithoutExtension(fn))
                .ToArray();

        public (GameHistory, Color) Load(string name)
            => Serializer.Deserialize(FileIO.ReadFile(BuildFileName(name)));

        public void Save(string name, GameHistory gameHistory, Color uiPlayerColor)
            => FileIO.WriteFile(BuildFileName(name),
                Serializer.Serialize(gameHistory, uiPlayerColor));
    }
}
