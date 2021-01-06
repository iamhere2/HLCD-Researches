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

        #region Dependencies

        private IFileIO FileIO { get; }

        #endregion

        #region Internals

        private GameSerializer Serializer { get; }

        #endregion

        #region Construction

        public FileStorage(IFileIO fileIO)
        {
            FileIO = fileIO ?? throw new ArgumentNullException(nameof(fileIO));
            Serializer = new GameSerializer();
        }

        #endregion

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
