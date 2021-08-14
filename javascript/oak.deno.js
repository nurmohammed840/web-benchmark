import { Application } from "https://deno.land/x/oak/mod.ts"

console.log('Oak.js (Deno), http://127.0.0.1:3000')

const app = new Application()

app.use(ctx => {
    ctx.response.body = "Hello World!"
})

app.listen({ port: 3000 })