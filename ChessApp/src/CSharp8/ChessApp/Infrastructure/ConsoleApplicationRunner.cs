using System;
using Microsoft.Extensions.DependencyInjection;

namespace Infrastructure
{
    static class ConsoleApplicationRunner
    {
        static int Main(string[] args)
        {
            try
            {
                var services = new ServiceCollection();

                services.Scan(cfg =>
                    cfg.FromEntryAssembly()
                        .AddClasses(
                            cls => cls.Where(t =>
                                !(t.Namespace ?? string.Empty).Contains("_Internals")),
                            publicOnly: false)
                        .AsImplementedInterfaces()
                        .AsSelf()
                        .WithSingletonLifetime());

                var serviceProvider = services.BuildServiceProvider();

                var consoleApp = serviceProvider.GetRequiredService<IConsoleApplication>();
                var exitCode = consoleApp.Run(args ?? Array.Empty<string>());

                Console.WriteLine($"Exit code = {exitCode}");
                Console.ReadLine();
                return exitCode;
            }
            catch (Exception e)
            {
                Console.WriteLine($"Unhandled exception: {e}");
                Console.ReadLine();
                return -250;
            }
        }
    }
}
