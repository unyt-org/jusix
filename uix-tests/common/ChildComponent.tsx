import { BaseComponent } from "common/BaseComponent.tsx";
import { Ref } from "datex-core-legacy/runtime/pointers.ts";

@template(({counter}) => <div>{counter}</div>)
export default class ChildComponent extends BaseComponent<{ 
	counter: Ref<number>
}> {
	
	protected onDisplay(): void | Promise<void> {
	  	console.log(this.properties.title, this.properties.color, this.properties.counter)
	}
}