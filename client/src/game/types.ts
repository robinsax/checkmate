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
}

export interface Game {
    legalMoves(): Move[];
    activePlayer(): [Color, string];
    board(): Board;
    takeTurn(move: Move): Promise<boolean>;
}
