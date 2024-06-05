export enum WeatherType {
    Sunny = 'Sunny',
    Rainy = 'Rainy',
    Foggy = 'Foggy',
    TropicalMonsoon = 'TropicalMonsoon',
    TrentinoSnow = 'TrentinoSnow'
}

export enum DayTime {
    Morning = 'Morning', Afternoon = 'Afternoon', Night = 'Night'
}

export enum LibEvent {
    Ready = 'Ready',
    Terminated = 'Terminated',
}

export enum TileType {
    DeepWater = 'DeepWater',
    ShallowWater = 'ShallowWater',
    Sand = 'Sand',
    Grass = 'Grass',
    Street = 'Street',
    Hill = 'Hill',
    Mountain = 'Mountain',
    Snow = 'Snow',
    Lava = 'Lava',
    Teleport = 'Teleport',
    Wall = 'Wall'
}

export enum ContentType {
    Rock = 'Rock',
    Tree = 'Tree',
    Garbage = 'Garbage',
    Fire = 'Fire',
    Coin = 'Coin',
    Bin = 'Bin',
    Crate = 'Crate',
    Bank = 'Bank',
    Water = 'Water',
    Market = 'Market',
    Fish = 'Fish',
    Building = 'Building',
    Bush = 'Bush',
    JollyBlock = 'JollyBlock',
    Scarecrow = 'Scarecrow',
    None = 'None'
}

export interface Environment {
    time: string;
    weather: WeatherType;
    day_time: DayTime;
}

export interface EnvironmentalConditions {
    time_progression_minutes: number;
    time_of_day: TimeOfDay;
    weather_forecast: WeatherType[];
}

export interface TimeOfDay {
    hour: number;
    minute: number;
}

export interface Tile {
    tile_type: TileType;
    content: Content;
    elevation: number;
}

export interface Content {
    type: ContentType;
    quantity?: number;
}

export interface RobotData {
    energy_level: number;
    coordinate: RobotCoordinate;
    backpack: RobotBackPack;
}

export interface RobotCoordinate {
    row: number;
    col: number;
}

export interface RobotBackPack {
    size: number;
    contents: Map<Content, number>;
}

export interface Update {
    event: LibEvent | null;
    robot_data: RobotData;
    environmental_condition: Environment;
    map: (Tile | null)[][];
}