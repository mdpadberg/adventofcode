class Ferry {
  constructor() {
    this._x = 0;
    this._y = 0;
    this._wayPointX = 10;
    this._wayPointY = 1;
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
          this._wayPointX += amount;
          break;
        case "N":
          this._wayPointY += amount;
          break;
        case "W":
          this._wayPointX -= amount;
          break;
        case "S":
          this._wayPointY -= amount;
          break;
        default:
          console.log("instruction unknown: ", instruction);
          break;
      }
    }
  }

  turn(instruction, amount) {
    switch (instruction) {
      case "L":
        const leftTurn = this.calculateLeftTurn(amount);
        this._wayPointX = leftTurn.xTurn;
        this._wayPointY = leftTurn.yTurn;
        break;
      case "R":
        const rightTurn = this.calculateRightTurn(amount);
        this._wayPointX = rightTurn.xTurn;
        this._wayPointY = rightTurn.yTurn;
        break;
      case "F":
        this._x += this._wayPointX * amount;
        this._y += this._wayPointY * amount;
        break;
      default:
        console.log("instruction unknown: ", instruction);
        break;
    }
  }

  calculateLeftTurn(amount) {
    return this.calculateTurn(amount);
  }

  calculateRightTurn(amount) {
    return this.calculateTurn(amount * -1);
  }

  calculateTurn(amount) {
    const angle = (amount * Math.PI) / 180;
    return {
      xTurn: Math.round(
        this._wayPointX * Math.cos(angle) - this._wayPointY * Math.sin(angle)
      ),
      yTurn: Math.round(
        this._wayPointX * Math.sin(angle) + this._wayPointY * Math.cos(angle)
      ),
    };
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
