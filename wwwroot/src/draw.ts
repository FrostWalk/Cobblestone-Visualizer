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