var Clay = require('@rebble/clay')
var clayConfig = require('./config')
var clay = new Clay(clayConfig)

const refresh = () => {
  const config = localStorage.getItem('config')
  if (config) {
    Pebble.sendAppMessage(JSON.parse(config))
  }
}

refresh()

Pebble.addEventListener('webviewclosed', function (e) {
  if (e && !e.response) {
    return
  }

  let config = Object.fromEntries(
    Object.entries(JSON.parse(e.response)).map((e) => [e[0], e[1].value])
  )

  localStorage.setItem('config', JSON.stringify(config))
  console.log('sending config', config)
  Pebble.sendAppMessage(config)
  console.log('sent config')
})
