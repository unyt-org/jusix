<input type="button" value="Hello" onclick:frontend={()=>{
    use(x, y, console, globalThis, alert);
    const x = <BaseComponent title="x" color="red"/>;
    const y = 10;
    console.log(x, y);
    globalThis.alert('feef');
    alert("Hello!");
}}/>;
