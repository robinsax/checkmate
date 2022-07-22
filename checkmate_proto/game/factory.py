from typing import Type

from ..model import Color, IPiece, BoardStateImpl, PositionImpl, Pawn, Rook, Bishop, \
    Knight, King, Queen

from .bases import IGame
from .game import GameImpl

def create_default_game() -> IGame:
    ranks = ('1', '2', '3', '4', '5', '6', '7', '8')
    files = ('a', 'b', 'c', 'd', 'e', 'f', 'g', 'h')

    def piece(Typ: Type[IPiece], rank: str, file: str, color: Color):
        return Typ(position=PositionImpl(rank=rank, file=file), color=color)

    def pawn_rank(rank: str, color: Color):
        return (piece(Pawn, rank, file, color) for file in files)

    def back_rank(rank: str, color: Color):
        return (
            piece(Rook, rank, 'a', color),
            piece(Knight, rank, 'b', color),
            piece(Bishop, rank, 'c', color),
            piece(Queen, rank, 'd', color),
            piece(King, rank, 'e', color),
            piece(Bishop, rank, 'f', color),
            piece(Knight, rank, 'g', color),
            piece(Rook, rank, 'h', color)
        )

    return GameImpl.create(
        board=BoardStateImpl.create(
            ranks=ranks,
            files=files,
            pieces=(
                *back_rank('8', Color.black),
                *pawn_rank('7', Color.black),
                *pawn_rank('2', Color.white),
                *back_rank('1', Color.white)
            )
        ),
        turn=Color.white
    )
