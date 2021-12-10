using System.IO;

namespace HLCD.Infrastructure.Implementations
{
#pragma warning disable IDE0040 // Instantiated by Scrutor
#pragma warning disable CA1812  // Instantiated by Scrutor
    class FileIO : IFileIO
#pragma warning restore IDE0040
#pragma warning restore CA1812
    {
        public string[] GetFiles(string path, string pattern) => Directory.GetFiles(path, pattern);
        public string GetCurrentDirectory() => Directory.GetCurrentDirectory();
        public string ReadFile(string path) => File.ReadAllText(path);
        public void WriteFile(string path, string content) => File.WriteAllText(path, content);
        public void Delete(string path) => File.Delete(path);
    }
}
