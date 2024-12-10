/**
 * Not 100% finished yet
 * 
 * Current implementation
 * Input:
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain 1 dotted black bags.
dotted black bags contain no other bags.
 * 
 * Output:
[
  {
    "amount": 1,
    "bagName": "dark olive",
    "containingBages": [
      [
        {
          "amount": 3,
          "bagName": "faded blue",
          "containingBages": [
            [
              {
                "amount": 1,
                "bagName": "dotted black",
                "containingBages": []
              }
            ]
          ]
        },
        {
          "amount": 4,
          "bagName": "dotted black",
          "containingBages": [
            [
              {
                "amount": 1,
                "bagName": "dotted black",
                "containingBages": []
              }
            ]
          ]
        }
      ],
      [
        {
          "amount": 5,
          "bagName": "faded blue",
          "containingBages": [
            [
              {
                "amount": 1,
                "bagName": "dotted black",
                "containingBages": []
              }
            ]
          ]
        },
        {
          "amount": 6,
          "bagName": "dotted black",
          "containingBages": [
            [
              {
                "amount": 1,
                "bagName": "dotted black",
                "containingBages": []
              }
            ]
          ]
        }
      ]
    ]
  },
  {
    "amount": 2,
    "bagName": "vibrant plum",
    "containingBages": [
      [
        {
          "amount": 3,
          "bagName": "faded blue",
          "containingBages": [
            [
              {
                "amount": 1,
                "bagName": "dotted black",
                "containingBages": []
              }
            ]
          ]
        },
        {
          "amount": 4,
          "bagName": "dotted black",
          "containingBages": [
            [
              {
                "amount": 1,
                "bagName": "dotted black",
                "containingBages": []
              }
            ]
          ]
        }
      ],
      [
        {
          "amount": 5,
          "bagName": "faded blue",
          "containingBages": [
            [
              {
                "amount": 1,
                "bagName": "dotted black",
                "containingBages": []
              }
            ]
          ]
        },
        {
          "amount": 6,
          "bagName": "dotted black",
          "containingBages": [
            [
              {
                "amount": 1,
                "bagName": "dotted black",
                "containingBages": []
              }
            ]
          ]
        }
      ]
    ]
  }
]
 *  */ 

const input = `light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain 1 dotted black bags.
dotted black bags contain no other bags.`
  .replace(/\./g, "")
  .split("\n");

calculateAmountOfBagsThatCanGoInBagName("shiny gold");

function calculateAmountOfBagsThatCanGoInBagName(bagName) {
  const tree = createTreeOfBagsThatCanGoInBagName(bagName);
  console.log(JSON.stringify(tree)) 
}

function createTreeOfBagsThatCanGoInBagName(bagName) {
  const initialInput = getContainingBags(input.filter((line) => searchForBagName(line, bagName)));
  const recursive = initialInput.map((object) => createTreeOfBagsThatCanGoInBagName(object.bagName)).filter(value => JSON.stringify(value) !== '[]');
  initialInput.forEach(bag => bag.containingBages = recursive)
  return initialInput;
}

function searchForBagName(line, bagName) {
  return line.substring(0, bagName.length) === bagName;
}

function getContainingBags(lines) {
  const containingBags = lines.flatMap((line) =>
    line.substring(line.search("contain") + 7).split(",")
  );
  return containingBags
    .map((bag) => {
      return {
        amount: parseAmountOfBagsFromString(bag),
        bagName: parseBagNameFromString(bag),
        containingBages: []
      };
    }).filter(bag => bag.amount !== 0);
}

function parseAmountOfBagsFromString(input) {
  if (input === " no other bags") {
    return 0;
  }
  return parseInt(input.match(/ \d+ /g));
}

function parseBagNameFromString(input) {
  if (input === " no other bags") {
    return "no other bags";
  }
  return input
    .match(/ \w+ \w+ bag/g)
    .map((string) => string.substr(1, string.length - 5))[0];
}
