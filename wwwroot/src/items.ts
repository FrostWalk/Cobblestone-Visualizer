import {Content, ContentType, RobotBackPack} from "./update";

function stoneShow(show: boolean): void {
    const elem = document.getElementById("stone");
    if (elem) {
        elem.style.display = show ? "flex" : "none";
    }
}

function woodShow(show: boolean): void {
    const elem = document.getElementById("wood");
    if (elem) {
        elem.style.display = show ? "flex" : "none";
    }
}

function coinShow(show: boolean): void {
    const elem = document.getElementById("coin");
    if (elem) {
        elem.style.display = show ? "flex" : "none";
    }
}

function fireShow(show: boolean): void {
    const elem = document.getElementById("fire");
    if (elem) {
        elem.style.display = show ? "flex" : "none";
    }
}

function waterShow(show: boolean): void {
    const elem = document.getElementById("water");
    if (elem) {
        elem.style.display = show ? "flex" : "none";
    }
}

function trashShow(show: boolean): void {
    const elem = document.getElementById("trash");
    if (elem) {
        elem.style.display = show ? "flex" : "none";
    }
}

function fishShow(show: boolean): void {
    const elem = document.getElementById("fish");
    if (elem) {
        elem.style.display = show ? "flex" : "none";
    }
}

function bushShow(show: boolean): void {
    const elem = document.getElementById("bush");
    if (elem) {
        elem.style.display = show ? "flex" : "none";
    }
}

function jollyShow(show: boolean): void {
    const elem = document.getElementById("jolly");
    if (elem) {
        elem.style.display = show ? "flex" : "none";
    }
}

function setStoneNumber(number: number | undefined): void {
    const elem = document.getElementById("stone-number") as HTMLDivElement;
    if (elem && number != undefined) {
        elem.textContent = number.toString();
        stoneShow(number !== 0);
    }
}

function setWoodNumber(number: number | undefined): void {
    const elem = document.getElementById("wood-number") as HTMLDivElement;
    if (elem && number != undefined) {
        elem.textContent = number.toString();
        woodShow(number !== 0);
    }
}

function setCoinNumber(number: number | undefined): void {
    const elem = document.getElementById("coin-number") as HTMLDivElement;
    if (elem && number != undefined) {
        elem.textContent = number.toString();
        coinShow(number !== 0);
    }
}

function setFireNumber(number: number | undefined): void {
    const elem = document.getElementById("fire-number") as HTMLDivElement;
    if (elem && number != undefined) {
        elem.textContent = number.toString();
        fireShow(number !== 0);
    }
}

function setWaterNumber(number: number | undefined): void {
    const elem = document.getElementById("water-number") as HTMLDivElement;
    if (elem && number != undefined) {
        elem.textContent = number.toString();
        waterShow(number !== 0);
    }
}

function setTrashNumber(number: number | undefined): void {
    const elem = document.getElementById("trash-number") as HTMLDivElement;
    if (elem && number != undefined) {
        elem.textContent = number.toString();
        trashShow(number !== 0);
    }
}

function setFishNumber(number: number | undefined): void {
    const elem = document.getElementById("fish-number") as HTMLDivElement;
    if (elem && number != undefined) {
        elem.textContent = number.toString();
        fishShow(number !== 0);
    }
}

function setBushNumber(number: number | undefined): void {
    const elem = document.getElementById("bush-number") as HTMLDivElement;
    if (elem && number != undefined) {
        elem.textContent = number.toString();
        bushShow(number !== 0);
    }
}

function setJollyNumber(number: number | undefined): void {
    const elem = document.getElementById("jolly-number") as HTMLDivElement;
    if (elem && number != undefined) {
        elem.textContent = number.toString();
        jollyShow(number !== 0);
    }
}

export function setBackpack(backpack: RobotBackPack): void {
    let content: Content = {
        type: ContentType.Rock
    }
    console.log(backpack.contents);
    setStoneNumber(backpack.contents.get(content) || 0);

    content.type = ContentType.Tree;
    setWoodNumber(backpack.contents.get(content) || 0);

    content.type = ContentType.Water;
    setWaterNumber(backpack.contents.get(content) || 0);

    content.type = ContentType.Coin;
    setCoinNumber(backpack.contents.get(content) || 0);

    content.type = ContentType.Fire;
    setFireNumber(backpack.contents.get(content) || 0);

    content.type = ContentType.Garbage;
    setTrashNumber(backpack.contents.get(content) || 0);

    content.type = ContentType.Fish;
    setFishNumber(backpack.contents.get(content) || 0);

    content.type = ContentType.Bush;
    setBushNumber(backpack.contents.get(content) || 0);

    content.type = ContentType.JollyBlock;
    setJollyNumber(backpack.contents.get(content) || 0);
}
