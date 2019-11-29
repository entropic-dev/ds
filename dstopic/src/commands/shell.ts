import figgyPudding, { ProxyFiggyPudding } from 'figgy-pudding';
import { renderApp } from '../components/app';
import { Argv } from 'yargs';

const ShellOpts = figgyPudding({
  cache: {},
  nodeArg: {},
  production: {}
});

async function shell(
  argv: Argv,
  opts: ProxyFiggyPudding<{ cache: {}; nodeArg: {}; production: {} }, {}>
) {
  opts = ShellOpts(opts);
  renderApp(argv, opts);
}

export default shell;
