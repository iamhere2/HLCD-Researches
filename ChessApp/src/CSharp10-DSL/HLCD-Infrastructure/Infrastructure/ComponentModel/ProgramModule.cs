using System.Collections.Generic;

namespace HLCD.Infrastructure.ComponentModel
{
    public record ProgramModule(
        string Name,
        IReadOnlyList<ComponentType> RootComponentTypes,
        IReadOnlyList<ValueType> TopLevelValueTypes,
        IReadOnlyList<InterfaceType> TopLevelInterfaceTypes);
}
