const x1 = <div>
        {_$method(apps, "map", (section)=><div>
                {_$method(_$(()=>Object.entries(section.apps)), "map", ()=><a/>)}
            </div>)}
    </div>;
const x2 = <div>
        {_$method(apps, "map", (section)=><div x={_$(()=>1 + 2)}>
                {_$method(_$(()=>Object.entries(section.apps)), "map", ()=><a/>)}
            </div>)}
    </div>;
const x3 = <div>
        {_$method(apps, "map", (section)=><div x={1 + 2}>
                {_$(()=>x * 100)}
            </div>)}
    </div>;
