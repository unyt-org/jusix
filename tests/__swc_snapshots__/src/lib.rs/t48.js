class A {
    test() {
        return <div>
                <input value={prop(x, "value")}/>
                <input value={prop(this, "value")}/>
                <input value={_$(()=>super.value)}/>
                <input value={prop(prop(a, 'x'), "value")}/>
            </div>;
    }
}
