use readonly::{ make as make_readonly };

use serde::{ Serialize };

use super::model::{State, Move, Position, EndResult, EndCondition};
pub use super::model::Color;

#[make_readonly]
#[derive(Serialize, Clone)]
pub struct MoveFormat {
    pub r#move: String,
    pub piece: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taken: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub castle: Option<String>
}

#[make_readonly]
#[derive(Serialize, Clone)]
pub struct EndResultFormat {
    pub winner: Option<String>,
    pub condition: String
}

#[make_readonly]
#[derive(Serialize, Clone)]
pub struct StateFormat {
    pub board: Vec<String>,
    pub active: String,
    pub moves: Vec<MoveFormat>,
    pub history: Vec<MoveFormat>,
    pub end: Option<EndResultFormat>
}

impl EndResultFormat {
    pub fn from_model(end_option: Option<EndResult>) -> Option<Self> {
        let end = end_option?;

        Some(Self{
            winner: match &end.winner {
                Some(color) => Some(format!("{}", color)),
                None => None
            },
            condition: match end.condition {
                EndCondition::Checkmate => "checkmate",
                EndCondition::Stalemate => "stalemate",
                EndCondition::InsufficientMateriel => "insufficient_materiel",
                EndCondition::Surrender => "surrender"
            }.to_string()
        })
    }
}

impl StateFormat {
    pub fn from_model(state: &State) -> Self {
        let white_pieces = state.board.piece_positions_for(&Color::White);
        let black_pieces = state.board.piece_positions_for(&Color::Black);

        let mut board_format: Vec<String> = Vec::with_capacity(white_pieces.len() + black_pieces.len());

        let mut add_board_members = |positions: &Vec<Position>| {
            for position in positions {
                let piece = state.board[position].as_ref().unwrap();

                board_format.push(
                    format!("{}{}{}", piece.color, piece.piece_type.to_char(), position)
                );
            }
        };
        add_board_members(white_pieces);
        add_board_members(black_pieces);

        let convert_moves = |moves: &Vec<Move>| -> Vec<MoveFormat> {
            let mut moves_format: Vec<MoveFormat> = Vec::with_capacity(moves.len());

            for move_item in moves {
                moves_format.push(
                    MoveFormat{
                        r#move: format!("{}{}", move_item.from, move_item.to),
                        piece: format!("{}{}", move_item.piece.color, move_item.piece.piece_type.to_char()),
                        promo: match &move_item.promotion {
                            Some(promo) => Some(format!("{}", promo.to_char())),
                            None => None
                        },
                        taken: match &move_item.taken {
                            Some(taken) => Some(
                                format!("{}{}", taken.color, taken.piece_type.to_char())
                            ),
                            None => None
                        },
                        castle: match &move_item.castle {
                            Some((castle_from, castle_to)) => Some(
                                format!("{}{}", castle_from, castle_to)
                            ),
                            None => None
                        }
                    }
                );
            }

            moves_format
        };

        Self{
            board: board_format,
            active: state.active_player.to_string(),
            moves: convert_moves(&state.legal_moves()),
            history: convert_moves(&state.history),
            end: EndResultFormat::from_model(state.check_result())
        }
    }
}
