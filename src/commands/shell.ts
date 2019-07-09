import figgyPudding, { ProxyFiggyPudding } from "figgy-pudding";
import { Argv } from "yargs";

const ShellOpts = figgyPudding({
  cache: {},
  nodeArg: {},
  production: {},
});

async function shell(argv: Argv, opts: ProxyFiggyPudding<{ cache: {}; nodeArg: {}; production: {} }, {}>) {
  opts = ShellOpts(opts);
  console.error("called `ds sh`:", argv);
}

export default shell;
