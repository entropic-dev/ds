import React, { PureComponent } from 'react';
import { render, Color } from 'ink';

import { ProxyFiggyPudding } from 'figgy-pudding';
import { Argv } from 'yargs';

export class App extends PureComponent {
  render() {
    return <Color green>Hello World</Color>;
  }
}

export function renderApp(
  argv: Argv,
  opts: ProxyFiggyPudding<{ cache: {}; nodeArg: {}; production: {} }, {}>
) {
  console.log(argv);
  console.log(opts);
  render(<App />);
}
