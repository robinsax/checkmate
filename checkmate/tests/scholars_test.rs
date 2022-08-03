use checkmate::model::{Color, Piece, PieceType, EndResult, EndCondition};
use checkmate::formats::{ToPGN, ToAlg, ToFEN, ToState, ToMove, ToPosition};

#[test]
fn scholars_mate_game() {
    let mut state = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_fen().to_state().unwrap();

    assert_eq!(state.active_color, Color::White);
    assert_eq!(state.board[&"e2".to_alg().to_position().unwrap()].as_ref().unwrap(), &Piece::new(Color::White, PieceType::Pawn));

    state = state.next_for_move(&"e4".to_alg().to_move(&state).unwrap());

    assert_eq!(state.active_color, Color::Black);
    assert_eq!(state.to_pgn().to_string(), "1. e4 ...".to_string());

    state = state.next_for_move(&"e5".to_alg().to_move(&state).unwrap());

    assert_eq!(state.to_pgn().to_string(), "1. e4 e5".to_string());
    assert_eq!(state.to_fen().to_string(), "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 2".to_string());

    state = state.next_for_move(&"Qh5".to_alg().to_move(&state).unwrap());

    assert_eq!(state.to_pgn().to_string(), "1. e4 e5 2. Qh5 ...".to_string());
    
    state = state.next_for_move(&"Nf6".to_alg().to_move(&state).unwrap());

    state = state.next_for_move(&"Bc4".to_alg().to_move(&state).unwrap());

    state = state.next_for_move(&"Nc6".to_alg().to_move(&state).unwrap());

    state = state.next_for_move(&"Qxf7".to_alg().to_move(&state).unwrap());

    assert_eq!(state.to_fen().to_string(), "r1bqkb1r/pppp1Qpp/2n2n2/4p3/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 4".to_string());
    assert_eq!(state.to_pgn().to_string(), "1. e4 e5 2. Qh5 Nf6 3. Bc4 Nc6 4. Qxf7 ...".to_string());

    assert_eq!(state.is_check_against(Color::Black), true);
    assert_eq!(state.get_legal_moves().len(), 0);
    assert_eq!(state.check_result(), Some(EndResult::win(Color::White, EndCondition::Checkmate)));
}
