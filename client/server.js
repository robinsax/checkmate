const fs = require('fs');
const path = require('path');
const express = require('express');
const { createServer: createViteServer } = require('vite');

const createServer = async () => {
    const app = express();

    const vite = await createViteServer({
        server: { middlewareMode: true },
        appType: 'custom'
    });

    app.use(vite.middlewares);

    app.use('*', async (req, resp) => {
        resp.sendFile(path.join(__dirname, 'pub/index.html'));
    });

    app.listen(9000);
};

createServer();
