from ..serialization import ISerializable
from ..model import Color, IBoardState, IPosition, IPiece, IPosition

class IGame(ISerializable):
    
    @property
    def turn(self) -> Color:
        raise NotImplementedError('IGame.turn')

    @property
    def board(self) -> IBoardState:
        raise NotImplementedError('IGame.board')

    def take_turn(self, piece: IPiece, new_position: IPosition) -> None:
        raise NotImplementedError('IGame.take_turn(piece, new_position)')
