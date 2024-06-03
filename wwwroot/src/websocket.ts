import {Command, Request} from './data';

// Base URL
const BASE_URL = 'http://0.0.0.0:8080';

// WebSocket connection
const socket = new WebSocket(`${BASE_URL.replace('http', 'ws')}/commands`);

socket.addEventListener('open', () => {
    console.log('Connected to WebSocket server');

    const request: Request = {
        command: Command.Start
    };
    //sendRequest(request);
});

socket.addEventListener('message', (event) => {
    console.log('Message from server:', event.data);
});

socket.addEventListener('close', () => {
    console.log('Disconnected from WebSocket server');
});

socket.addEventListener('error', (error) => {
    console.error('WebSocket error:', error);
});

export function sendCommand(command: Command): void {
    const request: Request = {
        command: command
    }
    if (socket.readyState === WebSocket.OPEN) {
        socket.send(JSON.stringify(request));
        console.log('Sent request:', request);
    } else {
        console.error('WebSocket is not open. Ready state:', socket.readyState);
    }
}
