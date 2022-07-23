from enum import Enum
from typing import TYPE_CHECKING

from ..serialization import ISerializable, SerializedInput

if TYPE_CHECKING:
    from .piece import Piece

class Color(Enum):
    white = 'white'
    black = 'black'

class Position(ISerializable):
    rank: str
    file: str

    def __init__(self, rank: str = None, file: str = None) -> None:
        super().__init__()

        self.rank = rank
        self.file = file

    def up(self) -> 'Position':
        return Position(chr(ord(self.rank) + 1), self.file)

    def down(self) -> 'Position':
        return Position(chr(ord(self.rank) - 1), self.file)
    
    def right(self) -> 'Position':
        return Position(self.rank, chr(ord(self.file) + 1))

    def left(self) -> 'Position':
        return Position(self.rank, chr(ord(self.file) - 1))

    def serialize(self) -> str:
        return str(self)

    def deserialize(self, data: SerializedInput) -> None:
        self.file, self.rank = data.as_str('[a-z][0-9]')

    def __eq__(self, other: 'Position') -> bool:
        return str(self) == str(other)

    def __str__(self) -> str:
        return ''.join((self.file, self.rank))

class Move(ISerializable):
    piece: 'Piece'
    taken: 'Piece'
    from_position: Position
    to_position: Position

    def __init__(
        self, piece: 'Piece' = None, taken: 'Piece' = None,
        from_position: 'Position' = None, to_position: 'Position' = None
    ) -> None:
        super().__init__()

        self.piece = piece
        self.taken = taken
        self.from_position = from_position
        self.to_position = to_position

    def serialize(self) -> dict:
        from .piece_types import AbstractPiece

        dict_repr = {
            'piece': AbstractPiece(self.piece),
            'from': str(self.from_position),
            'to': str(self.to_position)
        }
        if self.taken:
            dict_repr['taken'] = AbstractPiece(self.taken)

        return dict_repr

    def deserialize(self, data: SerializedInput) -> None:
        from .piece_types import AbstractPiece

        self.piece = data.dict_lookup('piece').as_instance(AbstractPiece).piece
        taken_repr = data.dict_lookup('taken', allow_empty=True)
        if taken_repr:
            self.taken = taken_repr.as_instance(AbstractPiece).piece

        self.from_position = data.dict_lookup('from').as_instance(Position)
        self.to_position = data.dict_lookup('to').as_instance(Position)

    def __str__(self) -> str:
        return '%s %s -> %s%s'%(
            str(self.piece), str(self.from_position), str(self.to_position),
            ' takes %s'%str(self.taken) if self.taken else str()
        )
