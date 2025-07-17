import { createInterface } from "node:readline/promises";
import { stdin, stdout } from "node:process";
import { once } from "node:events";

const rl = createInterface({
  input: stdin,
  output: stdout,
  terminal: false,
});

type Direction = "up" | "down" | "left" | "right";
type Tile = "empty" | "robot" | "wall" | "coins";

export async function move(dir: Direction) {
  console.log(`\0move ${dir}`);
  const [res] = await once(rl, "line");
  if (res !== "done") console.error("Failed to finish move.");
}

export async function attack(dir: Direction) {
  console.log(`\0attack ${dir}`);
  const [res] = await once(rl, "line");
  if (res !== "done") console.error("Failed to finish attack.");
}

export async function scan(x: number, y: number): Promise<Tile> {
  console.log(`\0scan ${x} ${y}`);
  const [res, tile] = (await once(rl, "line"))[0].split(" ");
  if (res !== "tile") console.error("Failed to finish scan.");
  return tile;
}

export function sleep(ms: number) {
  return new Promise((res) => {
    setTimeout(res, ms);
  });
}

export function dirToCoords(dir: Direction) {
  switch (dir) {
    case "up":
      return [0, -1];
    case "down":
      return [0, 1];
    case "left":
      return [-1, 0];
    case "right":
      return [1, 0];
  }
}
