using System;

namespace HLCD.Infrastructure
{
    /// <summary>
    /// Marks property as a holder for a dependency component reference
    /// </summary>
    [AttributeUsage(AttributeTargets.Property, AllowMultiple = false)]
    public sealed class DependencyAttribute : Attribute
    {
    }
}
