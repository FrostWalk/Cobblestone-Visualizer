export enum Command {
    Start = 'Start', Stop = 'Stop', Pause = 'Pause', Resume = 'Resume'
}

export interface Request {
    command: Command;
}