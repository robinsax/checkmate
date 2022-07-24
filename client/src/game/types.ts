export type Color = 'white' | 'black';

export interface Board {
    ranks: string[];
    files: string[];
    pieces: [string, Piece][];
    history: Move[];
}

export interface Piece {
    id: string;
    name: string;
}

export interface Move {
    piece: Piece;
    taken: Piece | null;
    from: string;
    to: string;
    castle_other: Move | null;
    promotion_to: Piece | null;
    promotion_from: Piece | null;
}

export interface GameResult {
    winner: Color;
    condition: string;
}

export interface GameState {
    board: Board;
    active_player: [Color, string];
    legal_moves: Move[];
    result: any;
}

export interface Game {
    state(): GameState;
    takeTurn(move: Move): Promise<boolean>;
}
