const net = require('net');

// Create a server object
const server = net.createServer((socket) => {
  console.log('Client connected');
  
  // Handle incoming data
  socket.on('data', (data) => {
    console.log(`Received: ${data}`);
    // Echo the data back to the client
    socket.write(`You said: ${data}`);
  });

  // Handle client disconnection
  socket.on('end', () => {
    console.log('Client disconnected');
  });
});

// Start the server on port 3000
server.listen(3000, () => {
  console.log('Server listening on port 3000');
});
