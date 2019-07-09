import figgyPudding from "figgy-pudding";
import { Argv } from "yargs";

const config = figgyPudding({});

function getConfigs(argv: Argv) {
  return config(argv);
}

export default getConfigs;
