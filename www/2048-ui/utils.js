
// return JSON.parse'd data if exists or undefined otherwise
export const getFromLocalStorage = (key) => {
    const raw = localStorage.getItem(key);
    return raw ? JSON.parse(raw) : undefined;
}

export const setLocalStorage = (key, val) => {
    localStorage.setItem(key, JSON.stringify(val));
}