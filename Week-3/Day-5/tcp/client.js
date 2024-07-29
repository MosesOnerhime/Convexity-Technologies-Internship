const net = require('net');

// creates a client object
const client = net.createConnection({ port: 3000 }, () => {
  console.log('Connected to server');
  // Send some data to the server
  client.write('Hello, server!');
});

// handles incoming data
client.on('data', (data) => {
  console.log(`Received: ${data}`);
  // End the connection after receiving the response
  client.end();
});

// handles client disconnection
client.on('end', () => {
  console.log('Disconnected from server');
});
