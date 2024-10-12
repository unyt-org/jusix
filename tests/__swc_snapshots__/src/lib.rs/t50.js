<div>
        <div>{prop(x, "y")}</div>
        <div>{x.y}</div>
        <div>{_$(()=>x())}</div>
        <div>{x()}</div>
        <div>{_$(()=>x + 1)}</div>
        <div>{x + 1}</div>

        <div>{x + 1}</div>
        <div>{x + 1}
        </div>
        <div>#staticxy
        {_$(()=>x + 1)}
        </div>
        <div>{x + 1}{_$(()=>x + 2)}</div>
    </div>;
