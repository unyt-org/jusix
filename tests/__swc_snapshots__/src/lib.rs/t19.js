<div>
        {_$method(array, "map", (item)=>item)}
        {_$method(array, "map", (item)=>item * x)}
         {_$method(array, "map", (item)=>{
    return item * x;
})}
        {_$method(array, "map", (item)=>{
    return <span>{_$(()=>item * x)}</span>;
})}
        {_$method(array, "map", (item)=><div>{_$(()=>item * x)}</div>)}
        {_$method(array, "filter", (item)=>{
    return <span>{_$(()=>item * x)}</span>;
})}
         {_$(()=>array.normalMethod((item)=>{
        return <span>{item * 2}</span>;
    }))}
    </div>;
