import './connect.scss';

import { Component, createSignal } from 'solid-js';

import { Input } from '../common';
import { navigate } from '../router';

export const ConnectView: Component = () => {
    const [host, setHost] = createSignal(
        window.localStorage.getItem('last_host') || 'http://localhost'
    );
    const [port, setPort] = createSignal(
        window.localStorage.getItem('last_port') || '8000'
    );

    const updateHost = (host: string) => {
        window.localStorage.setItem('last_host', host);

        setHost(host);
    };
    const updatePort = (port: string) => {
        window.localStorage.setItem('last_port', port);

        setPort(port);
    };

    return (
        <div class="connect-view">
            <div class="title">connect</div>
            <div class="fields">
                <Input value={ host() } onChange={ value => updateHost(value) }/>
                <Input value={ port() } onChange={ value => updatePort(value) }/>
            </div>
            <div class="actions">
                <button
                    onClick={ () => navigate('/game', { host: host(), port: port() }) }
                >
                    connect
                </button>
            </div>
        </div>
    )
};
