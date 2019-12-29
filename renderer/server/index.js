const LineByLineReader = require('line-by-line')
const path = require('path')
const ws = require('ws')

const FILE = '../generator/.output/1000-900.txt'

const [numEntities, numIterations] = FILE.split('/')
  .pop()
  .split('.')
  .shift()
  .split('-')
  .map(Number)

new ws.Server({
  port: 8080,
}).on('connection', (client) => {
  client.send(
    JSON.stringify({
      topic: 'metadata',
      payload: {
        numEntities,
        numIterations,
      },
    }),
  )

  // positions buffer containers a list of positions for each iteration
  // as soon as it reaches numEntities, it is sent to client and cleared
  let step = ''

  // read file line by line, appending to positions buffer
  // pause if buffer is numEntities, resume once it's cleared
  const reader = new LineByLineReader(path.resolve(process.cwd(), FILE))
  reader
    .on('error', console.error)
    .on('end', console.error)
    .on('line', (line) => {
      if (step == '') {
        step = line
      } else {
        reader.pause()

        const interval = setInterval(() => {
          if (step == '') {
            step = line
            clearInterval(interval)
            reader.resume()
          }
        }, 0)
      }
    })

  // on each message from client, send positions buffer
  // if it's full
  client.on('message', (message) => {
    const { topic } = JSON.parse(message)

    if (topic === 'update_request') {
      if (step === '') {
        console.log('throttle!')

        return
      }

      client.send(
        JSON.stringify({
          topic: 'update',
          payload: step,
        }),
      )

      step = ''
    }
  })
})
