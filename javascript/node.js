const PORT = 3000, HOST = "127.0.0.1";

require('http')
    .createServer((_, res) => {
        res.end('Hello World');
    })
    .listen(PORT, HOST, () => {
        console.log(`Node, http://${HOST}:${PORT}/`);
    });