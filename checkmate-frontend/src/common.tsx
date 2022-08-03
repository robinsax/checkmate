import { Component } from 'solid-js';

export interface ImportProps {
    value: string,
    onChange: (newValue: string) => void
}

export const Input: Component<ImportProps> = ({ value, onChange }: ImportProps) => (
    <input
        value={ value }
        onInput={ event => onChange((event.target as HTMLInputElement).value) }
    />
);
