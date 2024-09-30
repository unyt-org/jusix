import Component1, { Car } from "common/Component1.tsx";
import { BaseComponent } from "common/BaseComponent.tsx";
import ChildComponent from "common/ChildComponent.tsx";
import { _$method } from "datex-core-legacy/datex_short.ts";

import { renderFrontend } from "uix/base/render-methods.ts"

export default {

	'/datex-tests': () => import("./datex-tests.tsx"),

	'/test1': () => <div>Test 1</div>,
	'/test2': () => {
		const value = $$(0);

		const data = always(() => ({a: value.val}));

		console.log(data);

		const data2 = $$({a: 0});
		effect(() => {
			console.log("value:", value.val)
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
			
			<input class={({s:true})} type="number" value={'xy'}/>
			<input type={"button"} value={$("fe")} style={$$({color: 'red', x: [3]})} onclick:frontend={() => alert("Click!")}/>

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

	'/test3': () => <input type="number" value={Promise.resolve('123')}/>,

	'/test4': () => {
		const arr = $$([0]);
		setInterval(() => {
			arr.push(Math.random());
		}, 1000);

		return <div>
			<ul>
				{
					[1,2,3].map((i) => <li>{i}</li>)
				}
			</ul>
			<ul>
				{
					arr.map((i) => <li>{i}</li>)
				}
			</ul>
		</div>
	},

	'/test5': () => {
		const x = $(0);
		const y = 0;
		setInterval(() => {
			x.val++;
		});
		return <main>
			<div>{x + 1}</div>
			<div>{y + 1}</div>
			<div>{2}</div>
		</main>
	},

	'/test6': () => {
		const value = $(0);

		const data = $({a: 0});
		console.log(data);

		effect(() => {
			console.log("value = ", value.val)
			data.a = value.val;
		});

		return <div>
			<input
				value={value}
				type="text"
			/>
			
			<Component1 number1={value} number2={5} data={data}/>

		</div>
	},


	'/test7': () => {

		const x = $(0);

		effect(() => {
			console.log("x",x.val)
			x.val = x + 1;
		});
		
		return <div>
			<input value={x} type="number"/>
			X = {x}
		</div>
	},

	'/test8': () => {

		const x = $(0);
		const y = $(0);

		effect(() => {
			y.val = x + 1;
		});
		
		return <div>
			<input value={x} type="number"/>
			<div>X = { x }</div>
			<div>X + 1 = { y }</div>
			<div>X + 2 = { x + 2 }</div>
		</div>
	},

	'/test9': () => {
		const x = 0;
		const y = $$(1);
		return <div>
			<input value={y}/>
			<Component1 class="xyxyx" number1={y} number2={x + 2} data={{a:42}}/>
		</div>
	},

	'/test10': () => <main>
		<BaseComponent title="Base" color="red"/>
		<ChildComponent title="Child" color={"orange"} counter={$$(3)}/>
	</main>,

	'/test11': () => {
		const radius = $(0);

		return (
			<div>
				<h1>Circle Area Calculator</h1>
				<input step="0.01" type="number" placeholder="Radius" value={radius}/>
				<p>Area = { Math.PI * radius ** 2 }</p>
			</div>
		);
	},

	'/test12': () => {
		const showDialog = $(false);
		globalThis.showDialog = showDialog;
		return <div>
			<div>My Div</div>
			{val(showDialog) ? <div id="dialog">My Dialog</div> : null}
		</div>;
	},

	'/test13': () => {
		const showDialog = $(false);
		globalThis.showDialog = showDialog;
		return <div>
			<div>My Div</div>
			{val(showDialog) && <div id="dialog">My Dialog</div>}
		</div>;
	},

	'/test14': () => {
		const showDialog = $(true);
		globalThis.showDialog = showDialog;
		return <div>
			<div>My Div</div>
			{val(showDialog) && <div id="dialog">My Dialog</div>}
		</div>;
	},

	'/test15': () => {
		const showDialog = $(false);
		globalThis.showDialog = showDialog;
		return <div>
			<div>My Div</div>
			{val(showDialog) ? <div id="dialog">My Dialog</div> : "no dialog!"}
		</div>;
	},

	'/test16': () => {
		const showDialog = $(false);
		globalThis.showDialog = showDialog;
		return <div>
			<div>My Div</div>
			{val(showDialog) ? <div id="dialog">My Dialog</div> : [1,2,3]}
		</div>;
	},

	'/test17': () => <input 
		type="button"
		value="Hello"
		onclick:frontend={async () => {
			const { BaseComponent } = await import("common/BaseComponent.tsx");

			const x = <BaseComponent title="x" color="red"/>;
			console.log(x);
			globalThis.alert('feef')
			alert("Hello!");
		}}
	/>,

	'/test18': () => {
		const showDialog = $(false);
		globalThis.showDialog = showDialog;
		const componentInstance = <div>Content</div>;

		return <div>
			My Div
			{toggle (showDialog, componentInstance, <div/>)}
		</div>;
	},

	'/test19': () => {
		const x = $(0);
		setInterval(() => x.val++, 1000);

		return <div>
			Content:
			<div>
				{
					renderFrontend(
						() => use(x) && <div>4 + {x} = { x + 4 }</div>,
						"Loading......"
					)
				}
			</div>
		</div>
	}
}
