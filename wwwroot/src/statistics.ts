export function setEnergy(value: number): void {
    const elem = document.getElementById("energy") as HTMLSpanElement;
    if (elem) {
        elem.textContent = value.toString();
    }
}

export function setTime(value: string): void {
    const elem = document.getElementById("time") as HTMLSpanElement;
    if (elem) {
        elem.textContent = value;
    }
}

export function setCoordinates(x: number, y: number): void {
    const elem = document.getElementById("coordinates") as HTMLSpanElement;
    if (elem) {
        elem.textContent = `X: ${x} Y: ${y}`;
    }
}