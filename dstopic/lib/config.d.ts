import figgyPudding from 'figgy-pudding';
import { Argv } from 'yargs';
declare function getConfigs(argv: Argv): figgyPudding.ProxyFiggyPudding<{}, {}>;
export default getConfigs;
