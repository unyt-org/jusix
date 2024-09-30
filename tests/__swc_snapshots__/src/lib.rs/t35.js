run(async ()=>{
    use(w, externalFn, console);
    const { f, x: g, ...z } = await import("datex-core-legacy");
    const [v, ...w] = externalFn();
    let x = 10;
    console.log(x, f, g, z);
    f(1);
});
