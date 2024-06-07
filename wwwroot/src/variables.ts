export const BASE_URL = 'http://0.0.0.0:8080';
let isInPause: boolean = false;

export function setSize(value: string): void {
    const elem = document.getElementById("w-size") as HTMLSpanElement;
    if (elem) {
        elem.textContent = value;
    }
}

export function setSeed(value: string): void {
    const elem = document.getElementById("w-seed") as HTMLSpanElement;
    if (elem) {
        elem.textContent = value;
    }
}

export function setRobot(value: string): void {
    const elem = document.getElementById("w-robot") as HTMLSpanElement;
    if (elem) {
        elem.textContent = value;
    }
}

export function IsInPause(): boolean {
    return isInPause;
}

export function setIsInPause(v: boolean): void {
    isInPause = v;
}