export function resizeCanvas(): void {
    const canvas = document.getElementById('draw-area') as HTMLCanvasElement;
    const sidebarWidth = 200;
    const verticalMargin = 20;

    if (canvas) {
        canvas.width = window.innerWidth - sidebarWidth * 2;
        canvas.height = window.innerHeight -  verticalMargin;
    }
}