// looter.js by @mageowl
import * as robot from "../../userlib/robot.ts";

function dirToTangent(dir) {
  switch (dir) {
    case "up": return [-1, 0];
    case "down": return [1, 0];
    case "left": return [0, 1];
    case "right": return [0, -1];
  }
}

let direction = "right";

while (true) {
  switch (direction) {
    case "up": direction = "left"; break;
    case "left": direction = "down"; break;
    case "down": direction = "right"; break;
    case "right": direction = "up"; break;
  }
  
  // Move up to find a wall
  while (await robot.scan(...robot.dirToCoords(direction)) == "empty") {
    await robot.move(direction);
  }

  // Scan the board for a target
  let i = 0;
  let targetX, targetY;

  outer: while (true) {
    if (await robot.scan(...dirToTangent(direction)) == "empty") {
      switch (direction) {
        case "left": direction = "down"; break;
        case "down": direction = "right"; break;
        case "right": direction = "up"; break;
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

    await robot.move(direction);

    // Every 7 steps
    if (i % 7 == 0) {
      let wallUp = await robot.scan(0, -1) === "wall";
      let wallDown = await robot.scan(0, 1) === "wall";
      let wallLeft = await robot.scan(-1, 0) === "wall";
      let wallRight = await robot.scan(1, 0) === "wall";

      for (let x = -3; x <= 3; x++) {
        if (x < 0 && wallLeft) continue;
        if (x > 0 && wallRight) continue;
        for (let y = -3; y <= 3; y++) {
          if (y < 0 && wallUp) continue;
          if (y > 0 && wallDown) continue;
          if (x == 0 && y == 0) continue;

          if (await robot.scan(x, y) === "coins") {
            targetX = x;
            targetY = y;
            break outer;
          }
        }
      }
    }

    i++;
  }

  // Chase down the target
  while (targetX !== 0) {
    if (targetX < 0) {
      targetX += 1;
      await robot.move("left");
    } else {
      targetX -= 1;
      await robot.move("right");
    }
  }
  while (targetY !== 0) {
    if (targetY < 0) {
      targetY += 1;
      await robot.move("up");
    } else {
      targetY -= 1;
      await robot.move("down");
    }
  }
}
