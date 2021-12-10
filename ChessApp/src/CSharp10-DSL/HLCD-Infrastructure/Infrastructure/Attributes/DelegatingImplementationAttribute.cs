using System;

namespace HLCD.Infrastructure
{
    /// <summary>
    /// Marks the property (which holds dependency or internal component reference) as
    /// an instance, which should be used to delegate interface implementation
    /// </summary>
    [AttributeUsage(AttributeTargets.Property, AllowMultiple = true)]
    public sealed class DelegatingImplementationAttribute : Attribute
    {
        public DelegatingImplementationAttribute(Type interfaceType)
        {
            InterfaceType = interfaceType ?? throw new ArgumentNullException(nameof(interfaceType));
        }

        public Type InterfaceType { get; }
    }
}
