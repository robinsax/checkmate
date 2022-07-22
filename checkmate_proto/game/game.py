from ..serialization import ISerializedInput
from ..model import Color, IPosition, IPiece, IBoardState

from .bases import IGame
from .exceptions import GameStateError

class GameImpl(IGame):
    _board: IBoardState
    _turn: Color
    _turn_count: int

    @classmethod
    def create(
        cls, *, 
        board: IBoardState = None, turn: Color = None
    ):
        return cls(board=board, turn=turn)

    def __init__(
        self, *,
        board: IBoardState = None, turn: Color = None
    ) -> None:
        super().__init__()

        self._board = board
        self._turn = turn

        self._turn_count = 0

    @property
    def turn(self) -> Color:
        return self._turn

    @property
    def board(self) -> IBoardState:
        return self._board

    def take_turn(self, piece: IPiece, new_position: IPosition) -> None:
        if piece.color is not self.turn:
            raise GameStateError('not your turn')

        if new_position not in piece.moves(self.board):
            raise GameStateError('invalid move')

        self._board.move(piece, new_position)

        self._turn = Color.black if self.turn is Color.white else Color.white
        self._turn_count += 1

    def serialize(self) -> dict:
        return {
            'board': self.board,
            'turn': self.turn,
            'turn_count': self._turn_count
        }
    
    def deserialize(self, data: ISerializedInput) -> None:
        from ..model import BoardStateImpl

        self._board = data.dict_lookup('board').as_instance(BoardStateImpl)
        self._turn = Color.white if data.dict_lookup('turn').as_str() == 'white' else Color.black
        self._turn_count = data.dict_lookup('turn_count').as_int()

    def __str__(self):
        def taken_pieces_str(color: Color):
            taken = list(
                piece.name for piece in self.board.pieces if
                piece.color is color and not piece.position
            )
            if not len(taken):
                return str()

            return ''.join((
                '\n○' if color is Color.white else '\n●', ''.join(taken)
            ))

        return 'turn %s, %s to move%s%s\n\n%s'%(
            self._turn_count + 1, self.turn.value,
            taken_pieces_str(Color.white), taken_pieces_str(Color.black),
            str(self.board)
        )
