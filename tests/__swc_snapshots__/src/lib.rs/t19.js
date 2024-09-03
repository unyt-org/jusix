<div>
        {array.$.map((item)=>{
    return <span>{item * 2}</span>;
})}
        {array.$.filter((item)=>{
    return <span>{item * 2}</span>;
})}
         {always(()=>array.normalMethod((item)=>{
        return <span>{item * 2}</span>;
    }))}
    </div>;
