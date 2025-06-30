/**
 * Small wrapper for fetch to make it easier to pass props
 * Called wasmFetch to avoid naming conflict
 */
export declare function wasmFetch(url: string, method: string, body: string): Promise<Response>;
