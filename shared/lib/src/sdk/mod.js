"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.hasMaspParams = hasMaspParams;
exports.fetchAndStoreMaspParams = fetchAndStoreMaspParams;
exports.getMaspParams = getMaspParams;
exports.fetchAndStore = fetchAndStore;
exports.fetchParams = fetchParams;
exports.get = get;
exports.has = has;
exports.set = set;
const PREFIX = "Namada::SDK";
async function hasMaspParams() {
    return ((await has("masp-spend.params")) &&
        (await has("masp-output.params")) &&
        (await has("masp-convert.params")));
}
async function fetchAndStoreMaspParams(url) {
    return Promise.all([
        fetchAndStore("masp-spend.params", url),
        fetchAndStore("masp-output.params", url),
        fetchAndStore("masp-convert.params", url),
    ]);
}
async function getMaspParams() {
    return Promise.all([
        get("masp-spend.params"),
        get("masp-output.params"),
        get("masp-convert.params"),
    ]);
}
async function fetchAndStore(params, url) {
    const data = await fetchParams(params, url);
    await set(params, data);
}
async function fetchParams(params, url = "https://github.com/anoma/masp-mpc/releases/download/namada-trusted-setup/") {
    return fetch(`${url}${params}`)
        .then((response) => response.arrayBuffer())
        .then((ab) => new Uint8Array(ab));
}
function getDB() {
    return new Promise((resolve, reject) => {
        const request = indexedDB.open(PREFIX);
        request.onerror = (event) => {
            event.stopPropagation();
            reject(event.target);
        };
        request.onupgradeneeded = (event) => {
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            const db = event.target.result;
            db.createObjectStore(PREFIX, { keyPath: "key" });
        };
        request.onsuccess = () => {
            resolve(request.result);
        };
    });
}
async function get(key) {
    const tx = (await getDB()).transaction(PREFIX, "readonly");
    const store = tx.objectStore(PREFIX);
    return new Promise((resolve, reject) => {
        const request = store.get(key);
        request.onerror = (event) => {
            event.stopPropagation();
            reject(event.target);
        };
        request.onsuccess = () => {
            if (!request.result) {
                resolve(undefined);
            }
            else {
                resolve(request.result.data);
            }
        };
    });
}
async function has(key) {
    const tx = (await getDB()).transaction(PREFIX, "readonly");
    const store = tx.objectStore(PREFIX);
    return new Promise((resolve, reject) => {
        const request = store.openCursor(key);
        request.onerror = (event) => {
            event.stopPropagation();
            reject(event.target);
        };
        request.onsuccess = (e) => {
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            const cursor = e.target.result;
            resolve(!!cursor);
        };
    });
}
async function set(key, data) {
    const tx = (await getDB()).transaction(PREFIX, "readwrite");
    const store = tx.objectStore(PREFIX);
    return new Promise((resolve, reject) => {
        const request = store.put({
            key,
            data,
        });
        request.onerror = (event) => {
            event.stopPropagation();
            reject(event.target);
        };
        request.onsuccess = () => {
            resolve();
        };
    });
}
