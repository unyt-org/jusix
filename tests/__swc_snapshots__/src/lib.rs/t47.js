renderFrontend(()=>{
    use("silent-errors", console, e2);
    try {} catch (e1) {
        console.log(e1);
    }
    try {} catch (e2) {
        console.log(e2);
    }
    console.log(e2);
});
