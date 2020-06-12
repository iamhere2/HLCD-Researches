using System;

namespace ChessApp
{
    class TurnError : UserError
    {
        public TurnError(string message) : base(message)
        {
        }
    }

    class UserError : Exception
    {
        public UserError(string message) : base(message)
        {
        }
    }

    class RuleViolationError : UserError
    {
        public RuleViolationError(RuleViolation rv) : base(rv.Description)
        {
        }
    }
}
