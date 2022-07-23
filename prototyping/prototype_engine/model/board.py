from typing import List, Tuple, Mapping, Union

from ..serialization import ISerializable, SerializedInput

from .piece import Piece
from .piece_types import AbstractPiece
from .primitive import Color, Move, Position

class Board(ISerializable):
    ranks: List[str]
    files: List[str]
    state: Mapping[str, Piece]
    move_history: List[Move]

    @classmethod
    def create_with(
        cls, ranks: List[str], files: List[str],
        pieces: Tuple[Position, Piece]
    ):
        state = dict()
        for position, piece in pieces:
            piece.id = str(position)
            state[str(position)] = piece

        return cls(ranks, files, state, list())

    def __init__(
        self, ranks: List[str] = None, files: List[str] = None,
        state: Mapping[str, Piece] = None, move_history: List[Move] = None
    ) -> None:
        super().__init__()

        self.ranks = ranks
        self.files = files
        self.state = state
        self.move_history = move_history

    def is_valid(self, position: Position) -> bool:
        return position.rank in self.ranks and position.file in self.files

    def has_piece_moved(self, piece: Piece) -> bool:
        for move in self.move_history:
            if move.piece == piece:
                return True
        return False

    def legal_moves(self, color: Color) -> List[Move]:
        moves = list()

        for rank in self.ranks:
            for file in self.files:
                position = Position(rank, file)
                piece = self[position]
                if piece and piece.color is color:
                    moves.extend(piece.moves(self, position))

        return moves

    def apply_move(self, move: Move) -> None:
        del self.state[str(move.from_position)]
        self.state[str(move.to_position)] = move.piece

        self.move_history.append(move)

    def serialize(self) -> dict:
        from .piece_types import AbstractPiece

        pieces: List[Tuple[Position, AbstractPiece]] = list()

        for rank in self.ranks:
            for file in self.files:
                position = Position(rank, file)
                piece = self[position]
                if piece:
                    pieces.append((position, AbstractPiece(piece)))

        return {
            'ranks': ''.join(self.ranks),
            'files': ''.join(self.files),
            'pieces': pieces,
            'history': self.move_history
        }

    def deserialize(self, data: SerializedInput) -> None:
        from .piece_types import AbstractPiece

        class AbstractPieceEntry(ISerializable):
            piece: Piece
            position: Position

            def deserialize(self, data: SerializedInput) -> None:
                self.position = data.list_lookup(0).as_instance(Position)
                self.piece = data.list_lookup(1).as_instance(AbstractPiece).piece

        self.ranks = list(data.dict_lookup('ranks').as_str('[0-9]+'))
        self.files = list(data.dict_lookup('files').as_str('[a-z]+'))

        piece_entries = data.dict_lookup('pieces').list_unwrap().as_instance(AbstractPieceEntry)
        self.state = dict()
        for piece_entry in piece_entries:
            self.state[str(piece_entry.position)] = piece_entry.piece

        self.move_history = data.dict_lookup('history').list_unwrap().as_instance(Move)

    def view_matrix(self, with_legend: bool = False) -> List[List[str]]:
        ranks = list()

        for i, rank in enumerate(reversed(self.ranks)):
            rank_row = list()

            if with_legend:
                rank_row.append(rank)

            for j, file in enumerate(self.files):
                piece = self[(rank, file)]

                if piece:
                    rank_row.append(str(piece))
                    continue

                rank_row.append('x' if (i + j) % 2 else ' ')
            ranks.append(rank_row)

        if with_legend:
            ranks.append((' ', *self.files))

        return ranks

    def __getitem__(self, position: Union[Tuple[str, str], Position, str]) -> Piece:
        return self.state.get(
            position if isinstance(position, str) else (
                str(position) if isinstance(position, Position) else ''.join(reversed(position))
            )
        )

    def __str__(self) -> str:
        return '\n'.join(
            ' '.join(rank) for rank in self.view_matrix(with_legend=True)
        )
