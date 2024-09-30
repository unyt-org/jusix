import { Component } from 'uix/components/Component.ts';

@template((props) => {
	console.warn(props);
	return <>
		<h1>{props.title}</h1>
		<div style={{color: props.color}}>
            xyz
        </div>
	</>
})
export class BaseComponent<Options> extends Component<Options & { 
	title: string,
    color: string
}> {

}