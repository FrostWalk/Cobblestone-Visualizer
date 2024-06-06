import {RobotCoordinate} from "./datatypes";

export function setEnergy(value: number): void {
    const elem = document.getElementById("energy") as HTMLSpanElement;
    if (elem) {
        elem.textContent = value.toString();
    }
}

export function setTime(value: string, daytime: string): void {
    let elem = document.getElementById("time") as HTMLSpanElement;
    if (elem) {
        elem.textContent = value;
    }
    elem = document.getElementById("day-time") as HTMLSpanElement;
    if (elem) {
        elem.textContent = daytime;
    }
}

export function setCoordinates(value: RobotCoordinate): void {
    const elem = document.getElementById("coordinates") as HTMLSpanElement;
    if (elem) {
        elem.textContent = `X: ${value.row} Y: ${value.col}`;
    }
}