use super::color::Color;
use super::position::Position;
use super::piece_type::PieceType;
use super::piece::Piece;
use super::move_repr::Move;
use super::state::State;
use super::moves_builder::{MovesBuilder, MoveCondition};

type PieceMoveFinder = fn (
    builder: &mut MovesBuilder, state: &State, position: &Position, color: Color
) -> Vec<Move>;

fn pawn_moves(builder: &mut MovesBuilder, state: &State, position: &Position, color: Color) -> Vec<Move> {
    let forward_one = &position.forward(color);

    if builder.push_if_valid_and(forward_one, MoveCondition::IsNotAttack).is_some() {
        builder.push_if_valid_and(
            &forward_one.forward(color),
            MoveCondition::NoHistory | MoveCondition::IsNotAttack
        );
    }

    builder.push_if_valid_and(&forward_one.left(), MoveCondition::IsAttack);
    builder.push_if_valid_and(&forward_one.right(), MoveCondition::IsAttack);

    let moves = builder.build();
    if moves.iter().find(|&m| m.to.is_end_rank()).is_none() {
        return moves;
    }

    let mut updated_moves: Vec<Move> = Vec::with_capacity(moves.len() + 3);
    for mut check_move in moves {
        if !check_move.to.is_end_rank() {
            updated_moves.push(check_move);
            continue;
        }

        updated_moves.extend(check_move.as_promotions(&[
            PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight
        ]));
    }

    updated_moves
}

fn bishop_moves(builder: &mut MovesBuilder, state: &State, position: &Position, color: Color) -> Vec<Move> {
    builder.push_while_valid(|pos| pos.up().left());
    builder.push_while_valid(|pos| pos.up().right());
    builder.push_while_valid(|pos| pos.down().left());
    builder.push_while_valid(|pos| pos.down().right());
    
    builder.build()
}

fn rook_moves(builder: &mut MovesBuilder, state: &State, position: &Position, color: Color) -> Vec<Move> {
    builder.push_while_valid(|pos| pos.up());
    builder.push_while_valid(|pos| pos.down());
    builder.push_while_valid(|pos| pos.left());
    builder.push_while_valid(|pos| pos.right());
    
    builder.build()
}

fn knight_moves(builder: &mut MovesBuilder, state: &State, position: &Position, color: Color) -> Vec<Move> {
    builder.push_if_valid(&position.up().up().left());
    builder.push_if_valid(&position.up().up().right());
    builder.push_if_valid(&position.down().down().left());
    builder.push_if_valid(&position.down().down().right());
    builder.push_if_valid(&position.left().left().up());
    builder.push_if_valid(&position.right().right().up());
    builder.push_if_valid(&position.left().left().down());
    builder.push_if_valid(&position.right().right().down());

    builder.build()
}

fn queen_moves(builder: &mut MovesBuilder, state: &State, position: &Position, color: Color) -> Vec<Move> {
    builder.push_while_valid(|pos| pos.up().left());
    builder.push_while_valid(|pos| pos.up().right());
    builder.push_while_valid(|pos| pos.down().left());
    builder.push_while_valid(|pos| pos.down().right());
    builder.push_while_valid(|pos| pos.up());
    builder.push_while_valid(|pos| pos.down());
    builder.push_while_valid(|pos| pos.left());
    builder.push_while_valid(|pos| pos.right());
    
    builder.build()
}

fn king_moves(builder: &mut MovesBuilder, state: &State, position: &Position, color: Color) -> Vec<Move> {
    builder.push_if_valid(&position.up().left());
    builder.push_if_valid(&position.up().right());
    builder.push_if_valid(&position.down().left());
    builder.push_if_valid(&position.down().right());
    builder.push_if_valid(&position.up());
    builder.push_if_valid(&position.down());
    builder.push_if_valid(&position.left());
    builder.push_if_valid(&position.right());

    let mut check_castle = |direction_fn: fn (position: Position) -> Position| {
        let mut cur_position = direction_fn(position.clone());

        while cur_position.is_valid() {
            if let Some(piece_here) = &state.board[&cur_position] {
                let valid = piece_here.color == color &&
                    piece_here.piece_type == PieceType::Rook &&
                    !state.has_piece_at_moved(&cur_position);
                if !valid {
                    return;
                }
                
                let castle_move = builder.push_unchecked(
                    &direction_fn(direction_fn(position.clone())),
                    &(cur_position.clone(), direction_fn(position.clone()))
                );
            }

            cur_position = direction_fn(cur_position);
        }
    };
    check_castle(|pos| pos.right());
    check_castle(|pos| pos.left());

    builder.build()
}

pub fn compute_moves_for(state: &State, position: &Position, piece: &Piece, lookahead: bool) -> Vec<Move> {
    let finder: PieceMoveFinder = match piece.piece_type {
        PieceType::Pawn => pawn_moves,
        PieceType::Bishop => bishop_moves,
        PieceType::Rook => rook_moves,
        PieceType::Knight => knight_moves,
        PieceType::Queen => queen_moves,
        PieceType::King => king_moves
    };

    let mut builder = MovesBuilder::new(state, position, lookahead);

    finder(&mut builder, state, position, piece.color)
}
