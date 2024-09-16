import Component1 from "common/Component1.tsx";

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

		let x = 1;

		return <div>
			<input type="number" value={value}/>
			<Component1 value={value} data={data}/>
			<hr/>
			<Component1 value={value} data={data2}/>
			<hr/>
			<Component1 value={x} data={data2}/>
		</div>


	}

}