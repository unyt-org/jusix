<div>
        {renderFrontend(()=>{
    use(x);
    return <div>
                    <MyComponent/>
                    4 + {x} = {x + 4}
                </div>;
})}
        {renderBackend(<div>{_$(()=>x + 1)}</div>)}
    </div>;