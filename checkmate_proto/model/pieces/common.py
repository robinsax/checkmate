from typing import List, Callable

from ...serialization import ISerializable, ISerializedInput

from ..color import Color
from ..bases import IPiece, IPosition

def find_moves_walks(piece: IPiece, directions: List[Callable[[IPosition], IPosition]]):
    moves = list()

    for direction in directions:
        cur_pos = direction(piece.position)
        while True:
            allowed = (
                cur_pos.is_valid() and (
                    not cur_pos.is_occupied() or cur_pos.has_enemy(piece)
                )
            )
            if not allowed:
                break

            moves.append(cur_pos)

            if cur_pos.has_enemy(piece):
                break

            cur_pos = direction(cur_pos)

    return moves

def filter_moves_valid(piece: IPiece, moves: List[IPosition]):
    return list(
        move for move in moves if move.is_valid() and (
            not move.is_occupied() or move.has_enemy(piece)
        )
    )

class BasePiece(IPiece, ISerializable):
    _prev_positions: List[IPosition]
    _position: IPosition
    _color: Color

    def __init__(self, *, position: IPosition = None, color: Color = None) -> None:
        super().__init__()

        self._position = position
        self._color = color
        self._prev_positions = list()

    @property
    def name(self) -> str:
        if not self.names:
            raise NotImplementedError('Piece.names')

        return self.names()[int(self.color is Color.black)]

    def names(self) -> List[str]:
        raise NotImplementedError('IPiece.names()')

    @property
    def color(self) -> Color:
        return self._color

    @property
    def position(self) -> IPosition:
        return self._position

    def set_position(self, new_pos: IPosition) -> None:
        self._prev_positions.append(self.position)
        self._position = new_pos

    def is_enemy(self, other: IPiece) -> bool:
        return self.color is not other.color

    def serialize(self) -> str:
        return list((str(self), self._prev_positions))

    def deserialize(self, data: ISerializedInput) -> None:
        from ..board import PositionImpl
        name, rest = data.list_lookup(0).scan_str('|'.join(self.names()))

        self._color = Color.white if name == self.names()[0] else Color.black
        if len(rest.as_str()):
            self._position = rest.as_instance(PositionImpl)
        self._prev_positions = data.list_lookup(1).list_unwrap().as_instance(PositionImpl)

    def __str__(self):
        return ''.join((
            self.name, str(self.position) if self.position else str()
        ))
