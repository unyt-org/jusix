let x = 10;
run(()=>{
    use("silent-errors", console, x);
    console.log(x + 1);
});
