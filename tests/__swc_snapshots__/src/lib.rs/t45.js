renderFrontend(()=>{
    use("silent-errors", console);
    console.log(null, undefined, globalThis, window, true, false, NaN, Infinity, -Infinity);
});
