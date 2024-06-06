export function resizeCanvas(): void {
    const canvas = document.getElementById('draw-area') as HTMLCanvasElement;
    const container = document.querySelector('.container') as HTMLElement;
    const sidebarWidth = 200;
    const verticalMargin = 20;

    if (canvas && container) {
        const containerHeight = container.clientHeight;
        canvas.width = window.innerWidth - sidebarWidth * 2;
        canvas.height = window.innerHeight - containerHeight - verticalMargin;
    }
}

export function setEnergy(value: string): void {
    const energyElement = document.getElementById('energy');
    if (energyElement) {
        energyElement.textContent = value;
    }
}

export function setTime(value: string): void {
    const timeElement = document.getElementById('time');
    if (timeElement) {
        timeElement.textContent = value;
    }
}

export function setCoordinates(value: string): void {
    const coordinatesElement = document.getElementById('coordinates');
    if (coordinatesElement) {
        coordinatesElement.textContent = value;
    }
}