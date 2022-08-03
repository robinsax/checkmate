import { addRoute } from '../router';
import { ConnectView } from './connect';
import { GameView } from './game';

export const applyRoutes = () => {
    addRoute('/connect', ConnectView);
    addRoute('/game', GameView);
};
