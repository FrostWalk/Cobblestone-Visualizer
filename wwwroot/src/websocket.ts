import {Command, Request} from './request';
import {BASE_URL} from "./variables";
import {LibEvent, Update} from "./update";

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

export function initUpdateSocket() {
    const updatesSocket = new WebSocket(`${BASE_URL.replace('http', 'ws')}/updates`);

    updatesSocket.addEventListener('message', (event) => {
        try {
            // Deserializzare il messaggio JSON
            const update: Update = JSON.parse(event.data);

            if (update.event == LibEvent.Terminated) {
                alert("Robot terminated is job, reload the page to start over");
                sendCommand(Command.Stop);
                updatesSocket.close();
                commandSocket.close();
            }

            console.log(update);
        } catch (error) {
            alert(`Error deserializing update:${error}`);
        }
    });

    updatesSocket.addEventListener('close', (event) => {
        console.log('Disconnected from WebSocket server', event);
    });

    updatesSocket.addEventListener('error', (error) => {
        alert(`Update socket error: ${error.type}`);
        updatesSocket.close();
        commandSocket.close();
    });
}