import * as PIXI from 'pixi.js'

const SIZE = 2

window.addEventListener('load', () => {
  const app = new PIXI.Application({
    width: window.innerWidth,
    height: window.innerHeight,
    resizeTo: window,
    resolution: window.devicePixelRatio,
  })
  document.body.appendChild(app.view)

  const COLORS = [0xffffff, 0xff0000, 0x00ff00, 0x0000ff]
  const TEXTURE = app.renderer.generateTexture(
    new PIXI.Graphics()
      .beginFill(0xffffff)
      .drawRect(0, 0, SIZE, SIZE)
      .endFill(),
  )

  let container
  let paused = true
  const pressedKeys = new Set()

  const ws = new WebSocket('ws://localhost:8080')
  ws.addEventListener('message', (message) => {
    const { topic, payload } = JSON.parse(message.data)

    if (topic === 'metadata') {
      container = new PIXI.ParticleContainer(payload.numEntities, {
        rotation: false,
        uvs: false,
        tint: false,
      })

      container.x = window.innerWidth / 2
      container.y = window.innerHeight / 2

      for (let i = 0; i <= payload.numEntities; i++) {
        const color = Math.floor(i / (payload.numEntities / COLORS.length))

        let sprite = new PIXI.Sprite(TEXTURE)
        sprite.tint = COLORS[color]

        container.addChild(sprite)
      }

      app.stage.addChild(container)

      window.addEventListener('wheel', (e) => {
        container.scale.x += -e.deltaY / 5000
        container.scale.y += -e.deltaY / 5000

        if (container.scale.x < 0.1) container.scale.x = 0.1
        if (container.scale.y < 0.1) container.scale.y = 0.1
      })

      window.addEventListener('keydown', (e) => {
        pressedKeys.add(e.keyCode)

        if (e.keyCode === 32) {
          paused = !paused
        }
      })

      window.addEventListener('keyup', (e) => {
        pressedKeys.delete(e.keyCode)
      })
    } else if (topic === 'update') {
      const data = payload.split(',')

      for (let i = 0; i < data.length; i++) {
        const [x, y] = data[i].split(' ')

        container.children[i].position.x = x * SIZE
        container.children[i].position.y = y * SIZE
      }
    }
  })

  setInterval(() => {
    if (!paused) {
      ws.send(JSON.stringify({ topic: 'update_request' }))
    }
  }, 1000 / 30)

  setInterval(() => {
    if (pressedKeys.has(87)) {
      container.y += 10
    } else if (pressedKeys.has(83)) {
      container.y -= 10
    }

    if (pressedKeys.has(65)) {
      container.x += 10
    } else if (pressedKeys.has(68)) {
      container.x -= 10
    }
  }, 1000 / 60)
})
