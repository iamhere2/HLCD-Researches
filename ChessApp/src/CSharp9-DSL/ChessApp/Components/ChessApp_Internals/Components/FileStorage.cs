using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using HLCD.ChessAppExampleWithDSL.Components.ChessApp_Internals.Components.FileStorage_Internals.Components;
using HLCD.ChessAppExampleWithDSL.Data;
using HLCD.Infrastructure;

namespace HLCD.ChessAppExampleWithDSL.ChessApp_Internals
{
    [Component("CA-FS")]
    class FileStorage : IStorage
    {
        private static string BuildFileName(string name) => $"{name}.chess";

        #region Dependencies

        [Dependency]
        private IFileIO FileIO { get; }

        #endregion

        #region Internals

        [Child]
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
