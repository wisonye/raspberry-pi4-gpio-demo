export const js_sleep = (ms) => {
    return new Promise((resolve, reject) => {
        setTimeout(() => {Promise.resolve()}, ms)
    })
}
