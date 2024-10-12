run(()=>{
    use("silent-errors", y, console, ParentClass, alert, ParentClass2);
    const x1 = y as Z1;
    const x2 = y satisfies Z2;
    const x3: Z3 = y;
    const x4: {
        _x: Z4;
    } = y;
    interface Interface {
        x: Z5;
    }
    enum Color {
        Red = 1,
        Green = 2,
        Blue = 3
    }
    console.log(Color.Red);
    class MyClass extends ParentClass {
        x: T;
        y = this.x;
        method(methodParam: Z6) {
            super.method(methodParam);
            console.log(this);
            alert(1);
        }
        static staticMethod(staticMethodParam: Z7) {
            alert(2);
        }
        set setter(setterParam: Z8) {
            alert(3);
        }
        static set staticSetter(staticSetterParam: Z9) {
            alert(4);
        }
        #privateMethod(privateMethodParam: Z10) {
            alert(5);
        }
        static #privateStaticMethod(privateStaticMethodParam: Z11) {
            alert(6);
        }
    }
    class MyClass2<T> extends ParentClass2<T> {
        x: T;
    }
    console.log(new MyClass<W>());
    function f(a: A, b: B) {
        console.log(this);
        return a + b;
    }
});
