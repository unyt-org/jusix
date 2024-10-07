call(function({ title, icon, children }) {
    return <div class="card">
            <span title="Dialog schließen" onclick:frontend={()=>{
        return use(this) && this.closeDialog();
    }} class="desktop-close"><i class="fa-solid fa-xmark"/></span>
            <h1>
                <span title="Zurück" onclick:frontend={()=>{
        use("silent-errors", this);
        return this.closeDialog();
    }} class="mobile-back"><i class="fa-solid fa-chevron-left"/></span>
                {_$(()=>icon && <i class={`fas ${icon}`} style="margin-right:10px"/>)}
                {title}
            </h1>
            {...children}
            {...children}
            {..._$(()=>[
            1,
            2,
            3
        ])}
            {..._$method([
        1,
        2,
        3
    ], "map", (item)=><span>{item}</span>)}
        </div>;
});
