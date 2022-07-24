import { Board, Color, Piece, Game, Move, GameState } from '../types';

type RemotePrototypeStatePiece = [string, string];

interface RemotePrototypeStateMove {
    piece: RemotePrototypeStatePiece;
    taken?: RemotePrototypeStatePiece;
    from: string;
    to: string;
    castle_other?: RemotePrototypeStateMove;
    promotion_to?: RemotePrototypeStatePiece;
    promotion_from?: RemotePrototypeStatePiece;
}

interface RemotePrototypeState {
    state: {
        legal_moves: RemotePrototypeStateMove[];
        turn: Color;
        board: {
            ranks: string;
            files: string;
            pieces: [string, RemotePrototypeStatePiece][];
            history: RemotePrototypeStateMove[];
        };
        result: {
            winner: Color;
            condition: string;
        };
    };
    players: {
        white: string;
        black: string;
    };
}

export class RemotePrototypeHostDriver implements Game {
    private url: string;
    private currentState: GameState | null;
    private players: { [color: string]: string } | null;

    static async connect(host: string, port: number): Promise<Game> {
        const instance = new RemotePrototypeHostDriver([host, port].join(':'));
        await instance.initialize();
        return instance;
    }
    
    private constructor(url: string) {
        this.url = url;

        this.currentState = null;
        this.players = null;
    }

    private async initialize(): Promise<void> {
        const resp = await fetch(this.url + '/game');

        this.hydrateStateFrom(await resp.json() as RemotePrototypeState);
    }

    state(): GameState {
        return this.currentState as GameState;
    }

    async takeTurn(move: Move): Promise<boolean> {
        const convertMove = (internal: Move): RemotePrototypeStateMove => {
            const converted: RemotePrototypeStateMove = {
                piece: [internal.piece.name, internal.piece.id],
                from: internal.from,
                to: internal.to
            };
            if (internal.taken) {
                converted.taken = [internal.taken.name, internal.taken.id];
            }
            if (internal.castle_other) {
                converted.castle_other = convertMove(internal.castle_other);
            }
            if (internal.promotion_to && internal.promotion_from) {
                converted.promotion_to = [internal.promotion_to.name, internal.promotion_to.id];
                converted.promotion_from = [internal.promotion_from.name, internal.promotion_from.id];
            }

            return converted;
        };

        const resp = await fetch(this.url + '/game', {
            method: 'POST',
            body: JSON.stringify(convertMove(move)),
            headers: {
                'Content-Type': 'application/json'
            }
        });
        
        if (resp.status != 200) return false;

        this.hydrateStateFrom(await resp.json() as RemotePrototypeState);
        return true;
    }

    private hydrateStateFrom(input: RemotePrototypeState) {
        const parsePiece = (input: RemotePrototypeStatePiece): Piece => ({
            id: input[1],
            name: input[0]
        });

        const parseMove = (input: RemotePrototypeStateMove): Move => ({
            piece: parsePiece(input.piece),
            taken: input.taken ? parsePiece(input.taken) : null,
            from: input.from,
            to: input.to,
            castle_other: input.castle_other ? parseMove(input.castle_other) : null,
            promotion_to: input.promotion_to ? parsePiece(input.promotion_to) : null,
            promotion_from: input.promotion_from ? parsePiece(input.promotion_from) : null
        });
       
        const { state: { legal_moves, board, turn, result }, players } = input;

        this.players = players;
        this.currentState = {
            board: {
                ranks: board.ranks.split(''),
                files: board.files.split(''),
                pieces: board.pieces.map(info => [info[0], parsePiece(info[1])]),
                history: board.history.map(parseMove)
            },
            active_player: [turn, players[turn]],
            legal_moves: legal_moves.map(parseMove),
            result
        };
    }
}