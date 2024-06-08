import {Command, Request} from './request';
import {BASE_URL, getRobot} from "./variables";
import {LibEventType, Update} from "./datatypes";
import {setBackpack, setCoordinates, setEnergy, setTime, setWeather} from "./statistics";
import {drawMap} from "./draw";

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

    commandSocket.onclose = () => {
        console.log('Disconnected from command socket');
    };

    commandSocket.onerror = (error) => {
        alert(`WebSocket error:${error}`);
    };

    updatesSocket.onopen = () => {
        console.log('connected to updates socket ');
    };

    updatesSocket.onclose = (event) => {
        console.log('Disconnected from update socket', event);
    };

    updatesSocket.onmessage = (event) => {
        try {
            if (event.data == 'ping') {
                return;
            }
            const update: Update = JSON.parse(event.data);

            setCoordinates(update.robot_data.coordinate);
            setEnergy(update.robot_data.energy_level);
            setBackpack(update.robot_data.backpack);
            setTime(update.environment);
            setWeather(update.environment);
            drawMap(update.map, update.robot_data.coordinate);

            if (update.event && update.event.type == LibEventType.Terminated) {
                alert(`${getRobot()} terminated his job, reload the page to start over`);
                sendCommand(Command.Stop);
                closeSockets();
            }
        } catch (error) {
            console.error(event.data);
            alert(`Error deserializing update:${error}`);
        }
    }
}

export function closeSockets() {
    updatesSocket.close();
    commandSocket.close();
}