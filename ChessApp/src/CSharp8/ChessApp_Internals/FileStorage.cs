using System;
using System.Collections.Generic;
using Infrastructure;

namespace ChessApp.ChessApp_Internals
{
    class FileStorage : IStorage
    {
        private IFileIO FileIO { get; }

        public FileStorage(IFileIO fileIO)
        {
            FileIO = fileIO;
        }

        public void Delete(string name) => throw new NotImplementedException();
        public IReadOnlyCollection<string> GetNames() => throw new NotImplementedException();
        public (GameHistory, Color) Load(string name) => throw new NotImplementedException();
        public void Save(string name, GameHistory? gameHistory) => throw new NotImplementedException();
    }
}
