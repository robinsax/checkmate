use super::color::Color;
use super::position::Position;
use super::piece::Piece;
use super::piece_type::PieceType;
use super::move_repr::{Move, CastleMoves};
use super::state::State;
use super::moves_builder::MovesBuilder;

type MoveComputeFn = fn (builder: &mut MovesBuilder, state: &State, position: &Position, piece: &Piece);

fn pawn_moves(builder: &mut MovesBuilder, state: &State, position: &Position, piece: &Piece) {
    let end_rank = if piece.color == Color::White { 7 } else { 0 };
    let promotions = &[PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight];

    let mut push_move = |to: &Position, iff_take: bool| -> bool {
        let taken = &state.board[to];
        if iff_take != taken.is_some() {
            return false;
        }

        //  TODO: Yikes!
        let new_move = Move::new(
            position.clone(), to.clone(), piece.clone(),
            taken.clone(), None, None
        );
        if to.rank == end_rank {
            for promo in promotions {
                builder.push(new_move.with_promotion(*promo));
            }
        }
        else {
            builder.push(new_move);
        }

        true
    };

    //  Walk forward.
    let forward_one = &position.forward(piece.color);
    if push_move(forward_one, false) { 
        let initial_rank = if piece.color == Color::White { 1 } else { 6 };

        //  Push from initial rank.
        if position.rank == initial_rank {
            push_move(&forward_one.forward(piece.color), false);
        }
    }

    //  Normal attacks.
    let attack_right = &forward_one.right();
    let attack_left = &forward_one.left();
    push_move(attack_right, true);
    push_move(attack_left, true);

    //  En-passant.
    if let Some(target) = state.get_en_passant_position() {
        if attack_right == target {
            let taken = &state.board[&position.right()];
            builder.push(Move::new(
                position.clone(), attack_right.clone(), piece.clone(),
                taken.clone(), None, None
            ));
        }
        if attack_left == target {
            let taken = &state.board[&position.left()];
            builder.push(Move::new(
                position.clone(), attack_left.clone(), piece.clone(),
                taken.clone(), None, None
            ));
        }
    }
}

fn bishop_moves(builder: &mut MovesBuilder, _state: &State, _position: &Position, _piece: &Piece) {
    builder.push_while_valid(|pos| pos.up().left());
    builder.push_while_valid(|pos| pos.up().right());
    builder.push_while_valid(|pos| pos.down().left());
    builder.push_while_valid(|pos| pos.down().right());
}

fn rook_moves(builder: &mut MovesBuilder, _state: &State, _position: &Position, _piece: &Piece) {
    builder.push_while_valid(|pos| pos.up());
    builder.push_while_valid(|pos| pos.down());
    builder.push_while_valid(|pos| pos.left());
    builder.push_while_valid(|pos| pos.right());
}

fn knight_moves(builder: &mut MovesBuilder, _state: &State, position: &Position, _piece: &Piece) {
    builder.push_if_valid(&position.up().up().left());
    builder.push_if_valid(&position.up().up().right());
    builder.push_if_valid(&position.down().down().left());
    builder.push_if_valid(&position.down().down().right());
    builder.push_if_valid(&position.left().left().up());
    builder.push_if_valid(&position.right().right().up());
    builder.push_if_valid(&position.left().left().down());
    builder.push_if_valid(&position.right().right().down());
}

fn queen_moves(builder: &mut MovesBuilder, _state: &State, _position: &Position, _piece: &Piece) {
    builder.push_while_valid(|pos| pos.up().left());
    builder.push_while_valid(|pos| pos.up().right());
    builder.push_while_valid(|pos| pos.down().left());
    builder.push_while_valid(|pos| pos.down().right());
    builder.push_while_valid(|pos| pos.up());
    builder.push_while_valid(|pos| pos.down());
    builder.push_while_valid(|pos| pos.left());
    builder.push_while_valid(|pos| pos.right());
}

fn king_moves(builder: &mut MovesBuilder, state: &State, position: &Position, piece: &Piece) {
    builder.push_if_valid(&position.up().left());
    builder.push_if_valid(&position.up().right());
    builder.push_if_valid(&position.down().left());
    builder.push_if_valid(&position.down().right());
    builder.push_if_valid(&position.up());
    builder.push_if_valid(&position.down());
    builder.push_if_valid(&position.left());
    builder.push_if_valid(&position.right());

    let mut push_castle = |direction_fn: fn (position: Position) -> Position, castle_check: CastleMoves| {
        let mut cur_position = direction_fn(position.clone());

        while cur_position.is_valid() {
            if let Some(piece_here) = &state.board[&cur_position] {
                let valid = piece_here.color == piece.color &&
                    piece_here.piece_type == PieceType::Rook &&
                    state.get_allowed_castles(piece_here.color).contains(castle_check);
                if !valid {
                    return;
                }
                
                builder.push(Move::new(
                    position.clone(), direction_fn(direction_fn(position.clone())), piece.clone(), None,
                    None, Some((cur_position.clone(), direction_fn(position.clone())))
                ));
            }

            cur_position = direction_fn(cur_position);
        }
    };
    push_castle(|pos| pos.right(), CastleMoves::KingSide);
    push_castle(|pos| pos.left(), CastleMoves::QueenSide);
}

pub fn compute_moves_for(state: &State, position: &Position) -> Vec<Move> {
    let piece = state.board[position].as_ref().unwrap();

    let compute: MoveComputeFn = match piece.piece_type {
        PieceType::Pawn => pawn_moves,
        PieceType::Bishop => bishop_moves,
        PieceType::Rook => rook_moves,
        PieceType::Knight => knight_moves,
        PieceType::Queen => queen_moves,
        PieceType::King => king_moves
    };

    let mut builder = MovesBuilder::new(state, position);
    compute(&mut builder, state, position, &piece);

    builder.build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pawn_moves() {
        let state = State::default();

        let moves = compute_moves_for(&state, &Position::new(1, 4));

        assert_eq!(moves.len(), 2);
        assert_eq!(moves[0].to, Position::new(2, 4));
        assert_eq!(moves[1].to, Position::new(3, 4));
    }
    
    #[test]
    fn test_knight_moves() {
        let state = State::default();

        let moves = compute_moves_for(&state, &Position::new(0, 1));

        assert_eq!(moves.len(), 2);
        assert_eq!(moves[0].to, Position::new(2, 0));
        assert_eq!(moves[1].to, Position::new(2, 2));
    }
}
