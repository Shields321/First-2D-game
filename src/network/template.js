const WebSocket = require('ws');

const wss = new WebSocket.Server({ port: 8080 });

wss.on('connection', function connection(ws) {
  console.log('WebSocket connection established');

  ws.on('message', function incoming(message) {
    console.log('Message received from client:', message);
    
    // Echo the message back to the client
    ws.send('Hello from the server!');
  });

  ws.on('close', function close() {
    console.log('WebSocket connection closed');
  });
});
