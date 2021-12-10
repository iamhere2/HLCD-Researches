using System;
using System.Collections.Generic;

namespace HLCD.Infrastructure.ComponentModel
{
    public record ComponentType(
        ComponentCodePath ParentComponentCodePath,
        Type ClrType,

        IReadOnlyList<ComponentType> ChildrenComponentTypes,
        IReadOnlyList<ValueType> ChildrenValueTypes,
        IReadOnlyList<InterfaceType> ChildrenInterfaceTypes,

        IReadOnlyList<InterfaceType> ProvidedInterfaces,

        IReadOnlyList<(string name, ComponentType)> ChildrenComponentInstances,
        IReadOnlyList<(string name, ComponentType)> ComponentDependencies,

        IReadOnlyList<(InterfaceType, string childName)> DelegatingImplementations
    )
        : AbstractType(ParentComponentCodePath, ClrType);
}
