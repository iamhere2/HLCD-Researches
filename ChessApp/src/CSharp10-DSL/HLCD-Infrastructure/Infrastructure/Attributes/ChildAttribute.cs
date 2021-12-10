using System;

namespace HLCD.Infrastructure
{
    /// <summary>
    /// Marks property as a holder for a children component reference
    /// </summary>
    [AttributeUsage(AttributeTargets.Property, AllowMultiple = false)]
    public sealed class ChildAttribute : Attribute
    {
    }
}
