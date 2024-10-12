renderFrontend(()=>{
    use("silent-errors", console, this);
    console.log(null, undefined, this, globalThis, window, true, false, NaN, Infinity, -Infinity);
});
