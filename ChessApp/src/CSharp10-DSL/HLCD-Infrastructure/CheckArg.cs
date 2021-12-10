using System;
using System.Diagnostics.CodeAnalysis;
using System.Runtime.CompilerServices;

namespace CSharpUtils
{
    public static class CheckArg
    {
        public static T NotNull<T>([NotNull] T? value, [CallerArgumentExpression("value")] string name = "") where T : class
        {
            if (value is null)
                throw new ArgumentNullException(name);

            return value;
        }

        public static string NotEmpty([NotNull] string? value, [CallerArgumentExpression("value")] string name = "")
        {
            if (string.IsNullOrWhiteSpace(value))
                throw new ArgumentOutOfRangeException(name, $"String argument '{name}' can not be empty");

            return value;
        }
    }
}
