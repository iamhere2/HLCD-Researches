namespace Infrastructure
{
    public interface IFileIO
    {
        string GetCurrentDirectory();
        string[] GetFiles(string path, string mask);
        string ReadFile(string path);
        void WriteFile(string path, string content);
        void Delete(string path);
    }
}
