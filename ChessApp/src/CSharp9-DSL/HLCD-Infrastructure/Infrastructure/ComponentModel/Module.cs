using System.Collections.Generic;
using HLCD.Infrastructure.Attributes;

namespace HLCD.Infrastructure.ComponentModel
{
    [Value]
    public record Module(
        string Name,
        IReadOnlyList<ComponentType> RootComponentTypes,
        IReadOnlyList<ValueType> TopLevelValueTypes,
        IReadOnlyList<InterfaceType> TopLevelInterfaceTypes);
}
