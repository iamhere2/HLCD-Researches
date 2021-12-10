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
                                !(t.Namespace ?? string.Empty).Contains("_Internals", StringComparison.InvariantCultureIgnoreCase)),
                            publicOnly: true)
                        .AsImplementedInterfaces()
                        .AsSelf()
                        .WithSingletonLifetime());

                var serviceProvider = services.BuildServiceProvider();

                var consoleApp = serviceProvider.GetRequiredService<IConsoleApplication>();
                var exitCode = consoleApp.Run(args ?? Array.Empty<string>());

                return exitCode;
            }
#pragma warning disable CA1031 // We need it on top level
            catch (Exception e)
#pragma warning restore CA1031  
            {
                Console.WriteLine($"Unhandled exception: {e}");
                Console.ReadLine();
                return -250;
            }
        }
    }
}
