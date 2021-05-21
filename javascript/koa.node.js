const Application = require('koa')
const app = new Application()

app.use(ctx => {
    ctx.body = 'Hello Koa'
})

app.listen(3000)

console.log('Koa.js (Node), http://127.0.0.1:3000')