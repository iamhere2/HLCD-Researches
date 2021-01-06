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
        public ComponentAttribute(string parentComponentCodePath)
        {
            ParentComponentCodePath = new ComponentCodePath(parentComponentCodePath ?? throw new ArgumentNullException(nameof(parentComponentCodePath)));
        }

        public ComponentCodePath ParentComponentCodePath { get; }
    }
}
