import {Command, Request} from './request';
import {BASE_URL} from "./variables";
import {LibEvent, Update} from "./update";

// WebSocket connection
const commandSocket = new WebSocket(`${BASE_URL.replace('http', 'ws')}/commands`);
const updatesSocket = new WebSocket(`${BASE_URL.replace('http', 'ws')}/updates`);

commandSocket.addEventListener('message', (event) => {
});

commandSocket.addEventListener('close', () => {
    console.log('Disconnected from command socket');
});

commandSocket.addEventListener('open', () => {
    console.log('Connected to command socket');
});

commandSocket.addEventListener('error', (error) => {
    alert(`Command socket error: ${error}`)
});

updatesSocket.addEventListener('open', () => {
    console.log('Connected to update socket');
});

updatesSocket.addEventListener('close', (event) => {
    console.log('Disconnected from update socket', event);
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

    updatesSocket.addEventListener('message', (event) => {
        try {
            // Deserializzare il messaggio JSON
            const update: Update = JSON.parse(event.data);
            console.log(update.robot_data.backpack);
            if (update.event == LibEvent.Terminated) {
                alert("Robot terminated is job, reload the page to start over");
                sendCommand(Command.Stop);
                closeSockets();

            }
        } catch (error) {
            alert(`Error deserializing update:${error}`);
        }
    });

    updatesSocket.addEventListener('error', (error) => {
        alert(`Update socket error: ${error.type}`);
        closeSockets();
    });
}

export function closeSockets(){
    updatesSocket.close();
    commandSocket.close();
}