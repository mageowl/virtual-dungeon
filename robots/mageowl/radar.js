// radar.js by @mageowl
// https://mageowl.dev

import * as robot from "../../userlib/robot.ts";

let targetX, targetY;

async function chase() {
  // Chase down the target
  console.log("chasing");

  while (targetX != null) {
    while (targetX !== 0) {
      if (targetX < 0) {
        targetX += 1;
        await robot.move("left");
      } else {
        targetX -= 1;
        await robot.move("right");
      }
    }
    while (Math.abs(targetY) > 1) {
      if (targetY < -1) {
        targetY += 1;
        await robot.move("up");
      } else {
        targetY -= 1;
        await robot.move("down");
      }
    }

    if (targetY === -1) {
      if (await robot.scan(0, -1) === "robot")
        await robot.attack("up");
    } else if (targetY === 1) {
      if (await robot.scan(0, 1) === "robot")
        await robot.attack("down");
    } else if (targetX === -1) {
      if (await robot.scan(-1, 0) === "robot")
        await robot.attack("left");
    } else {
      if (await robot.scan(1, 0) === "robot")
        await robot.attack("right");
    }
    
    // Make sure we actually killed the target
    targetX = null;
    for (let x = -1; x <= 1; x++) {
      for (let y = -1; y <= 1; y++) {
        if (x == 0 && y == 0) continue;
        if (await robot.scan(x, y) === "robot") {
          targetX = x;
          targetY = y;
          break;
        }
      }
    }
  }
}

while (true) {
  // Move up to find a wall
  console.log("Stage 1 started");

  let direction = ["up", "down", "left", "right"][Math.floor(Math.random() * 4)];
  while (await robot.scan(...robot.dirToCoords(direction)) === "empty") {
    await robot.move(direction);
  }

  // Scan the board for a target
  console.log("Stage 2 started");


  let xPositions = [];
  let yPositions = [];
  for (let x = -3; x <= 3; x++) {
    if (Math.abs(x) === 3 && await robot.scan(Math.sign(x), 0) === "wall") continue;
    for (let y = -3; y <= 3; y++) {
      if (Math.abs(y) === 3 && await robot.scan(0, Math.sign(y)) === "wall") continue;
      if (x == 0 && y == 0) continue;

      let scan = await robot.scan(x, y); 
      if (scan === "robot") {
        targetX = x;
        targetY = y
        await chase();
      }
      if (scan !== "wall" && (Math.abs(x) === 3 || Math.abs(y) === 3)) {
        
        xPositions.push(x);
        yPositions.push(y);
      }
    }
  }
  
  while (targetX == null) {
    for (let i = 0; i < xPositions.length; i++) {
      let x = xPositions[i];
      let y = yPositions[i];
      if (await robot.scan(x, y) === "robot") {
        targetX = x;
        targetY = y;
        break;
      }
    }
  }

  await chase();
}
