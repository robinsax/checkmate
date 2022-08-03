export type Color = 'white' | 'black';

export type PieceType = 'pawn' | 'knight' | 'rook' | 'bishop' | 'queen' | 'king';

export const RANKS = '12345678'.split('');
export const FILES = 'abcdefgh'.split('');

export interface Piece {
    type: PieceType;
    color: Color;
}

export interface Move {
    piece: Piece;
    from: string;
    to: string;
    taken: Piece | null;
    promotion: PieceType | null;
    castle: {
        from: string,
        to: string
    } | null;
}

export interface Result {
    winner: Color;
    condition: string;
}

export interface State {
    board: [string, Piece][],
    active: string;
    moves: Move[];
    history: Move[];
    result: Result | null;
}

export interface Game {
    state(): State;
    takeTurn(move: Move): Promise<boolean>;
    restart(): Promise<boolean>;
}
