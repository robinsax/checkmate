from typing import TYPE_CHECKING, List

from .primitive import Color, Position, Move

if TYPE_CHECKING:
    from .board import Board

class IPieceType:

    def names(self):
        raise NotImplementedError('IPieceType.names()')

    def moves(self, board: 'Board', piece: 'Piece', position: Position) -> Position:
        raise NotImplementedError('IPieceType.moves(board)')

class Piece:
    id: str
    name: str
    color: Color
    _type: IPieceType

    @classmethod
    def of_type(cls, type_name: str, color: Color) -> 'Piece':
        from .piece_types import piece_type_from_name

        return cls(piece_type_from_name(type_name), color)

    def __init__(self, type: IPieceType, color: Color) -> None:
        super().__init__()

        self._type = type
        self.color = color
        self.name = self._type.names()[int(self.color is not Color.white)]
        self.id = None

    def moves(self, board: 'Board', position: Position) -> List[Move]:
        move_positions = self._type.moves(board, self, position)

        moves = list()
        for new_position in move_positions:
            moves.append(Move(self, board[new_position], position, new_position))

        return moves

    def __eq__(self, other: 'Piece') -> bool:
        return self.id == other.id

    def __str__(self) -> str:
        return self.name
