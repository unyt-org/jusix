const x = <div a0={"5"} a1={true}>
        <span a2={_$(()=>reactive())}>Test</span>
    </div>;
export default <Test a3={true} a4={_$(()=>makeReactive())} a5={<div a6={$(reactive)}>Test</div>}>
        <span>{x}</span>
        <span a7={$(reactiveVar)}>
            Test
            <div a8={x + 1}>Test</div>
        </span>
        <div a9={$(true)}>Test</div>
    </Test>;
