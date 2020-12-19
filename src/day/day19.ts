import * as fs from "fs";

class Mut<T> {
    constructor(public val: T) {

    }
}

class Rule {
    constructor(
        public number: number,
        public sequences: IterableIterator<Sequence>,
    ) {}

    static parse(s: string) {
        // split by ": "
        let itr = s.split(": ");

        // left is number
        let number_str = itr[0];

        // right split by | to get a list of subrules strings
        let sequences_strs = itr[1].split(" | ");

        const number = parseInt(number_str);
        const sequences = sequences_strs.map(sequence_parse)[Symbol.iterator]();

        return new Rule(number, sequences);
    }

    pass(s: Mut<string>, rules: Map<number, Rule>): boolean {
        // return true if any sub rule passes

        for (const seq of this.sequences) {
            let safely_modifiable_s = new Mut(s.val);
            if (sequence_pass(seq, safely_modifiable_s, rules)) {
                s.val = safely_modifiable_s.val;
                return true;
            }
        }

        return false;
    }
}


console.log(day19());

function day19() {
    let file = fs.readFileSync("input/day19.txt").toString();

    console.log("Part 2: ", part2(file));
}

function part2(input: string): number {
    let itr = input.split("\n\n");
    let rules_str = itr[0];
    let messages_str = itr[1];

    let rules = rules_str.split("\n").map(Rule.parse);

    // create a hashmap of rule numbers to rules
    let rules_map = new Map<number, Rule>();
    for (const rule of rules) {
        rules_map.set(rule.number, rule);
    }

    // now modify the incorrect rules and put them in the map
    // overwriting the old rules where necessary

    /// ???????????????

    const poop = messages_str
        .split("\n")
        .filter(message => {
            let s = new Mut(message);
            let rule_passes = (rules_map.get(0) as Rule).pass(s, rules_map);

            // rule passes and there's no more characters that need checking
            return rule_passes && s.val.length == 0;
        })
    return [...poop].length;
}

type Step = number | string;

function step_parse(s: string): Step {
    // parse int
    // success: Symbol
    // fail: take 2nd char from string, return Char

    let num = parseInt(s);

    if (!Number.isNaN(num)) {
        return num;
    } else {
        return s[0];
    }
}

function step_pass(self: Step, s: Mut<string>, rules: Map<number, Rule>): boolean {
    // if I want a char, check the char
    // else call pass on the SubRule referenced

    if (typeof self === "number") {
        return (rules.get(self) as Rule).pass(s, rules);

    } else {
        console.log(self, s);
        if (s.val.length > 0) {
            let c = s.val[0];
            s.val = s.val.slice(1);
            return c == self;
        } else {
            return false;
        }
    }
}


type Sequence = IterableIterator<Step>;

function sequence_parse(s: string): Sequence {
    // split each subrule string by ' ' to get a list of step strings
    // parse string into step
    return s.split(' ').map(step_parse)[Symbol.iterator]();
}

function sequence_pass(sequence: Sequence, s: Mut<string>, rules: Map<number, Rule>): boolean {
    // return true if all steps pass

    for (const step of sequence) {
        if (!step_pass(step, s, rules)) {
            return false;
        }
    }

    return true;
}
