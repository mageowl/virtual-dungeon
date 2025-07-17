// scanner.js by @mageowl
// https://mageowl.dev

import * as robot from "../../userlib/robot.ts";

function dirToTangent(dir) {
  switch (dir) {
    case "up": return [-1, 0];
    case "down": return [1, 0];
    case "left": return [0, 1];
    case "right": return [0, -1];
  }
}

while (true) {
  // Move up to find a wall
  console.log("Stage 1 started");

  while (await robot.scan(0, -1) == "empty") {
    await robot.move("up");
  }

  // Scan the board for a target
  console.log("Stage 2 started");

  let direction = "right";
  let i = 0;
  let targetX, targetY;

  while (targetX == null) {
    await robot.move(direction);

    if (await robot.scan(...dirToTangent(direction)) == "empty") {
      switch (direction) {
        case "right": direction = "up"; break;
        case "down": direction = "right"; break;
        case "left": direction = "down"; break;
        case "up": direction = "left"; break;
      }
    } else if (await robot.scan(...robot.dirToCoords(direction)) != "empty") {
      switch (direction) {
        case "right": direction = "down"; break;
        case "down": direction = "left"; break;
        case "left": direction = "up"; break;
        case "up": direction = "right"; break;
      }
    }

    // Every 7 steps
    if (i % 7 == 0) {
      for (let x = -2; x <= 2; x++) {
        for (let y = -2; y <= 2; y++) {
          if (x == 0 && y == 0) continue;
          switch (await robot.scan(x, y)) {
            case "robot":
              targetX = x;
              targetY = y;
              break;
          }
        }
      }
    }

    i++;
  }

  // Chase down the target
  console.log("Stage 3 started");

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
      await robot.attack("up");
    } else if (targetY === 1) {
      await robot.attack("down");
    } else if (targetX === -1) {
      await robot.attack("left");
    } else {
      await robot.attack("right");
    }
    
    // Make sure we actually killed the target
    targetX = null;
    for (let x = -1; x <= 1; x++) {
      for (let y = -1; y <= 1; y++) {
        if (x == 0 && y == 0) continue;
        switch (await robot.scan(x, y)) {
          case "robot":
            targetX = x;
            targetY = y;
            break;
        }
      }
    }
  }
}
