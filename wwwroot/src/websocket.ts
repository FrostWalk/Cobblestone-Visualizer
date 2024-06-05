import {Command, Request} from './request';
import {BASE_URL} from "./variables";
import {Update} from "./update";
import {getWait} from "./storage";

// WebSocket connection
const commandSocket = new WebSocket(`${BASE_URL.replace('http', 'ws')}/commands`);
commandSocket.addEventListener('message', (event) => {
});

commandSocket.addEventListener('close', () => {
    console.log('Disconnected from WebSocket server');
});

commandSocket.addEventListener('error', (error) => {
    alert(`Command socket error: ${error}`)
});

export function sendCommand(command: Command): void {
    const request: Request = {
        command: command
    }
    if (commandSocket.readyState === WebSocket.OPEN) {
        commandSocket.send(JSON.stringify(request));
    } else {
        alert(`WebSocket is not open. ${commandSocket.readyState}`);
    }
}

export function createWebSocket() {
    const updatesSocket = new WebSocket(`${BASE_URL.replace('http', 'ws')}/updates`);
    let disconnections = 0;

    updatesSocket.addEventListener('message', (event) => {
        try {
            // Deserializzare il messaggio JSON
            const update: Update = JSON.parse(event.data);
            console.log('Received update:', update);
        } catch (error) {
            alert(`Error deserializing update:${error}`);
        }
    });

    updatesSocket.addEventListener('close', (event) => {
        const wait: number = getWait();
        console.log('Disconnected from WebSocket server', event);

        if (disconnections < 3) {
            disconnections += 1;
            createWebSocket();
        }
    });

    updatesSocket.addEventListener('error', (error) => {
        alert(`Update socket error: ${error.type}`);
        updatesSocket.close();
        commandSocket.close();
    });
}