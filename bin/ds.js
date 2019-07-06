#!/usr/bin/env node

require('../src/node/index.js')
const reqmain = require.main
require = require('esm')(module, { //eslint-disable-line
  force: true,
  wasm: true,
  cjs: {
    interop: true,
    namedExports: true,
    extensions: true,
    vars: true
  }
})

const CMDS = new Set([
  'shell',
])

const ALIASES = new Map([
  ['prep', 'prepare'],
  ['sh', 'shell']
])

if (reqmain === module) {
  main(process.argv)
}

module.exports = main
function main (argv) {
  return runCommandWithYargs(argv)
}

function runCommandWithYargs (argv) {
  let config = require('yargs')
    .demandCommand(1, 'Subcommand is required')
    .recommendCommands()
    .help()
    .alias('help', 'h')
    .alias('version', 'v')
    .completion()
  if (ALIASES.has(argv[2])) {
    config = config.command(require(`../src/yargs-modules/${ALIASES.get(argv[2])}.js`))
  } else if (CMDS.has(argv[2])) {
    config = config.command(require(`../src/yargs-modules/${argv[2]}.js`))
  } else {
    for (const mod of CMDS.values()) {
      config = config.command(require(`../src/yargs-modules/${mod}.js`))
    }
  }
  config = config.argv
}

