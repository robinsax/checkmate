import './index.scss';

import { Component, For, createSignal, onMount } from 'solid-js';
import { render } from 'solid-js/web';
import queryString from 'query-string';

import { Router, navigate, QueryMap } from './router';
import { applyRoutes } from './views';

applyRoutes();

const Root: Component = () => {
    const [theme, setTheme] = createSignal('light');

    const updateTheme = (newTheme: string) => {
        document.body.setAttribute('data-theme', newTheme);
        window.localStorage.setItem('theme', newTheme);

        setTheme(newTheme);
    };

    onMount(() => {
        updateTheme(window.localStorage.getItem('theme') || 'light');
    });
    
    return (
        <div class="root">
            <div class="root-settings">
                theme
                <select
                    onChange={ event => updateTheme((event.target as HTMLSelectElement).value) }
                >
                    <For each={ ['light', 'dark'] }>
                        { item => (
                            <option selected={ theme() == item } value={ item }>
                                { item }
                            </option>
                        ) }
                    </For>
                </select>
            </div>
            <Router/>
        </div>
    )
};

render(() => <Root />, document.getElementById('mount') as HTMLElement);

navigate(window.location.pathname, queryString.parse(window.location.search) as QueryMap);
