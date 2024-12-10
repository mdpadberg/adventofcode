const input = require("fs")
  .readFileSync("adventofcode-private/data/2020/8.txt", "utf8")
  .split("\n")
  .map((line) => createObjectFromInput(line));

console.log(calculateAccumulator(input));

function calculateAccumulator(data) {
  let accumulator = 0;
  for (let i = 0; i < data.length; ) {
    if (data[i].visitedBefore === true) {
      return accumulator;
    } else {
      data[i].visitedBefore = true;
    }
    accumulator += adjustAccumulatorBasedOnInput(data[i].key, data[i].value);
    i += adjustCounterBasedOnInput(data[i].key, data[i].value);
  }
  return accumulator;
}

function adjustCounterBasedOnInput(key, value) {
  switch (key) {
    case "acc":
      return 1;
    case "jmp":
      return value;
    case "nop":
      return 1;
    default:
      console.log(`input ${key} is unknown`);
      return 1;
  }
}

function adjustAccumulatorBasedOnInput(key, value) {
  switch (key) {
    case "acc":
      return value;
    case "jmp":
      return 0;
    case "nop":
      return 0;
    default:
      console.log(`input ${key} is unknown`);
      return 0;
  }
}

function createObjectFromInput(data) {
  const keyValue = data.split(" ");
  return {
    key: keyValue[0],
    value: parseInt(keyValue[1]),
    visitedBefore: false,
  };
}
