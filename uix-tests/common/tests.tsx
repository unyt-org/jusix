import Component1, { Car } from "common/Component1.tsx";
import { ObjectRef, Ref } from "datex-core-legacy/runtime/pointers.ts";

export default {

	'/test1': () => <div>Test 1</div>,
	'/test2': () => {
		const value = $(0);

		const data = always({a: value.val});

		console.log(data);

		const data2 = $({a: 0});
		effect(() => {
			console.log("value = " + value)
			data2.a = value;
		});

		let x = $$(1);

		const classes = $$(["a", "b"]);
		setInterval(() => {
			classes.push("c_" + Math.round(Math.random()*100));
		}, 1000);

		const classesObj = $$({a: true, b: false, c:4});
		setInterval(() => {
			classesObj.a = !classesObj.a;
			classesObj.b = !classesObj.b;
		}, 1000);

		return <div id="xy" class={$$("xyz")}>
			<input
				class={classes}
				disabled={$$(false)}
				required={true}
				type={$$("text" as const)} 
				value={value}
				onclick={(e) => {
					console.log("click", e);
				}}
				onclick:frontend={(e) => {
					console.log("frontend click", e);
				}}
			/>
			
			<input class={['3',5,true]} type="number" value={'xy'}/>
			<input type={"button"} value={$("fe")} style={$$({color: 'red', x: [3]})}/>

			<x-custom-element x="234" y={5}>x</x-custom-element>
			<Component1 number1={value} number2={value} data={data}/>
			<hr/>
			<Component1 number1={value} number2={5} data={data2}/>
			<hr/>
			<Component1 number1={x} data={data2} car={Car({
				brand: "Toyota",
				model: "Corolla",
				year: 2012
			})}/>
		</div>


	},

	'/test3': () => <input type="number" value={Promise.resolve('xy')}/>

}