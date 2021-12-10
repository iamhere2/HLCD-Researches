namespace HLCD.ChessAppExampleWithDSL.ChessApp_Internals
{
    // TODO: It's a sign of bad design - there is a dependency cycle, should be eliminated
    class LateBindingPlayerAdapter : IPlayer
    {
        public bool IsBound => _implementation is not null;

        private IPlayer Implementation => _implementation ?? throw new InvalidOperationException("Adapter is not bound");
        private IPlayer? _implementation;

        public void Bind(IPlayer implementation)
        {
            CheckArg.NotNull(implementation);

            if (IsBound)
                throw new InvalidOperationException("Adapter is already bound");

            _implementation = implementation;
        }

        public Task<Turn> NextTurn(BoardState state)
            => Implementation.NextTurn(state);
    }
}
