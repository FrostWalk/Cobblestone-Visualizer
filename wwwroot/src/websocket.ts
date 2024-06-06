import {Command, Request} from './request';
import {BASE_URL} from "./variables";
import {LibEvent, Update} from "./datatypes";
import {setCoordinates, setEnergy, setTime} from "./statistics";
import {setBackpack} from "./items";

const commandSocket = new WebSocket(`${BASE_URL.replace('http', 'ws')}/commands`);
let updatesSocket: WebSocket;

export function sendCommand(command: Command): void {
    const request: Request = {
        command: command
    }
    if (commandSocket.readyState === WebSocket.OPEN) {
        commandSocket.send(JSON.stringify(request));
    } else {
        alert(`Command socket is not open. ${commandSocket.readyState}`);
    }
}

export function initUpdateSockets() {
    updatesSocket = new WebSocket(`${BASE_URL.replace('http', 'ws')}/updates`);

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

    updatesSocket.addEventListener('message', (event) => {
        try {
            const update: Update = JSON.parse(event.data);

            setCoordinates(update.robot_data.coordinate);
            setEnergy(update.robot_data.energy_level);
            setTime(update.environment.time, update.environment.day_time);
            setBackpack(update.robot_data.backpack);

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

export function closeSockets() {
    updatesSocket.close();
    commandSocket.close();
}