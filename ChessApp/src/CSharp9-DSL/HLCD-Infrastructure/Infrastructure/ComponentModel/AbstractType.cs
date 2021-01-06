using System;
using HLCD.Infrastructure.Attributes;

namespace HLCD.Infrastructure.ComponentModel
{
    public abstract record AbstractType(
        ComponentCodePath ParentComponentCodePath,
        Type ClrType
    );
}
