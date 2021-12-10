using System;
using System.Collections.Generic;
using System.Text.RegularExpressions;

namespace HLCD.Infrastructure.ComponentModel
{
    public record ComponentCodePath
    {
        private const string CodePathRegexPattern = @"^([A-Z]+)(\-[A-Z]+)*$";
        private static readonly Regex CodePathRegex = new(CodePathRegexPattern, RegexOptions.Compiled);

        public ComponentCodePath(string codePath)
        {
            CodePath = codePath ?? throw new ArgumentNullException(nameof(codePath));

            if (codePath.Length == 0)
            {
                Parts = Array.Empty<string>();
            }
            else
            {
                if (!CodePathRegex.IsMatch(codePath))
                    throw new ArgumentOutOfRangeException(nameof(codePath), codePath, $"CodePath should match regex: {CodePathRegexPattern}");

                Parts = codePath.Split('-');
            }
        }

        public string CodePath { get; }

        public IReadOnlyList<string> Parts { get; set; }

        public bool IsEmpty => CodePath.Length == 0;

        public static readonly ComponentCodePath Empty = new(string.Empty);
    }
}
