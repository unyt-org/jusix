import { PointerWithPrimitive, Ref } from "datex-core-legacy/runtime/pointers.ts";
import { inferType } from "datex-core-legacy/types/struct.ts";
import { Component } from 'uix/components/Component.ts';

export const Car = struct({
	brand: string,
	model: string,
	year: number
});
type Car = inferType<typeof Car>;

@template(({number1, number2, boolean, data, data2, car}) => {
	console.log("value", number1.val, car, data.a, data2?.$.a, car?.$.brand);
	return <>
		<div>Current value: <b>{number1}</b></div>
		<div>Current value * 10: <b>{number1 * 10}</b></div>
		<div>A: {data.a}</div>
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

}