import {Command, Request} from './data';

// Base URL
const BASE_URL = 'http://0.0.0.0:8080';

// WebSocket connection
const commandSocket = new WebSocket(`${BASE_URL.replace('http', 'ws')}/commands`);

commandSocket.addEventListener('open', () => {
    console.log('Connected to WebSocket server');

    const request: Request = {
        command: Command.Start
    };
    //sendRequest(request);
});

commandSocket.addEventListener('message', (event) => {
    console.log('Message from server:', event.data);
});

commandSocket.addEventListener('close', () => {
    console.log('Disconnected from WebSocket server');
});

commandSocket.addEventListener('error', (error) => {
    console.error('WebSocket error:', error);
});

export function sendCommand(command: Command): void {
    const request: Request = {
        command: command
    }
    if (commandSocket.readyState === WebSocket.OPEN) {
        commandSocket.send(JSON.stringify(request));
        console.log('Sent request:', request);
    } else {
        console.error('WebSocket is not open. Ready state:', commandSocket.readyState);
    }
}
