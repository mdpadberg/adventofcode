class Grid {
  constructor(values) {
    this._values = values;
  }

  /**
   * Grid data should be:
   * L.LL.LL.LL\n
   * LLLLLLL.LL\n
   * L.L.L..L..
   */
  static createGrid(input) {
    const dataLineByLine = input.split("\n");
    const grid = [];
    for (let y = 0; y < dataLineByLine.length; y++) {
      const row = [];
      for (let x = 0; x < dataLineByLine[y].length; x++) {
        row[x] = dataLineByLine[y].charAt(x);
      }
      grid[y] = row;
    }
    return grid;
  }

  getCell(y, x) {
    const indexOutOfBound =
      y < 0 ||
      y > this._values.length - 1 ||
      x < 0 ||
      x > this._values.length - 1;
    return indexOutOfBound ? undefined : this._values[y][x];
  }

  setCell(y, x, value) {
    this._values[y][x] = value;
  }

  getCoordinatesOfCellsWithValue(value) {
    const cellsWithValue = [];
    for (let y = 0; y < this._values.length; y++) {
      const row = this._values[y];
      if (row.includes(value)) {
        for (let x = 0; x < row.length; x++) {
          if (row[x] === value) {
            cellsWithValue.push({
              y: parseInt(y),
              x: parseInt(x),
            });
          }
        }
      }
    }
    return cellsWithValue;
  }

  getNeighborsIgnoreFloorSpaces(y, x) {
    const neighborCoordinates = {
      topLeft: { y: -1, x: -1 },
      top: { y: -1, x: 0 },
      topRight: { y: -1, x: 1 },
      left: { y: 0, x: -1 },
      right: { y: 0, x: 1 },
      bottomLeft: { y: 1, x: -1 },
      bottom: { y: 1, x: 0 },
      bottomRight: { y: 1, x: 1 },
    };
    const neighbors = [];
    for (let i = 0; i < Object.keys(neighborCoordinates).length; i++) {
      const object = neighborCoordinates[Object.keys(neighborCoordinates)[i]];
      for (let loopY = y, loopX = x; ; ) {
        loopY += object.y;
        loopX += object.x;
        const cell = this.getCell(loopY, loopX);
        if (!cell) {
          break;
        }
        if (cell !== FLOOR) {
          neighbors.push(cell);
          break;
        }
      }
    }
    return neighbors;
  }

  /**
   * Array.from(..) only does a shallow copy of multi-dimensional arrays
   * Spread operator [...this._values] only does a shallow copy of multi-dimensional arrays
   */
  getValues() {
    return JSON.parse(JSON.stringify(this._values));
  }

  printGrid() {
    let rows = "";
    for (let y = 0; y < this._values.length; y++) {
      let row = "";
      for (let x = 0; x < this._values[y].length; x++) {
        row += this.getCell(y, x);
      }
      rows += row + "\n";
    }
    console.log(rows);
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////

const input = require("fs").readFileSync("adventofcode-private/data/2020/11.txt", "utf8");
const EMPTY = "L";
const OCCUPIED = "#";
const FLOOR = ".";
const startingGrid = new Grid(Grid.createGrid(input));

loadFerry(startingGrid);

function loadFerry(startingGrid) {
  let newGrid = new Grid(startingGrid.getValues());
  while(true) {
    const firstRun = occupyEmptySeats(newGrid);
    const secondRun = checkIfOccupiedSeatsBecomesEmptyAgain(firstRun);
    newGrid = secondRun;
    if(JSON.stringify(firstRun.getValues()) === JSON.stringify(secondRun.getValues())) {
        break;
    }
  } 
  console.log(newGrid.getValues().flat().filter((seats) => seats === OCCUPIED).length)
}

function occupyEmptySeats(inputGrid) {
    const filterFunction = function(value) {
        return value === 0
    };
    return loopThroughSeats(inputGrid, EMPTY, OCCUPIED, filterFunction);
}

function checkIfOccupiedSeatsBecomesEmptyAgain(inputGrid) {
    const filterFunction = function(value) {
        return value >= 5;
    };
    return loopThroughSeats(inputGrid, OCCUPIED, EMPTY, filterFunction);
}

function loopThroughSeats(inputGrid, oldSeatStatus, newSeatStatus, filterFunction) {
  const newGrid = new Grid(inputGrid.getValues());
  const seats = newGrid.getCoordinatesOfCellsWithValue(oldSeatStatus);
  for (let i = 0; i < seats.length; i++) {
    const neighborsSeats = inputGrid.getNeighborsIgnoreFloorSpaces(seats[i].y, seats[i].x);
    const amountOfOccupiedNeighborSeats = neighborsSeats.filter((seats) => seats === OCCUPIED).length;
    if(filterFunction(amountOfOccupiedNeighborSeats)) {
      newGrid.setCell(seats[i].y, seats[i].x, newSeatStatus);
    }
  }
  return newGrid;
}