use super::color::Color;
use super::position::Position;
use super::piece_type::PieceType;
use super::piece::Piece;
use super::board::Board;

struct BoardPlacement {
    piece: Piece,
    position: Position
}

/// `BoardBuilder` allows progressive construction of a board state.
///
/// Usually used as an exposure of [`StateBuilder`].
pub struct BoardBuilder {
    placements: Vec<BoardPlacement>
}

impl BoardBuilder {
    pub fn new() -> Self {
        Self{
            placements: Vec::new()
        }
    }

    pub fn place_piece(&mut self, piece: Piece, position: Position) {
        self.placements.push(BoardPlacement{piece, position});
    }

    //  TODO: Remove clones.
    pub fn build(self) -> Board {
        let mut positions: [[Option<Piece>; 8]; 8] = Default::default();
        let mut piece_positions: [Vec<Position>; 2] = Default::default();

        for placement in &self.placements {
            positions[placement.position.rank][placement.position.file] = Some(placement.piece.clone());
            
            let color_idx: usize = placement.piece.color.into();
            piece_positions[color_idx].push(placement.position.clone());
        }

        Board::new(positions, piece_positions)
    }
}

impl Default for Board {
    /// Construct a canonical initial chess board state.
    fn default() -> Self {
        let mut builder = BoardBuilder::new();

        let mut create_rank = |color: Color, rank_idx: usize, pieces: &Vec<PieceType>| {
            for (file_idx, piece_type) in pieces.iter().enumerate() {
                builder.place_piece(Piece::new(color, *piece_type), Position::new(rank_idx, file_idx));
            }
        };

        let back_rank = &vec![
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook
        ];
        let pawn_rank = &vec![
            PieceType::Pawn,
            PieceType::Pawn,
            PieceType::Pawn,
            PieceType::Pawn,
            PieceType::Pawn,
            PieceType::Pawn,
            PieceType::Pawn,
            PieceType::Pawn
        ];
        create_rank(Color::Black, 7, back_rank);
        create_rank(Color::Black, 6, pawn_rank);
        create_rank(Color::White, 1, pawn_rank);
        create_rank(Color::White, 0, back_rank);

        builder.build()
    }
}
