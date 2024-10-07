renderFrontend(async ()=>{
    use("silent-errors", console, fn3, fn1);
    function blabla(fn1, fn2) {
        console.log(fn1, fn2, fn3);
    }
    console.log(fn1, blabla());
});
