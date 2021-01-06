using System;
using Microsoft.Extensions.DependencyInjection;

namespace HLCD.Infrastructure
{
    public static class ConsoleApplicationRunner
    {
        public static int Run(string[] args)
        {
            try
            {
                var services = new ServiceCollection();

                services.Scan(cfg =>
                    cfg.FromCallingAssembly()
                        .AddClasses(publicOnly: false)
                        .AsImplementedInterfaces()
                        .AsSelf()
                        .WithSingletonLifetime());

                services.Scan(cfg =>
                    cfg.FromEntryAssembly()
                        .AddClasses(
                            cls => cls.Where(t =>
                                !(t.Namespace ?? string.Empty).Contains("_Internals")),
                            publicOnly: true)
                        .AsImplementedInterfaces()
                        .AsSelf()
                        .WithSingletonLifetime());

                var serviceProvider = services.BuildServiceProvider();

                var consoleApp = serviceProvider.GetRequiredService<IConsoleApplication>();
                var exitCode = consoleApp.Run(args ?? Array.Empty<string>());

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
