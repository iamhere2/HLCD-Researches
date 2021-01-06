using System.Collections.Generic;

namespace HLCD.Infrastructure.ComponentModel
{
    public record Module(
        string Name,
        IReadOnlyList<ComponentType> RootComponentTypes,
        IReadOnlyList<ValueType> TopLevelValueTypes,
        IReadOnlyList<InterfaceType> TopLevelInterfaceTypes);
}
