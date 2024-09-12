()=>{
    const y = $$(42);
    const z = <div>{_$(()=>y + 1)}</div>;
};
