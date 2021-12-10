using System;
using HLCD.Infrastructure.ComponentModel;

namespace HLCD.Infrastructure.Attributes
{
    /// <summary>
    /// Marks interface as root or belonging to some component by "code path" of short codes through all the component ownership hierarchy
    /// </summary>
    [AttributeUsage(AttributeTargets.Interface, AllowMultiple = false, Inherited = false)]
    public sealed class InterfaceAttribute : Attribute
    {
        public InterfaceAttribute(string parentComponentCodePathAsString)
        {
            ParentComponentCodePathAsString = parentComponentCodePathAsString ?? throw new ArgumentNullException(nameof(parentComponentCodePathAsString));
            ParentComponentCodePath = new ComponentCodePath(parentComponentCodePathAsString);
        }

        public ComponentCodePath ParentComponentCodePath { get; }
        public string ParentComponentCodePathAsString { get; }
    }
}
