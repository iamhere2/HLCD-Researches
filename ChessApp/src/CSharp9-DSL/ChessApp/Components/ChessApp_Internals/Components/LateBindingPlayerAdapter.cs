using System;
using System.Threading.Tasks;
using HLCD.ChessAppExampleWithDSL.Data;

namespace HLCD.ChessAppExampleWithDSL.ChessApp_Internals
{
    class LateBindingPlayerAdapter : IPlayer
    {
        public bool IsBound => _implementation != null;

        private IPlayer Implementation => _implementation ?? throw new InvalidOperationException("Adapter is not bound");
        private IPlayer? _implementation;

        public void Bind(IPlayer implementation)
        {
            if (implementation is null)
                throw new ArgumentNullException(nameof(implementation));

            if (IsBound)
                throw new InvalidOperationException("Adapter is already bound");

            _implementation = implementation;
        }

        public TaskCompletionSource<Turn> NextTurn(BoardState state)
            => Implementation.NextTurn(state);
    }
}
