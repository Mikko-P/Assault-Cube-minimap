const net = require('net');
const WebSocket = require('ws');

const PORT = 3000;
const HOST = 'localhost';

const server = net.createServer((socket) => {
  console.log('connected');

  socket.on('data', (data) => {
    console.log('R:', data.toString());

    wss.clients.forEach((client) => {
      if (client.readyState === WebSocket.OPEN) {
        client.send(data.toString());
      }
    });
  });

  socket.on('end', () => {
    console.log('disconnected');
  });
});


server.listen(PORT, HOST, () => {
  console.log(`Server listening on ${HOST}:${PORT}`);
});

const wss = new WebSocket.Server({ port: 3001 });

wss.on('connection', (ws) => {
  console.log('WebSocket connected');

  ws.on('close', () => {
    console.log('WebSocket disconnected');
  });
});

