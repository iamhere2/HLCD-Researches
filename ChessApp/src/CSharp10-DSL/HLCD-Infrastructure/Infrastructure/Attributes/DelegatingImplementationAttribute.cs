using System;

namespace HLCD.Infrastructure
{
    /// <summary>
    /// Marks the property (which holds dependency or internal component reference) as
    /// an instance, which should be used to delegate interface implementation
    /// </summary>
    [AttributeUsage(AttributeTargets.Property, AllowMultiple = true)]
    public sealed class DelegatingImplementationAttribute<TInterfaceType> : Attribute
    {
        public DelegatingImplementationAttribute()
        {
            InterfaceType = typeof(TInterfaceType);
        }

        public Type InterfaceType { get; }
    }
}
