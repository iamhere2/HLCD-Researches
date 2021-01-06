using System;
using HLCD.Infrastructure.Attributes;

namespace ChessApp
{
    [Value]
    class UserError : Exception
    {
        public UserError(string message) : base(message)
        {
        }
    }

    class TurnError : UserError
    {
        public TurnError(string message) : base(message)
        {
        }
    }

    class RuleViolationError : TurnError
    {
        public RuleViolationError(RuleViolation rv) : base(rv.Description)
        {
        }
    }
}
