'use strict'

const figgyPudding = require('figgy-pudding')

const config = figgyPudding({})
module.exports = getConfigs
function getConfigs (argv) {
  return config(argv)
}
