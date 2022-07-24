from typing import Tuple
from enum import Enum

from ..serialization import ISerializable, SerializedInput
from ..model import Color, Board, Move, Piece, Position

class ResultCondition(Enum):
    checkmate = 'checkmate'
    stalemate = 'stalemate'
    insufficient_materiel = 'insufficient_materiel'

class GameResult(ISerializable):
    winner: Color
    condition: ResultCondition

    def __init__(
        self, winner: Color = None, condition: ResultCondition = None
    ) -> None:
        self.winner = winner
        self.condition = condition

    def serialize(self) -> dict:
        return {
            'winner': self.winner,
            'condition': self.condition
        }

    def deserialize(self, data: SerializedInput) -> None:
        self.winner = data.as_instance(Color)
        self.condition = data.as_instance(ResultCondition)

class IPlayer(ISerializable):

    def name(self) -> str:
        raise NotImplementedError()

    def is_human(self) -> bool:
        raise NotImplementedError()

    def take_turn(self, game: 'IGame') -> None:
        raise NotImplementedError()

class IGame(ISerializable):
    
    @property
    def active_player(self) -> IPlayer:
        raise NotImplementedError()

    @property
    def result(self) -> GameResult:
        raise NotImplementedError()

    @property
    def board(self) -> Board:
        raise NotImplementedError()

    @property
    def turn(self) -> Color:
        raise NotImplementedError()

    def set_players(self, players: Tuple[IPlayer, IPlayer]) -> None:
        raise NotImplementedError()

    def as_move(self, piece: Piece, position: Position) -> Move:
        raise NotImplementedError()

    def make_move(self, move: Move) -> None:
        raise NotImplementedError()

    def start(self) -> None:
        raise NotImplementedError()
