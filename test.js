console.log("i am going to move down");

setInterval(() => {
  let directions = ["up", "down", "left", "right"];
  let idx = Math.floor(Math.random() * directions.length);
  console.log(`\0move ${directions[idx]}`);
}, 1000);
