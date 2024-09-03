<div>
        {array.$.map((item)=>item)}
        {array.$.map((item)=>item * x)}
         {array.$.map((item)=>{
    return item * x;
})}
        {array.$.map((item)=>{
    return <span>{_$(()=>item * x)}</span>;
})}
        {array.$.map((item)=><div>{_$(()=>item * x)}</div>)}
        {array.$.filter((item)=>{
    return <span>{_$(()=>item * x)}</span>;
})}
         {_$(()=>array.normalMethod((item)=>{
        return <span>{item * 2}</span>;
    }))}
    </div>;
