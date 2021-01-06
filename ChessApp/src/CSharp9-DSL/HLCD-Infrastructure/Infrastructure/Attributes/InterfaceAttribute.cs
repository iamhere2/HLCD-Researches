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
        public InterfaceAttribute(string parentComponentCodePath)
        {
            ParentComponentCodePath = new ComponentCodePath(parentComponentCodePath ?? throw new ArgumentNullException(nameof(parentComponentCodePath)));
        }

        public ComponentCodePath ParentComponentCodePath { get; }
    }
}
