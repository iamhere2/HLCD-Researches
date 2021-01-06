using System.IO;
using HLCD.Infrastructure;

namespace ChessApp.Infrastructure.Implementations
{
    class FileIO : IFileIO
    {
        public string[] GetFiles(string path, string pattern) => Directory.GetFiles(path, pattern);
        public string GetCurrentDirectory() => Directory.GetCurrentDirectory();
        public string ReadFile(string path) => File.ReadAllText(path);
        public void WriteFile(string path, string content) => File.WriteAllText(path, content);
        public void Delete(string path) => File.Delete(path);
    }
}
