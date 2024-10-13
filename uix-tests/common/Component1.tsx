import { Ref } from "datex-core-legacy/runtime/pointers.ts";
import { inferType } from "datex-core-legacy/types/struct.ts";
import { toHashString } from "https://deno.land/std@0.164.0/crypto/util.ts";
import { Component } from 'uix/components/Component.ts';

export const Car = struct({
	brand: string,
	model: string,
	year: number
});
type Car = inferType<typeof Car>;


function test() {
	alert("test 1");
}

@template(function ({number1, number2, boolean, data, data2, car}) {
	console.warn("values", number1, number2, boolean, car, data, data2?.$.a, car?.$.brand);
	return <>
		{/* <div>Current value: <b>{number1}</b></div> */}
		<div>Current value * 10: <b>{number1 * 10}</b></div>
		<div>A: {data.a}</div>
		<button onclick:frontend={() => globalThis.console.log(this.properties.number1.val)}>CLICK</button>
		<button onclick:frontend={ () => {
	alert("test 1");
}}>Test</button>
	</>
})
export default class Component1 extends Component<{ 
	number1: Ref<number>,
	number2?: number,
	boolean?: Ref<boolean>,
	data: {a: number}, 
	data2?: Ref<{a: number}>,
	car?: Car 
}> {

	test() {
		alert("test");
	}

	protected override onCreate(): void | Promise<void> {
		console.log("created",this.properties,this.properties.number1);
	}

	protected override onDisplay() {
		console.log("props",this.options,this.properties,this.properties.number1);
	}
}