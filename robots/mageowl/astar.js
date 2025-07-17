// astar.js by @mageowl
// https://mageowl.dev
import * as robot from "../../userlib/robot.ts";

let x = 0;
let y = 0;
async function move(dir) {
  let [dx, dy] = robot.dirToCoords(dir);
  if (await robot.scan(dx, dy) === "empty") {
    x += dx;
    y += dy;
    await robot.move(dir);
    return true
  } else return false
}

while (await move("up")) {}
while (await move("left")) {}

console.log("done")
