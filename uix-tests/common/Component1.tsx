import { Component } from 'uix/components/Component.ts';

@template(({value, data}) => {
	console.log("value", value.val)
	return <>
		<div>Current value: <b>{value}</b></div>
		<div>Current value * 10: <b>{value * 10}</b></div>
		<div>A: {data.a}</div>
	</>
})
export default class Component1 extends Component<{ value: number, data: {a: number} }> {

}