import './router.scss';

import { Component, onMount, createSignal } from 'solid-js';
import { Dynamic } from 'solid-js/web';
import queryString from 'query-string';

export type QueryMap = { [key: string]: string };

export type RouteComponent = Component | Component<{ query: QueryMap }>;

interface CreatedRouter {
    Router: Component,
    addRoute: (route: string, componentFn: RouteComponent) => void,
    navigate: (route: string, query?: QueryMap) => void
}

const createRouter = (): CreatedRouter => {
    const history: [string, QueryMap][] = [];
    const historyHooks: ((route: string, query: QueryMap) => void)[] = [];

    const routes: { [route: string]: RouteComponent } = {};

    const navigate = (route: string, query: QueryMap = {}) => {
        history.push([route, query]);
        
        let pushRoute = route;
        if (Object.keys(query).length) {
            pushRoute += '?' + queryString.stringify(query);
        }
        window.history.pushState(query, '', pushRoute);

        for (const hook of historyHooks) hook(route, query);
    };
 
    const addRoute = (route: string, componentFn: RouteComponent) => (
        routes[route] = componentFn
    );

    const notFound = (newRoute: string): Component => (
        () => (
            <div class="not-found-view">{ newRoute } not found</div>
        )
    );

    const Router: Component = () => {
        const [route, setRoute] = createSignal<[string, QueryMap]>(['', {}]);

        onMount(() => {
            historyHooks.push((newRoute, newQuery) => setRoute([newRoute, newQuery]));
        });

        return (
            <Dynamic
                component={routes[route()[0]] || notFound(route()[0]) }
                //  @ts-ignore
                query={ route()[1] }
            />
        );
    };

    return { Router, addRoute, navigate };
};

export const { Router, addRoute, navigate } = createRouter();
