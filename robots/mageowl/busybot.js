
// busybot.js by @mageowl
// https://mageowl.dev

import * as robot from "../../userlib/robot.ts";

let targetX, targetY;

function dirToTangent(dir) {
  switch (dir) {
    case "up": return [-1, 0];
    case "down": return [1, 0];
    case "left": return [0, 1];
    case "right": return [0, -1];
  }
}

// copied from radar.js
async function chase() {
  // Chase down the target
  console.log("chasing");

  outer: while (targetX != null) {
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
          continue outer;
        }
      }
    }

    for (let x = -2; x <= 2; x++) {
      for (let y = -2; y <= 2; y++) {
        if (Math.abs(x) < 2 && Math.abs(y) < 2) continue;
        if (await robot.scan(x, y) === "robot") {
          targetX = x;
          targetY = y;
          continue outer;
        }
      }
    }
  }
}

async function trace(length) {
  let direction;

  if (robot.isBlocking(await robot.scan(1, 0))) {
    direction = "down";
  } else if (robot.isBlocking(await robot.scan(-1, 0))) {
    direction = "up";
  } else if (robot.isBlocking(await robot.scan(0, 1))) {
    direction = "left";
  } else if (robot.isBlocking(await robot.scan(0, -1))) {
    direction = "right";
  } else {
    direction = "right";
  }

  let i = 0;
  while (i < length) {
    if (!robot.isBlocking(await robot.scan(...dirToTangent(direction)))) {
      switch (direction) {
        case "right": direction = "up"; break;
        case "down": direction = "right"; break;
        case "left": direction = "down"; break;
        case "up": direction = "left"; break;
      }
    } else if (robot.isBlocking(await robot.scan(...robot.dirToCoords(direction)))) {
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
      for (let x = -2; x <= 2; x++) {
        for (let y = -2; y <= 2; y++) {
          if (x == 0 && y == 0) continue;
          switch (await robot.scan(x, y)) {
            case "robot":
              targetX = x;
              targetY = y;
              break;
            case "coins":
              while (Math.abs(x) > 0) {
                await robot.move(["left", "right"][Math.sign(x) / 2 + .5]);
                x -= Math.sign(x);
              }
              while (Math.abs(y) > 0) {
                await robot.move(["up", "down"][Math.sign(y) / 2 + .5]);
                y -= Math.sign(y);
              }

              switch (direction) {
                case "right": direction = "up"; break;
                case "down": direction = "right"; break;
                case "left": direction = "down"; break;
                case "up": direction = "left"; break;
              }
              while (!robot.isBlocking(await robot.scan(...robot.dirToCoords(direction)))) {
                await robot.move(direction);
              }
              switch (direction) {
                case "right": direction = "down"; break;
                case "down": direction = "left"; break;
                case "left": direction = "up"; break;
                case "up": direction = "right"; break;
              }

              break;
          }
        }
      }
    }

    i++;
  }
}

let numWanders = 0;
async function wander() {
  numWanders++;

  if (numWanders === 5) {
    numWanders = 0;
    return trace(50);
  }
  
  let xDir = Math.round(Math.random());
  let yDir = Math.round(Math.random());

  if (robot.isBlocking(await robot.scan([-1, 1][xDir], 0))) {
    xDir = [1, 0][xDir];
  }
  if (robot.isBlocking(await robot.scan(0, [-1, 1][yDir]))) {
    yDir = [1, 0][yDir];
  }

  while (true) {
    await robot.move(["left", "right"][xDir]);
    await robot.move(["up", "down"][yDir]);

    for (let x = -3; x <= 3; x++) {
      for (let y = -3; y <= 3; y++) {
        if (y !== [-3, 3][yDir] && x !== [-3, 3][xDir]) {
          continue;
        }

        let scan = await robot.scan(x, y);
        if (scan === "coins") {
          while (Math.abs(x) > 0) {
            await robot.move(["left", "right"][Math.sign(x) / 2 + .5]);
            x -= Math.sign(x);
          }
          while (Math.abs(y) > 0) {
            await robot.move(["up", "down"][Math.sign(y) / 2 + .5]);
            y -= Math.sign(y);
          }
        } else if (scan === "robot") {
          targetX = x;
          targetY = y;
          await chase();
        }
      }
    }

    if (robot.isBlocking(await robot.scan([-1, 1][xDir], [-1, 1][yDir]))) {
      break;
    }
  }
}

async function scan() {
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
  
  return [xPositions, yPositions];
}

while (true) {
  // Move up to find a wall
  console.log("Stage 1 started");

  let direction = ["up", "down", "left", "right"][Math.floor(Math.random() * 4)];
  while (!robot.isBlocking(await robot.scan(...robot.dirToCoords(direction)))) {
    await robot.move(direction);
  }

  // Scan the board for a target
  console.log("Stage 2 started");
  let [xPositions, yPositions] = await scan();
  
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

    if (Math.random() < 0.08) {
      console.log("Wandering...");
      await wander();
      let [nxp, nyp] = await scan();
      xPositions = nxp;
      yPositions = nyp;
    }
  }

  await chase();
}
