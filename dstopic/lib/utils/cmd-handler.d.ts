import { Argv } from 'yargs';
export default function mkCmdHandler(cb: (argv: Argv, npmConfig: any) => void): (argv: Argv<{}>) => Promise<void>;
