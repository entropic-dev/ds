import { PureComponent } from 'react';
import { ProxyFiggyPudding } from 'figgy-pudding';
import { Argv } from 'yargs';
export declare class App extends PureComponent {
    render(): JSX.Element;
}
export declare function renderApp(argv: Argv, opts: ProxyFiggyPudding<{
    cache: {};
    nodeArg: {};
    production: {};
}, {}>): void;
