import { Argv } from 'yargs';
import npmConfig from '../config';

export default function mkCmdHandler(cb: (argv: Argv, npmConfig: any) => void) {
  return async function(argv: Argv) {
    return cb(argv, npmConfig(argv));
  };
}
