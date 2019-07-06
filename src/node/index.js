'use strict'

module.exports.overrideNode = overrideNode
function overrideNode () {
  require('./extensions.js').overrideNode()
}
overrideNode()
