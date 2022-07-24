import { Board, Color, Piece, Game, Move } from '../types';

type RemotePrototypeStatePiece = [string, string];

interface RemotePrototypeStateMove {
    piece: RemotePrototypeStatePiece;
    taken?: RemotePrototypeStatePiece;
    from: string;
    to: string;
}

interface RemotePrototypeState {
    legal_moves: RemotePrototypeStateMove[];
    board: {
        ranks: string;
        files: string;
        pieces: [string, RemotePrototypeStatePiece][];
        history: RemotePrototypeStateMove[];
    };
    turn: Color;
    players: {
        white: string;
        black: string;
    };
}

export class RemotePrototypeHostDriver implements Game {
    private url: string;
    private currentLegalMoves: Move[] | null;
    private currentBoard: Board | null;
    private currentTurn: Color | null;
    private players: { [color: string]: string } | null;

    static async connect(host: string, port: number): Promise<Game> {
        const instance = new RemotePrototypeHostDriver([host, port].join(':'));
        await instance.initialize();
        return instance;
    }
    
    private constructor(url: string) {
        this.url = url;

        this.currentLegalMoves = null;
        this.currentBoard = null;
        this.currentTurn = null;
        this.players = null;
    }

    private async initialize(): Promise<void> {
        const resp = await fetch(this.url + '/game');

        this.hydrateStateFrom(await resp.json() as RemotePrototypeState);
    }

    legalMoves(): Move[] {
        return this.currentLegalMoves as Move[];
    }

    board(): Board {
        return this.currentBoard as Board;
    }

    activePlayer(): [Color, string] {
        if (!this.currentTurn || !this.players) return ['white', 'noone'];

        return [this.currentTurn, this.players[this.currentTurn]];
    }

    async takeTurn(move: Move): Promise<boolean> {
        const convertedMove: RemotePrototypeStateMove = {
            piece: [move.piece.name, move.piece.id],
            from: move.from,
            to: move.to
        };
        if (move.taken) {
            convertedMove.taken = [move.taken.name, move.taken.id];
        }

        const resp = await fetch(this.url + '/game', {
            method: 'POST',
            body: JSON.stringify(convertedMove),
            headers: {
                'Content-Type': 'application/json'
            }
        });
        
        if (resp.status != 200) return false;

        this.hydrateStateFrom(await resp.json() as RemotePrototypeState);
        return true;
    }

    private hydrateStateFrom(state: RemotePrototypeState) {
        const parsePiece = (input: RemotePrototypeStatePiece): Piece => ({
            id: input[1],
            name: input[0]
        });

        const parseMove = (input: RemotePrototypeStateMove): Move => ({
            piece: parsePiece(input.piece),
            taken: input.taken ? parsePiece(input.taken) : null,
            from: input.from,
            to: input.to
        });
        
        this.currentBoard = {
            ranks: state.board.ranks.split(''),
            files: state.board.files.split(''),
            pieces: state.board.pieces.map(input => [input[0], parsePiece(input[1])]),
            history: state.board.history.map(parseMove)
        };
        this.currentTurn = state.turn;
        this.currentLegalMoves = state.legal_moves.map(parseMove);
        this.players = state.players;
    }
}