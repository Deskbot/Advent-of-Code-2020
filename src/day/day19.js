var fs = require("fs");
var Mut = (function () {
    function Mut(val) {
        this.val = val;
    }
    return Mut;
})();
var Rule = (function () {
    function Rule(number, sequences) {
        this.number = number;
        this.sequences = sequences;
    }
    Rule.parse = function (s) {
        // split by ": "
        var itr = s.split(": ");
        // left is number
        var number_str = itr[0];
        // right split by | to get a list of subrules strings
        var sequences_strs = itr[1].split(" | ");
        var number = parseInt(number_str);
        var sequences = sequences_strs.map(sequence_parse)[Symbol.iterator]();
        return new Rule(number, sequences);
    };
    Rule.prototype.pass = function (s, rules) {
        // return true if any sub rule passes
        for (var _i = 0, _a = this.sequences; _i < _a.length; _i++) {
            var seq = _a[_i];
            var safely_modifiable_s = new Mut(s.val);
            if (sequence_pass(seq, safely_modifiable_s, rules)) {
                s.val = safely_modifiable_s.val;
                return true;
            }
        }
        return false;
    };
    return Rule;
})();
console.log(day19());
function day19() {
    var file = fs.readFileSync("input/day19.txt").toString();
    console.log("Part 2: ", part2(file));
}
function part2(input) {
    var itr = input.split("\n\n");
    var rules_str = itr[0];
    var messages_str = itr[1];
    var rules = rules_str.split("\n").map(Rule.parse);
    // create a hashmap of rule numbers to rules
    var rules_map = new Map();
    for (var _i = 0; _i < rules.length; _i++) {
        var rule = rules[_i];
        rules_map.set(rule.number, rule);
    }
    // now modify the incorrect rules and put them in the map
    // overwriting the old rules where necessary
    /// ???????????????
    var poop = messages_str
        .split("\n")
        .filter(function (message) {
        var s = new Mut(message);
        var rule_passes = (rules_map.get(0)), as = Rule;
    }).pass(s, rules_map);
    // rule passes and there's no more characters that need checking
    return rule_passes && s.val.length == 0;
}
return poop.slice().length;
function step_parse(s) {
    // parse int
    // success: Symbol
    // fail: take 2nd char from string, return Char
    var num = parseInt(s);
    if (!Number.isNaN(num)) {
        return num;
    }
    else {
        return s[0];
    }
}
function step_pass(self, s, rules) {
    // if I want a char, check the char
    // else call pass on the SubRule referenced
    if (typeof self === "number") {
        return (rules.get(self));
        as;
        Rule;
        pass(s, rules);
    }
    else {
        console.log(self, s);
        if (s.val.length > 0) {
            var c = s.val[0];
            s.val = s.val.slice(1);
            return c == self;
        }
        else {
            return false;
        }
    }
}
function sequence_parse(s) {
    // split each subrule string by ' ' to get a list of step strings
    // parse string into step
    return s.split(' ').map(step_parse)[Symbol.iterator]();
}
function sequence_pass(sequence, s, rules) {
    // return true if all steps pass
    for (var _i = 0; _i < sequence.length; _i++) {
        var step = sequence[_i];
        if (!step_pass(step, s, rules)) {
            return false;
        }
    }
    return true;
}
