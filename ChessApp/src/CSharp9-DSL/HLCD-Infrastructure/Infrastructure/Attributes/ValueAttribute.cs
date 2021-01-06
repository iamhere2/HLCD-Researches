using System;

namespace HLCD.Infrastructure.Attributes
{
    /// <summary>
    /// Marks class as representing an immutable value
    /// </summary>
    [AttributeUsage(AttributeTargets.Class, AllowMultiple = false, Inherited = true)]
    public class ValueAttribute : Attribute
    {
    }
}
