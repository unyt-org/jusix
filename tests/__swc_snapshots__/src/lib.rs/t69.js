const y = <div>
		{_$(()=>true ? <div stylereactive={$("color:green")}>Active</div> : <div stylereactive={$("color:red")}>Not active</div>)}
	</div>;
export default <Example num={static()} bool={_$(()=>reactive())} user={static()}/>;
