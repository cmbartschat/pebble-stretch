var Clay = require('pebble-clay')
var clayConfig = require('./config')
var clay = new Clay(clayConfig)

const sendState = () => {
  Pebble.sendAppMessage(JSON.parse(localStorage.getItem('config')))
}

Pebble.addEventListener('webviewclosed', function (e) {
  if (e && !e.response) {
    return
  }

  console.log(e.response)
  localStorage.setItem('config', e.response)
  refresh()
})

refresh()
