import { serve } from "https://deno.land/std/http/server.ts";

const s = serve({ port: 3000 });

console.log("Deno, http://localhost:3000/");

for await (const req of s) {
    req.respond({ body: "Hello World" });
}