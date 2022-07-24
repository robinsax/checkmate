from typing import TYPE_CHECKING, List, Callable, Tuple

from ..serialization import ISerializable, SerializedInput

from .piece import IPieceType
from .primitive import Position, Color, Move

if TYPE_CHECKING:
    from .board import Board
    from .piece import Piece

def find_walk_positions(
    board: 'Board', piece: 'Piece', position: Position,
    directions: List[Callable[[Position], Position]]
):
    moves = list()

    for direction in directions:
        cur_position = direction(position)
        while True:
            if not board.is_valid(cur_position):
                break

            attacked_piece = board[cur_position]
            if attacked_piece and attacked_piece.color is piece.color:
                break

            moves.append(cur_position)

            if attacked_piece:
                break

            cur_position = direction(cur_position)

    return moves

def valid_positions(
    board: 'Board', piece: 'Piece', positions: List[Position],
):
    valid = list()

    for position in positions:
        if not board.is_valid(position):
            continue

        attacked_piece = board[position]
        if attacked_piece and attacked_piece.color is piece.color:
            continue
    
        valid.append(position)

    return valid

class PawnType(IPieceType):

    def names(self) -> List[str]:
        return ('♙', '♟')

    def moves(self, board: 'Board', piece: 'Piece', position: Position) -> List[Position]:
        from .piece import Piece

        moves: List[Position] = list()

        def forward(pos: Position) -> Position:
            if piece.color is Color.white:
                return pos.up()
            return pos.down()

        walk_position = forward(position)
        if board.is_valid(walk_position) and not board[walk_position]:
            moves.append(walk_position)

            if not board.has_piece_moved(piece):
                walk_position = forward(walk_position)
                if board.is_valid(walk_position) and not board[walk_position]:
                    moves.append(walk_position)

        def check_attack(attack_position: Position):
            if not board.is_valid(attack_position):
                return
            
            attacked_piece = board[attack_position]
            if attacked_piece and attacked_piece.color is not piece.color:
                moves.append(attack_position)

        check_attack(forward(position).right())
        check_attack(forward(position).left())

        full_moves = list()
        for move in moves:
            is_last_rank = move.rank in (board.ranks[-1], board.ranks[0])
            if not is_last_rank:
                full_moves.append(move)
                continue

            options = (QueenType(), RookType(), KnightType(), BishopType())
            name_index = self.names().index(piece.name)
            for option in options:
                promo_piece = Piece(option, (Color.white, Color.black)[name_index])
                promo_piece.id = piece.id

                full_moves.append(Move(
                    piece=piece,
                    from_position=position,
                    to_position=move,
                    promotion_from=piece,
                    promotion_to=promo_piece
                ))

        return full_moves

class BishopType(IPieceType):

    def names(self) -> List[str]:
        return ('♗', '♝')
 
    def moves(self, board: 'Board', piece: 'Piece', position: Position) -> List[Position]:
        return find_walk_positions(board, piece, position, (
            lambda pos: pos.up().left(),
            lambda pos: pos.up().right(),
            lambda pos: pos.down().left(),
            lambda pos: pos.down().right()
        ))

class RookType(IPieceType):

    def names(self) -> List[str]:
        return ('♖', '♜')
 
    def moves(self, board: 'Board', piece: 'Piece', position: Position) -> List[Position]:
        return find_walk_positions(board, piece, position, (
            lambda pos: pos.up(),
            lambda pos: pos.down(),
            lambda pos: pos.left(),
            lambda pos: pos.right()
        ))

class KnightType(IPieceType):

    def names(self) -> List[str]:
        return ('♘', '♞')
 
    def moves(self, board: 'Board', piece: 'Piece', position: Position) -> List[Position]:
        return valid_positions(board, piece, (
            position.up().up().left(),
            position.up().up().right(),
            position.down().down().left(),
            position.down().down().right(),
            position.left().left().up(),
            position.right().right().up(),
            position.left().left().down(),
            position.right().right().down()
        ))

class QueenType(IPieceType):

    def names(self) -> List[str]:
        return ('♕', '♛')
 
    def moves(self, board: 'Board', piece: 'Piece', position: Position) -> List[Position]:
        return find_walk_positions(board, piece, position, (
            lambda pos: pos.up(),
            lambda pos: pos.down(),
            lambda pos: pos.left(),
            lambda pos: pos.right(),
            lambda pos: pos.up().left(),
            lambda pos: pos.up().right(),
            lambda pos: pos.down().left(),
            lambda pos: pos.down().right()
        ))

class KingType(IPieceType):

    def names(self) -> List[str]:
        return ('♔', '♚')
 
    def moves(self, board: 'Board', piece: 'Piece', position: Position) -> List[Position]:
        moves = valid_positions(board, piece, (
            position.up(),
            position.up().left(),
            position.up().right(),
            position.left(),
            position.right(),
            position.down(),
            position.down().left(),
            position.down().right()
        ))

        def find_castle(direction: Callable[[Position], Position]) -> Move:
            def find_rook_info() -> Tuple['Piece', Position]:
                search = direction(position)
                while board.is_valid(search):
                    check_piece = board[search]
                    if not check_piece:
                        search = direction(search)
                        continue

                    if check_piece.name in ('♖', '♜') and not board.has_piece_moved(check_piece):
                        return (check_piece, search)

                    return None
                return None
            
            rook_info = find_rook_info()
            if not rook_info:
                return None
            
            return Move(
                piece=piece,
                from_position=position,
                to_position=direction(direction(position)),
                castle_other=Move(
                    piece=rook_info[0],
                    from_position=rook_info[1],
                    to_position=direction(position)
                )
            )

        if not board.has_piece_moved(piece):
            left_castle = find_castle(lambda pos: pos.left())
            if left_castle:
                moves.append(left_castle)
            right_castle = find_castle(lambda pos: pos.right())
            if right_castle:
                moves.append(right_castle)

        return moves

_piece_types: List[IPieceType] = (
    PawnType(), BishopType(), RookType(), KnightType(), QueenType(), KingType()
)

def piece_type_from_name(type_name: str):
    type_name = '%sType'%type_name.title()
    
    for type in _piece_types:
        if type.__class__.__name__ == type_name:
            return type

    raise ValueError('invalid piece type name')

class AbstractPiece(ISerializable):
    piece: 'Piece'

    def __init__(self, piece: 'Piece' = None) -> None:
        super().__init__()

        self.piece = piece

    def serialize(self) -> List[str]:
        return (self.piece.name, self.piece.id)

    def deserialize(self, data: SerializedInput) -> None:
        from .piece import Piece

        piece_repr = data.list_lookup(0).as_str('[♔-♟]')
        piece_id = data.list_lookup(1).as_str('[a-z0-9]+')

        for piece_type in _piece_types:
            piece_type_names = piece_type.names()
            if piece_repr not in piece_type_names:
                continue 
            
            self.piece = Piece(
                piece_type,
                (Color.white, Color.black)[piece_type_names.index(piece_repr)]
            )
            self.piece.id = piece_id
            break
