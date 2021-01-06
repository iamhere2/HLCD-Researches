using System;

namespace HLCD.Infrastructure.ComponentModel
{
    public record InterfaceType(
        ComponentCodePath ParentComponentCodePath,
        Type ClrType
    )
        : AbstractType (ParentComponentCodePath, ClrType);
}
