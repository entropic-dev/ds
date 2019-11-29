import { ProxyFiggyPudding } from 'figgy-pudding';
import { Argv } from 'yargs';
declare function shell(argv: Argv, opts: ProxyFiggyPudding<{
    cache: {};
    nodeArg: {};
    production: {};
}, {}>): Promise<void>;
export default shell;
