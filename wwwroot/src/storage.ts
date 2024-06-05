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

function getItem(key: string): any {
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
    if (Number.isInteger(value)) {
        setItem('wait', value);
    } else {
        console.error('Error: value must be an integer.');
    }
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
