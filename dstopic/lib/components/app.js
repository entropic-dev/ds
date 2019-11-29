import React, { PureComponent } from 'react';
import { render, Color } from 'ink';
export class App extends PureComponent {
    render() {
        return React.createElement(Color, { green: true }, "Hello World");
    }
}
export function renderApp(argv, opts) {
    console.log(argv);
    console.log(opts);
    render(React.createElement(App, null));
}
