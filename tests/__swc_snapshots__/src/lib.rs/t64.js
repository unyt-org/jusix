export function Test({ x, y, z }: {
    x: Ref<number>;
    y: number;
    z: Ref<number[]>;
    bool: Ref<boolean>;
}) {
    console.log("x", x);
    console.log("y", y);
    console.log("z", z);
    console.log("value of x", val(x));
    return <div a={"true"} b={1} c={true}>{x}</div>;
}
const obj = $({
    x: 42,
    z: [
        1,
        2,
        3
    ]
});
export default <Test x={obj.x ? 1 : 0} y={6} z={obj.z} bool={true}/>;
