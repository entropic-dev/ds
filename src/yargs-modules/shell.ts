import mkCmd from "../utils/cmd-handler";
import { Argv } from "yargs";

const Shell = (module.exports = {
  command: "shell",
  aliases: ["sh"],
  describe: "Launch a ds shell or execute a script",
  builder(yargs: Argv) {
    return yargs
      .help()
      .alias("help", "h")
      .options(Shell.options);
  },
  options: Object.assign({}, require("../common-opts"), {
    _: { default: [] },
    nodeArg: {
      alias: ["n", "node-arg"],
      describe: "Arguments to pass down directly to node",
      type: "array",
    },
    prefix: {
      alias: "C",
      describe: "Directory to execute package management operations in.",
      type: "string",
    },
    also: {
      hidden: true,
    },
    dev: {
      hidden: true,
    },
    development: {
      hidden: true,
    },
    only: {
      hidden: true,
    },
    production: {
      type: "boolean",
      describe: "Limit downloads to production dependencies, skipping devDependencies.",
    },
  }),
  handler: mkCmd((...args) => require("../commands/shell.js")(...args)),
});
