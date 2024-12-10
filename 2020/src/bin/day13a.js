const input = parseInputFile(require("fs").readFileSync("adventofcode-private/data/2020/13.txt", "utf8"));

console.log(findFirstBus(input.timestamp, input.buses));

function parseInputFile(file) {
  const fileByLine = file.split("\n");
  return {
    timestamp: fileByLine[0],
    buses: fileByLine[1]
      .split(/[,|x,]/)
      .filter((nonBlank) => nonBlank)
      .map((time) => Number(time)),
  };
}

function findFirstBus(timestamp, buses) {
  const max = Math.max(...buses);
  for (let i = timestamp; i <= timestamp + max; i++) {
    for (const bus of buses) {
      if (!(i % bus)) {
        return (i - timestamp) * bus;
      }
    }
  }
}
