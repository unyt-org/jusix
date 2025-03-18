const y = <div data={_$(()=>true ? <div style={$("color:greenreactive")}>Active</div> : <Test style={$("reactive!")}>Not active</Test>)}>
	</div>;
export default <Example num={_$(()=>reactive())} bool={static()} user={_$(()=>reactive())}/>;
