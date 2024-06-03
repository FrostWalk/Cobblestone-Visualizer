import {Command, Request} from './data.js';
import {BASE_URL} from "./variables.js";

const socket = new WebSocket(`${BASE_URL.replace('http', 'ws')}/commands`);

// Event listener for receiving messages
socket.addEventListener('message', (event) => {
    console.log('Message from server:', event.data);
    // Handle incoming messages from the server here
});

// Event listener for connection close
socket.addEventListener('close', () => {
    console.log('Disconnected from the WebSocket server');
});

// Event listener for connection error
socket.addEventListener('error', (error) => {
    console.error('WebSocket error:', error);
});

// Function to send a request
function sendCommand(command: Command) {
    if (socket.readyState === WebSocket.OPEN) {
        const request: Request = {
            command: command
        };

        socket.send(JSON.stringify(request));
        console.log('Sent command:', command);
    } else {
        console.error('WebSocket is not open. Ready state:', socket.readyState);
    }
}

sendCommand(Command.Start);