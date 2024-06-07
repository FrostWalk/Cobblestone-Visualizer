import {ContentType, RobotCoordinate, Tile, TileType} from "./datatypes";

const BIN_IMAGE = new Image();
BIN_IMAGE.src = 'dist/tiles/bin.webp';
const CHEST_IMAGE = new Image();
CHEST_IMAGE.src = 'dist/tiles/chest.webp';
const COBBLESTONE = new Image();
COBBLESTONE.src = 'dist/tiles/cobblestone.webp';
const COIN_IMAGE = new Image();
COIN_IMAGE.src = 'dist/tiles/coin.svg';
const CRATE_IMAGE = new Image();
CRATE_IMAGE.src = 'dist/tiles/crate.webp';
const DEEP_WATER_IMAGE = new Image();
DEEP_WATER_IMAGE.src = 'dist/tiles/deep_water.webp';
const HILL_IMAGE = new Image();
HILL_IMAGE.src = 'dist/tiles/dirt.webp';
const FIRE_IMAGE = new Image();
FIRE_IMAGE.src = 'dist/tiles/fire.svg';
const FISH_IMAGE = new Image();
FISH_IMAGE.src = 'dist/tiles/fish.webp';
const GRASS_IMAGE = new Image();
GRASS_IMAGE.src = 'dist/tiles/grass.webp';
const LAVA_IMAGE = new Image();
LAVA_IMAGE.src = 'dist/tiles/lava.webp';
const MARKET_IMAGE = new Image();
MARKET_IMAGE.src = 'dist/tiles/market.webp';
const TELEPORT_IMAGE = new Image();
TELEPORT_IMAGE.src = 'dist/tiles/portal.webp';
const SAND_IMAGE = new Image();
SAND_IMAGE.src = 'dist/tiles/sand.webp';
const SNOW_IMAGE = new Image();
SNOW_IMAGE.src = 'dist/tiles/snow.webp';
const STREET_IMAGE = new Image();
STREET_IMAGE.src = 'dist/tiles/street.webp';
const TRASH_IMAGE = new Image();
TRASH_IMAGE.src = 'dist/tiles/trash.png';
const TREE_IMAGE = new Image();
TREE_IMAGE.src = 'dist/tiles/tree.webp';
const WALL_IMAGE = new Image();
WALL_IMAGE.src = 'dist/tiles/wall.webp';
const WATER_IMAGE = new Image();
WATER_IMAGE.src = 'dist/tiles/water.webp';
const STONE_IMAGE = new Image();
STONE_IMAGE.src = 'dist/tiles/stone.webp';
const BUSH_IMAGE = new Image();
BUSH_IMAGE.src = 'dist/tiles/bush.webp';
const SCARECROW = new Image();
SCARECROW.src = 'dist/tiles/scarecrow.svg';
const LUCKY_IMAGE = new Image();
LUCKY_IMAGE.src = 'dist/tiles/lucky.webp';
const ROBOT_IMAGE = new Image();
ROBOT_IMAGE.src = 'dist/tiles/creeper.jpg';

export function resizeCanvas(): void {
    const canvas = document.getElementById('draw-area') as HTMLCanvasElement;
    const sidebarWidth = 200;
    const verticalMargin = 20;

    if (canvas) {
        canvas.width = window.innerWidth - sidebarWidth * 2;
        canvas.height = window.innerHeight - verticalMargin;
    }
}

export function drawMap(world_map: (Tile | null)[][], coordinate: RobotCoordinate) {
    const canvas = document.getElementById('draw-area') as HTMLCanvasElement;
    const ctx = canvas.getContext('2d');

    if (!ctx) return;

    ctx.clearRect(0, 0, canvas.width, canvas.height);

    const TILE_SIZE = 32;
    const maxRows = Math.min(world_map.length, Math.ceil(canvas.height / TILE_SIZE));
    const maxCols = Math.min(world_map[0].length, Math.ceil(canvas.width / TILE_SIZE));

    ctx.drawImage(ROBOT_IMAGE, (coordinate.col + 200) * TILE_SIZE, (coordinate.row + 200) * TILE_SIZE);
    for (let row = 0; row < maxRows; row++) {
        for (let col = 0; col < maxCols; col++) {
            const tile = world_map[row][col];
            if (tile) {
                const x = col * TILE_SIZE;
                const y = row * TILE_SIZE;
                if (typeof tile.content !== 'string') {
                    const contentType = Object.keys(tile.content)[0] as keyof typeof ContentType;
                    switch (contentType) {
                        case ContentType.Rock:
                            ctx.drawImage(COBBLESTONE, x, y, TILE_SIZE, TILE_SIZE);
                            break;
                        case ContentType.Tree:
                            ctx.drawImage(TREE_IMAGE, x, y, TILE_SIZE, TILE_SIZE);
                            break;
                        case ContentType.Garbage:
                            ctx.drawImage(TRASH_IMAGE, x, y, TILE_SIZE, TILE_SIZE);
                            break;
                        case ContentType.Fire:
                            ctx.drawImage(FIRE_IMAGE, x, y, TILE_SIZE, TILE_SIZE);
                            break;
                        case ContentType.Coin:
                            ctx.drawImage(COIN_IMAGE, x, y, TILE_SIZE, TILE_SIZE);
                            break;
                        case ContentType.Bin:
                            ctx.drawImage(BIN_IMAGE, x, y, TILE_SIZE, TILE_SIZE);
                            break;
                        case ContentType.Crate:
                            ctx.drawImage(CRATE_IMAGE, x, y, TILE_SIZE, TILE_SIZE);
                            break;
                        case ContentType.Bank:
                            ctx.drawImage(CHEST_IMAGE, x, y, TILE_SIZE, TILE_SIZE);
                            break;
                        case ContentType.Market:
                            ctx.drawImage(MARKET_IMAGE, x, y, TILE_SIZE, TILE_SIZE);
                            break;
                        case ContentType.Fish:
                            ctx.drawImage(FISH_IMAGE, x, y, TILE_SIZE, TILE_SIZE);
                            break;
                        case ContentType.Bush:
                            ctx.drawImage(BUSH_IMAGE, x, y, TILE_SIZE, TILE_SIZE);
                            break;
                        case ContentType.Scarecrow:
                            ctx.drawImage(SCARECROW, x, y, TILE_SIZE, TILE_SIZE);
                            break;
                        case ContentType.JollyBlock:
                            ctx.drawImage(LUCKY_IMAGE, x, y, TILE_SIZE, TILE_SIZE);
                            break;
                        case ContentType.Building:
                            alert("building");
                            break;
                        case ContentType.Water:
                            break;
                    }
                } else {
                    switch (tile.tile_type) {
                        case TileType.DeepWater:
                            ctx.drawImage(DEEP_WATER_IMAGE, x, y, TILE_SIZE, TILE_SIZE);
                            break;
                        case TileType.ShallowWater:
                            ctx.drawImage(WATER_IMAGE, x, y, TILE_SIZE, TILE_SIZE);
                            break;
                        case TileType.Sand:
                            ctx.drawImage(SAND_IMAGE, x, y, TILE_SIZE, TILE_SIZE);
                            break;
                        case TileType.Grass:
                            ctx.drawImage(GRASS_IMAGE, x, y, TILE_SIZE, TILE_SIZE);
                            break;
                        case TileType.Street:
                            ctx.drawImage(STREET_IMAGE, x, y, TILE_SIZE, TILE_SIZE);
                            break;
                        case TileType.Hill:
                            ctx.drawImage(HILL_IMAGE, x, y, TILE_SIZE, TILE_SIZE);
                            break;
                        case TileType.Mountain:
                            ctx.drawImage(STONE_IMAGE, x, y, TILE_SIZE, TILE_SIZE);
                            break;
                        case TileType.Snow:
                            ctx.drawImage(SNOW_IMAGE, x, y, TILE_SIZE, TILE_SIZE);
                            break;
                        case TileType.Lava:
                            ctx.drawImage(LAVA_IMAGE, x, y, TILE_SIZE, TILE_SIZE);
                            break;
                        case TileType.Teleport:
                            ctx.drawImage(TELEPORT_IMAGE, x, y, TILE_SIZE, TILE_SIZE);
                            break;
                        case TileType.Wall:
                            ctx.drawImage(WALL_IMAGE, x, y, TILE_SIZE, TILE_SIZE);
                            break;
                    }
                }
            }
        }
    }
}