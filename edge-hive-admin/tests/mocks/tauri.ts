
export const invoke = (command: string, args?: any) => {
    if (command === 'stream_logs') {
        return Promise.resolve();
    }
    if (command === 'get_config') {
        return Promise.resolve('[server]\nport = 8080');
    }
    if (command === 'save_config') {
        return Promise.resolve();
    }
    return Promise.reject(new Error(`Unknown command: ${command}`));
};
