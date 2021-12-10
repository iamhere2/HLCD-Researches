using System;
using HLCD.Infrastructure.ComponentModel;

namespace HLCD.Infrastructure
{
    /// <summary>
    /// Marks class as a component type and specifies "code path" of short codes through all the component ownership hierarchy
    /// </summary>
    [AttributeUsage(AttributeTargets.Class, AllowMultiple = false, Inherited = false)]
    public sealed class ComponentAttribute : Attribute
    {
        public ComponentAttribute(string parentComponentCodePathAsString)
        {
            ParentComponentCodePathAsString = parentComponentCodePathAsString ?? throw new ArgumentNullException(nameof(parentComponentCodePathAsString));
            ParentComponentCodePath = new ComponentCodePath(parentComponentCodePathAsString);
        }

        public ComponentCodePath ParentComponentCodePath { get; }

        public string ParentComponentCodePathAsString { get; }
    }
}
