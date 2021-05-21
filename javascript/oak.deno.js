import { Application } from "https://deno.land/x/oak@v7.5.0/mod.ts"

const app = new Application()

app.use(ctx => {
    ctx.response.body = "Hello World!"
})

console.log('Oak.js (Deno), http://127.0.0.1:3000')

await app.listen({ port: 3000 })
