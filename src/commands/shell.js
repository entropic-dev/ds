'use strict'

const figgyPudding = require('figgy-pudding')

const ShellOpts = figgyPudding({
  cache: {},
  nodeArg: {},
  production: {}
})

module.exports = shell
async function shell (argv, opts) {
  opts = ShellOpts(opts)
  console.error('called `ds sh`:', argv)
}
