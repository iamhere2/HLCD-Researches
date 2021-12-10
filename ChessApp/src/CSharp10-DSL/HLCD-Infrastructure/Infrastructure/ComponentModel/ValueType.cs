using System;

namespace HLCD.Infrastructure.ComponentModel
{
    public record ValueType(
        ComponentCodePath ParentComponentCodePath,
        Type ClrType
    )
        : AbstractType(ParentComponentCodePath, ClrType);
}
