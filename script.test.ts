import { describe, expect, test, stringMatching } from "bun:test";

import * as wasm from "./pkg/node/wasm";
import { task_new } from "./pkg/node/wasm_bg.wasm";

describe("wasm", () => {
    test("fib", () => {
        expect(wasm.fib(5)).toBe(8);
        for (let i = 0; i < 10; i++) {
            console.log(wasm.fib(i));
        }
    });

    test("task", () => {
        const task = new wasm.Task(42, "write a blog post");
        expect(task.id).toBe(42);
        expect(task.description).toBe("write a blog post");
    });

    test("sheet", () => {
        const s = new wasm.Sheet(
            Array.from({ length: 10 }, (_, row) =>
                Array.from({ length: 5 }, (_, col) => ({
                    id: [row, col],
                    value: { type: "NUMBER", value: row * col },
                }))
            )
        );
        expect(s.width).toBe(5);
        expect(s.height).toBe(10);
        expect(s.get_average(3)).toBe(27);
    });

    test("id as string", () => {
        const id: wasm.Id = wasm.make_id();
        console.log(id);
        expect(id).toMatch(/^[0-9a-f]+$/);
    });
});
