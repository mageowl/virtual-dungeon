import { createInterface } from "node:readline/promises";
import { stdin, stdout } from "node:process"; 
import { once } from "node:events";

const rl = createInterface({
  input: stdin,
  output: stdout,
  terminal: false,
});

export async function move(dir) {
  console.log(`\0move ${dir}`);
  let [res] = await once(rl, "line");
  if (res != "done") console.error("Failed to finish move.");
}

export async function attack(dir) {
  console.log(`\0attack ${dir}`);
  let [res] = await once(rl, "line");
  if (res != "done") console.error("Failed to finish attack.");
}

export async function scan(x, y) {
  console.log(`\0scan ${x} ${y}`);
  let [res, tile] = (await once(rl, "line"))[0].split(" ");
  if (res != "tile") console.error("Failed to finish scan.");
  return tile;
}

export function dirToCoords(dir) {
  switch (dir) {
    case "up": return [0, -1];
    case "down": return [0, 1];
    case "left": return [-1, 0];
    case "right": return [1, 0];
  }
}
