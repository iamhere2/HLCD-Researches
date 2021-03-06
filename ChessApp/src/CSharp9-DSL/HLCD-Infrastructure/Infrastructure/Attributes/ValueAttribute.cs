using System;
using HLCD.Infrastructure.ComponentModel;

namespace HLCD.Infrastructure.Attributes
{
    /// <summary>
    /// Marks class as representing an immutable value type and specifies "code path" of short codes through all the component ownership hierarchy
    /// </summary>
    [AttributeUsage(AttributeTargets.Class, AllowMultiple = false, Inherited = false)]
    public sealed class ValueAttribute : Attribute
    {
        public ValueAttribute(string parentComponentCodePath)
        {
            ParentComponentCodePath = new ComponentCodePath(parentComponentCodePath ?? throw new ArgumentNullException(nameof(parentComponentCodePath)));
        }

        public ComponentCodePath ParentComponentCodePath { get; }
    }
}
