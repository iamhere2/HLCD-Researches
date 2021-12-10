namespace HLCD.ChessAppExampleWithDSL.Errors
{
    [Value("CA")]
    public class UserErrorException : Exception
    {
        public UserErrorException(string message) : base(message)
        {
        }

        public UserErrorException()
        {
        }

        public UserErrorException(string message, Exception innerException) : base(message, innerException)
        {
        }
    }

    public class TurnError : UserErrorException
    {
        public TurnError(string message) : base(message)
        {
        }

        public TurnError()
        {
        }

        public TurnError(string message, Exception innerException) : base(message, innerException)
        {
        }
    }

    public sealed class RuleViolationError : TurnError
    {
        public RuleViolationError(RuleViolation rv) : this(rv.Description)
        {
        }

        public RuleViolationError()
        {
        }

        public RuleViolationError(string message) : base(message)
        {
        }

        public RuleViolationError(string message, Exception innerException) : base(message, innerException)
        {
        }
    }
}
