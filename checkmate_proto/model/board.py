from typing import List, Mapping, Tuple

from ..serialization import ISerializedInput

from .bases import IPosition, IBoardState, IPiece, IPosition

class PositionImpl(IPosition):
    _rank: str
    _file: str
    _board: 'BoardStateImpl'

    def __init__(
        self, *,
        rank: str = None, file: str = None,
        board: 'BoardStateImpl' = None
    ) -> None:
        super().__init__()

        self._rank = rank
        self._file = file
        self._board = board

    @property
    def rank(self) -> str:
        return self._rank    

    @property
    def file(self) -> str:
        return self._file

    def is_valid(self) -> bool:
        return str(self) in self._board._piece_lookup

    @property
    def piece(self) -> IPiece:
        if not self.is_valid():
            return None

        return self._board._piece_lookup[str(self)]

    def is_occupied(self) -> bool:
        return self.piece is not None

    def has_enemy(self, piece: IPiece) -> bool:
        return self.piece and self.piece.is_enemy(piece)

    def up(self) -> IPosition:
        return self._board[(chr(ord(self.rank) + 1), self.file)]

    def down(self) -> IPosition:
        return self._board[(chr(ord(self.rank) - 1), self.file)]
    
    def right(self) -> IPosition:
        return self._board[(self.rank, chr(ord(self.file) + 1))]

    def left(self) -> IPosition:
        return self._board[(self.rank, chr(ord(self.file) - 1))]

    def serialize(self) -> str:
        return str(self)

    def deserialize(self, data: ISerializedInput) -> None:
        self._file, self._rank = data.as_str('[a-z][0-9]')

    def __str__(self):
        return ''.join((self.file, self.rank))

    def __eq__(self, other: IPosition) -> bool:
        return self.rank == other.rank and self.file == other.file

class BoardStateImpl(IBoardState):
    _ranks: List[str]
    _files: List[str]
    _peices: List[IPiece]
    _piece_lookup: Mapping[str, IPiece]

    @classmethod
    def create(
        cls, *,
        ranks: List[str] = None,
        files: List[str] = None,
        pieces: List[IPiece] = None
    ) -> IBoardState:
        return cls(
            ranks=ranks,
            files=files,
            pieces=pieces
        )

    def __init__(
        self, *,
        ranks: List[str] = None,
        files: List[str] = None,
        pieces: List[IPiece] = None
    ) -> None:
        super().__init__()

        self._ranks = ranks or list()
        self._files = files or list()
        self._pieces = pieces or list()
        self._piece_lookup = dict()

        self._hydrate()

    def _hydrate(self) -> None:
        for rank in self._ranks:
            for file in self._files:
                self._piece_lookup[str(self[(rank, file)])] = None

        for piece in self._pieces:
            if piece.position:
                self._piece_lookup[str(piece.position)] = piece

                piece.position._board = self

    @property
    def ranks(self) -> List[str]:
        return self._ranks

    @property
    def files(self) -> List[str]:
        return self._files

    @property
    def pieces(self) -> List[IPiece]:
        return self._pieces

    def move(self, piece: IPiece, new_position: IPosition) -> None:
        taken = new_position.piece
        if taken and taken.is_enemy(piece):
            taken.set_position(None)

        piece.set_position(new_position)
        self._hydrate()

    def serialize(self) -> dict:
        return {
            'ranks': self._ranks,
            'files': self._files,
            'pieces': self._pieces
        }

    def deserialize(self, data: ISerializedInput) -> None:
        from .pieces import Pawn, Rook, Bishop, Knight, Queen, King

        self._ranks = data.dict_lookup('ranks').list_unwrap().as_str()
        self._files = data.dict_lookup('files').list_unwrap().as_str()
        self._pieces = data.dict_lookup('pieces').list_unwrap()\
                .as_instance(Pawn, Rook, Bishop, Knight, Queen, King)

        self._hydrate()

    def __getitem__(self, position: Tuple[str, str]) -> IBoardState:
        return PositionImpl(rank=position[0], file=position[1], board=self)

    def __str__(self):
        rows = list()
        for i, rank in enumerate(reversed(self._ranks)):
            row = list((rank,))

            for j, file in enumerate(self._files):
                piece = self[(rank, file)].piece

                if piece:
                    row.append(piece.name)
                    continue
                
                if (i + j) % 2:
                    row.append('x')
                    continue

                row.append(' ')

            rows.append(' '.join(row))        
        rows.append(' '.join((' ', *self._files)))

        return '\n'.join(rows)
