using System;
using HLCD.Infrastructure.ComponentModel;

namespace HLCD.Infrastructure.Attributes
{
    /// <summary>
    /// Marks class as representing an immutable value type and specifies "code path" of short codes through all the component ownership hierarchy
    /// </summary>
    [AttributeUsage(AttributeTargets.Class | AttributeTargets.Struct, AllowMultiple = false, Inherited = false)]
    public sealed class ValueAttribute : Attribute
    {
        public ValueAttribute(string parentComponentCodePathAsString)
        {
            ParentComponentCodePathAsString = parentComponentCodePathAsString ?? throw new ArgumentNullException(nameof(parentComponentCodePathAsString));
            ParentComponentCodePath = new ComponentCodePath(parentComponentCodePathAsString);
        }

        public ComponentCodePath ParentComponentCodePath { get; }

        public string ParentComponentCodePathAsString { get; }
    }
}
