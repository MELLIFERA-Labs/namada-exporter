export declare function hasMaspParams(): Promise<boolean>;
export declare function fetchAndStoreMaspParams(url?: string): Promise<[void, void, void]>;
export declare function getMaspParams(): Promise<[unknown, unknown, unknown]>;
export declare function fetchAndStore(params: string, url?: string): Promise<void>;
export declare function fetchParams(params: string, url?: string): Promise<Uint8Array>;
export declare function get(key: string): Promise<unknown>;
export declare function has(key: string): Promise<boolean>;
export declare function set(key: string, data: unknown): Promise<void>;
