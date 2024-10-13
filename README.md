# JUSIX

UIX uses [JUSIX](https://github.com/unyt-org/jusix), which handles the interpretation of JSX code as reactive JavaScript.

UIX uses a custom version of Deno as backend runtime called [Deno for UIX](https://github.com/unyt-org/deno). JUSIX is integrated into the [`deno_ast` parser](https://github.com/unyt-org/deno_ast) of the custom Deno build, which allows for running JSX expressions as reactive code. JUSIX does also work for frontend *(browser)* code by transpiling the frontend modules to plain JavaScript using SWC with a [JUSIX WASM plugin](https://github.com/unyt-org/jusix/tree/wasm-plugin) enabled. That allows the browser to treat reactivity the same way as the backend does. Additionally SWC does handle the conversion of TypeScript and JSX *(TS/TSX)* into plain JavaScript as browser have no native support for TypeScript nor JSX.

UIX can automatically wrap certain expressions in `always` calls. This eliminates the need for developers to write `always` explicitly every time they want reactivity. 


## Workwise

DATEX introduces the `_$` method, which is essentially a shorthand for `always`. It comes with optimizations and performance enhancements tailored to JSX.

For instance, JSX expressions like:
```tsx
<p>Counter + 1 = {counter + 1}</p>;
```

are transpiled by JUSIX into JavaScript code that looks like that:


```tsx
<p>Counter + 1 = {_$(() => counter + 1)}</p>;
```

### Reactivity examples

Reactive tenary statements to allow for updating the DOMs children based on conditions can be written like this:
```tsx
const isLoggedIn = $(false);
<div>
  <button onclick={() => isLoggedIn.val = true}>Click to login!</button>
  {
      isLoggedIn ? 
          <HelloComponent/> : 
          <span>Please login first</span>
  }
</div>;
```

Above code is transpiled to something like:

```tsx
const isLoggedIn = $(false);
<div>
  <button onclick={() => isLoggedIn.val = true}>Click to login!</button>
  {
      _$(() => isLoggedIn ? 
          <HelloComponent/> : 
          <span>Please login first</span>)
  }
</div>;
```

#### Reactivity for attributes
The reactivity does not only work for HTML children or content but also for HTML attribute values:

```tsx
const counter = $(0);
<button
  value={'Clicked:' + myValue}
  onclick={() => counter.val++}/>;
```

is transpiled to:

```tsx
const counter = $(0);
<button
  value={_$(() => 'Clicked:' + myValue)}
  onclick={() => counter.val++}/>;
```


### Reactive properties
To improve performance when updating properties of complex objects, such as arrays or JavaScript objects, DATEX propagates updates for an object's pointer properties. JUSIX will optimize the handling of the updates to use special accessors instead of the `always` call.

Properties of an object can be accessed using the `prop(ref, key)` call.

```tsx
const myForm = $({name: 'John'});
<input value={myForm.name}/>;
```

will transpile to:

```tsx
<input value={prop(myForm, 'name')}/>;
```

This will also work when using nested property access such as `myComplexForm.user.name` and transpile to something like:
```tsx
<input value={prop(prop(myComplexForm, 'user'), 'name')}/>;
```

---

<sub>&copy; unyt 2024 • [unyt.org](https://unyt.org)</sub>

