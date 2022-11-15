/**
 * @internal
 */
export function getDotNotation(obj: any, path: string): any {
    const keys = path.split('.');
    return keys.reduce((obj, k) => obj[k], obj);
}

/**
 * @internal
 */
export function setDotNotation(obj: any, path: string, value: any): any {
    const keys = path.split('.');
    let current = obj;
    for (let i = 0; i < keys.length - 1; i++) {
        const key = keys[i];
        if (!(key in current)) current[key] = {};
        current = current[key];
    }
    current[keys[keys.length - 1]] = value;
    return value;
}