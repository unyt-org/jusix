template(()=>{
    const y = always(()=>42 + x);
    const z = <div>{_$(()=>y + 1)}</div>;
});
