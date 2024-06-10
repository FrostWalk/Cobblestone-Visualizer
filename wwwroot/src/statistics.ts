import {Backpack, DayTime, Environment, RobotCoordinate, WeatherType} from "./datatypes";
import {
    setBushNumber,
    setCoinNumber,
    setFireNumber,
    setFishNumber,
    setJollyNumber,
    setStoneNumber,
    setTrashNumber,
    setWaterNumber,
    setWoodNumber
} from "./items";

export function setEnergy(value: number): void {
    const elem = document.getElementById("energy") as HTMLSpanElement;
    if (elem) {
        elem.textContent = value.toString();
    }
}

export function setTime(value: Environment): void {
    let elem = document.getElementById("time") as HTMLSpanElement;
    if (elem) {
        elem.textContent = value.time;
    }
    elem = document.getElementById("day-time") as HTMLSpanElement;
    if (elem) {
        elem.textContent = value.day_time.toString() + ', ' + value.weather.toString();
    }
}

export function setCoordinates(value: RobotCoordinate): void {
    const elem = document.getElementById("coordinates") as HTMLSpanElement;
    if (elem) {
        elem.textContent = `X: ${value.row} Y: ${value.col}`;
    }
}

export function setBackpack(backpack: Backpack): void {
    setStoneNumber(backpack.Rock);
    setWoodNumber(backpack.Tree);
    setTrashNumber(backpack.Garbage);
    setFireNumber(backpack.Fire);
    setCoinNumber(backpack.Coin);
    setWaterNumber(backpack.Water);
    setFishNumber(backpack.Fish);
    setBushNumber(backpack.Bush);
    setJollyNumber(backpack.JollyBlock);
}

export function setWeather(environment: Environment): void {
    const weatherImg = document.getElementById('weather') as HTMLImageElement;
    if (!weatherImg) {
        console.error('Element with id "weather" not found');
        return;
    }

    const base: String = 'dist/img/';

    if (environment.weather == WeatherType.Sunny) {
        switch (environment.day_time) {
            case DayTime.Morning:
                weatherImg.src = `${base}morning.png`;
                break;
            case DayTime.Afternoon:
                weatherImg.src = `${base}afternoon.png`;
                break;
            case DayTime.Night:
                weatherImg.src = `${base}night.png`;
                break;
        }
        return;
    }

    switch (environment.weather) {
        case WeatherType.Rainy:
            weatherImg.src = `${base}rain.png`;
            break;
        case WeatherType.Foggy:
            weatherImg.src = `${base}fog.png`;
            break;
        case WeatherType.TropicalMonsoon:
            weatherImg.src = `${base}storm.png`;
            break;
        case WeatherType.TrentinoSnow:
            weatherImg.src = `${base}snow.png`;
            break;
        default:
            weatherImg.src = '';
            break;
    }
}

export function addEventEntry(event: string[]): void {
    const logBox = document.getElementById('log-box') as HTMLDivElement;

    for (let e of event) {
        while (logBox.children.length > 11) {
            logBox.removeChild(logBox.firstChild as Node);
        }

        const logEntry = document.createElement('div');
        logEntry.className = 'log-entry';
        logEntry.textContent = e;
        logBox.appendChild(logEntry);
    }
    logBox.scrollTop = logBox.scrollHeight;
}