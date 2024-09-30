<input type="button" value="Hello" onclick:frontend={async ()=>{
    use(alert);
    const x = <BaseComponent title="x" color="red"/>;
    const y = 10;
    const { console } = globalThis;
    console.log(x, y);
    globalThis.alert('feef');
    alert("Hello!");
}}/>;
