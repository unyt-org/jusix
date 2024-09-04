const x = <div>
		<input value={_$(()=>count + '2')}/>
        <div>{_$(()=>x + y)}</div>
        <div>
            <div id={_$(()=>x + y)}>
                <span class="static">{_$(()=>x + y)}</span>
            </div>
        </div>
	</div>;
