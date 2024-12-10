class Ferry {
  constructor() {
    this._x = 0;
    this._y = 0;
    this._degrees = 0;
  }

  static DIRECTIONS = {
    0: "E",
    1: "N",
    2: "W",
    3: "S",
  };

  execute(instruction, amount) {
    const directions = Object.values(Ferry.DIRECTIONS);
    if (!directions.includes(instruction)) {
      this.turn(instruction, amount);
    } else {
      switch (instruction) {
        case "E":
          this._x += amount;
          break;
        case "N":
          this._y += amount;
          break;
        case "W":
          this._x -= amount;
          break;
        case "S":
          this._y -= amount;
          break;
        default:
          console.log("instruction unknown: ", instruction);
          break;
      }
    }
  }

  turn(instruction, amount) {
    const amountOfDirections = Object.keys(Ferry.DIRECTIONS).length;
    switch (instruction) {
      case "L":
        this._degrees = (this._degrees + amount / 90 + amountOfDirections) % amountOfDirections;
        break;
      case "R":
        this._degrees = (this._degrees - amount / 90 + amountOfDirections) % amountOfDirections;
        break;
      case "F":
        this.execute(Ferry.DIRECTIONS[this._degrees], amount);
        break;
      default:
        console.log("instruction unknown: ", instruction);
        break;
    }
  }
}

const input = require("fs")
  .readFileSync("adventofcode-private/data/2020/12.txt", "utf8")
  .split("\n");

const ferry = new Ferry();

for (let i = 0; i < input.length; i++) {
  const instruction = input[i].charAt(0);
  const nummer = parseInt(input[i].substr(1));
  ferry.execute(instruction, nummer);
}

console.log(Math.abs(ferry._x) + Math.abs(ferry._y));
