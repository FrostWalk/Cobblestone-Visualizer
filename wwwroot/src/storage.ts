function setItem(key: string, value: any): void {
    if (typeof window !== 'undefined') {
        try {
            const serializedValue = JSON.stringify(value);
            localStorage.setItem(key, serializedValue);
        } catch (error) {
            console.error('Error setting item in localStorage:', error);
        }
    }
}

function getItem(key: string): string {
    if (typeof window !== 'undefined') {
        try {
            const serializedValue = localStorage.getItem(key);
            return serializedValue === null ? '' : JSON.parse(serializedValue);
        } catch (error) {
            console.error('Error getting item from localStorage:', error);
            return '';
        }
    }
    return '';
}

export function setWait(value: number): void {
    setItem('wait', value);

}

export function getWait(): number {
    const value = getItem('wait');
    if (value === '') {
        return 100;
    }

    const parsedValue = parseInt(value, 10);
    if (Number.isInteger(parsedValue)) {
        return parsedValue;
    } else {
        console.error('Error: stored value is not an integer.');
        return 100;
    }
}


export function setSeed(value: number): void {
    setItem('seed', value);
}

export function getSeed(): number {
    const value = getItem('seed');
    if (value === '') {
        return 0;
    }

    const parsedValue = parseInt(value, 10);
    if (Number.isInteger(parsedValue)) {
        return parsedValue;
    } else {
        console.error('Error: stored value is not an integer.');
        return 0;
    }
}

export function setSize(value: number): void {
    setItem('size', value);
}

export function getSize(): number {
    const value = getItem('size');
    if (value === '') {
        return 100;
    }

    const parsedValue = parseInt(value, 10);
    if (Number.isInteger(parsedValue)) {
        return parsedValue;
    } else {
        console.error('Error: stored value is not an integer.');
        return 100;
    }
}

export function setIsRunning(value: boolean): void {
    setItem('size', value);
}


export function getIsRunning(): boolean {
    const value = getItem('isRunning');
    if (value === '') {
        return false;
    }

    return value === 'true';
}
