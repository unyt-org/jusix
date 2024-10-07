renderFrontend(()=>{
    use("silent-errors", console, outer1, inner1);
    {
        const inner1 = 1;
        const inner2 = 2;
        console.log(inner1, inner2, outer1);
    }
    console.log(inner1);
});
