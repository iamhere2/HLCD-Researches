using System;
using HLCD.Infrastructure.ComponentModel;

namespace HLCD.Infrastructure
{
    /// <summary>
    /// Marks class as a component type and specifies "code path" of short codes through all the component ownership hierarchy
    /// </summary>
    [AttributeUsage(AttributeTargets.Class, AllowMultiple = false, Inherited = false)]
    public class ComponentAttribute : Attribute
    {
        public ComponentAttribute(string codePath)
        {
            CodePath = new ComponentCodePath(codePath ?? throw new ArgumentNullException(nameof(codePath)));
        }

        public ComponentCodePath CodePath { get; }
    }
}
