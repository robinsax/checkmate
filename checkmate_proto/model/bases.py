from typing import List, Tuple

from ..serialization import ISerializable

from .color import Color

class IPiece(ISerializable):

    @property
    def name(self) -> str:
        raise NotImplementedError('IPiece.name')

    @property
    def position(self) -> 'IPosition':
        raise NotImplementedError('IPiece.position')

    def set_position(self, position: 'IPosition') -> None:
        raise NotImplementedError('IPiece.set_position(position)')

    @property
    def color(self) -> Color:
        raise NotImplementedError('IPiece.color')

    def is_enemy(self, other: 'IPiece') -> bool:
        raise NotImplementedError('IPiece.is_enemy(other)')

    def moves(self, board: 'IBoardState') -> List['IPosition']:
        raise NotImplementedError('IPiece.moves(board)')

class IPosition(ISerializable):

    @property
    def rank(self) -> str:
        raise NotImplementedError('IPosition.rank')

    @property
    def file(self) -> str:
        raise NotImplementedError('IPosition.file')

    def is_valid(self) -> bool:
        raise NotImplementedError('IPosition.is_valid()')

    def is_occupied(self) -> bool:
        raise NotImplementedError('IPosition.is_occupied()')

    def has_enemy(self, piece: IPiece) -> bool:
        raise NotImplementedError('IPosition.has_enemy(piece)')

    @property
    def piece(self) -> IPiece:
        raise NotImplementedError('IPosition.piece')

    def up(self) -> 'IPosition':
        raise NotImplementedError('IPosition.up()')
        
    def down(self) -> 'IPosition':
        raise NotImplementedError('IPosition.down()')
        
    def left(self) -> 'IPosition':
        raise NotImplementedError('IPosition.left()')
        
    def right(self) -> 'IPosition':
        raise NotImplementedError('IPosition.right()')

class IBoardState(ISerializable):
    
    @property
    def ranks(self) -> List[str]:
        raise NotImplementedError('IBoardState.ranks')

    @property
    def files(self) -> List[str]:
        raise NotImplementedError('IBoardState.files')

    @property
    def pieces(self) -> List[IPiece]:
        raise NotImplementedError('IBoardState.pieces')

    def __getitem__(self, position: Tuple[str, str]) -> IPosition:
        raise NotImplementedError('IBoardState[position]')

    def move(self, piece: IPiece, new_position: IPosition) -> None:
        raise NotImplementedError('IBoardState.move(piece, new_position)')
