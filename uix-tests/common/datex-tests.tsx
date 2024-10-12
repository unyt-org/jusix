export default <>
    Hello DATEX
</>


import { Datex, property } from "datex-core-legacy/datex.ts";
import { inferType, StorageSet } from 'datex-core-legacy/datex_all.ts';

const C = 0;


// @sync class A {
//     @property a!: number;
//     construct() {
//         this.a = 42;
//     }
// }
// @sync class B extends A {
//     @property b!: number;
//     construct() {
//         super.construct();
//         this.b = 69;
//     }
// }

// const list = eternalVar("tmp1-"+C) ?? $$(new StorageSet<A>());
// if (await list.getSize() === 0) {
//     await list.add(new A());
//     await list.add(new B());
// }
// for await (const entry of list) {
//     console.log(entry,entry.a, entry.b)
// }
// console.log("")


{ // Structs
	const A = struct(
		class A {
			@property a!: number;
			construct() {
                console.log("contrucst A")
				this.a = 42;
			}
		}
	)
	type A = inferType<typeof A>;
	
	const B = struct(
		class Bee extends A {
			@property b!: number;
			construct() {
				super.construct();
				this.b = 69;
			}
		}
	)
	type B = inferType<typeof B>;
	const list = eternalVar("tmp2-"+C) ?? $$(new StorageSet<A>());
	if (await list.getSize() === 0) {
		await list.add(new A());
		await list.add(new B());
	}
	for await (const entry of list) {
		console.log(entry, entry.a, entry.b)
	}
}