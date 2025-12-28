
export const listen = (event: string, handler: (event: { payload: any }) => void) => {
    if (event === 'log-message') {
        const interval = setInterval(() => {
            handler({ payload: 'Mock log message' });
        }, 1000);
        return Promise.resolve(() => clearInterval(interval));
    }
    return Promise.resolve(() => {});
};
