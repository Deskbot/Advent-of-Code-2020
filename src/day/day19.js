"use strict";
var __values = (this && this.__values) || function(o) {
    var s = typeof Symbol === "function" && Symbol.iterator, m = s && o[s], i = 0;
    if (m) return m.call(o);
    if (o && typeof o.length === "number") return {
        next: function () {
            if (o && i >= o.length) o = void 0;
            return { value: o && o[i++], done: !o };
        }
    };
    throw new TypeError(s ? "Object is not iterable." : "Symbol.iterator is not defined.");
};
var __read = (this && this.__read) || function (o, n) {
    var m = typeof Symbol === "function" && o[Symbol.iterator];
    if (!m) return o;
    var i = m.call(o), r, ar = [], e;
    try {
        while ((n === void 0 || n-- > 0) && !(r = i.next()).done) ar.push(r.value);
    }
    catch (error) { e = { error: error }; }
    finally {
        try {
            if (r && !r.done && (m = i["return"])) m.call(i);
        }
        finally { if (e) throw e.error; }
    }
    return ar;
};
var __spread = (this && this.__spread) || function () {
    for (var ar = [], i = 0; i < arguments.length; i++) ar = ar.concat(__read(arguments[i]));
    return ar;
};
exports.__esModule = true;
var fs = require("fs");
var Mut = /** @class */ (function () {
    function Mut(val) {
        this.val = val;
    }
    return Mut;
}());
var Rule = /** @class */ (function () {
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
        var e_1, _a;
        try {
            for (var _b = __values(this.sequences), _c = _b.next(); !_c.done; _c = _b.next()) {
                var seq = _c.value;
                var safely_modifiable_s = new Mut(s.val);
                if (sequence_pass(seq, safely_modifiable_s, rules)) {
                    s.val = safely_modifiable_s.val;
                    return true;
                }
            }
        }
        catch (e_1_1) { e_1 = { error: e_1_1 }; }
        finally {
            try {
                if (_c && !_c.done && (_a = _b["return"])) _a.call(_b);
            }
            finally { if (e_1) throw e_1.error; }
        }
        return false;
    };
    return Rule;
}());
console.log(day19());
function day19() {
    var file = fs.readFileSync("input/day19.txt").toString();
    console.log("Part 2: ", part2(file));
}
function part2(input) {
    var e_2, _a;
    var itr = input.split("\n\n");
    var rules_str = itr[0];
    var messages_str = itr[1];
    var rules = rules_str.split("\n").map(Rule.parse);
    // create a hashmap of rule numbers to rules
    var rules_map = new Map();
    try {
        for (var rules_1 = __values(rules), rules_1_1 = rules_1.next(); !rules_1_1.done; rules_1_1 = rules_1.next()) {
            var rule = rules_1_1.value;
            rules_map.set(rule.number, rule);
        }
    }
    catch (e_2_1) { e_2 = { error: e_2_1 }; }
    finally {
        try {
            if (rules_1_1 && !rules_1_1.done && (_a = rules_1["return"])) _a.call(rules_1);
        }
        finally { if (e_2) throw e_2.error; }
    }
    // now modify the incorrect rules and put them in the map
    // overwriting the old rules where necessary
    /// ???????????????
    var poop = messages_str
        .split("\n")
        .filter(function (message) {
        var s = new Mut(message);
        var rule_passes = rules_map.get(0).pass(s, rules_map);
        // rule passes and there's no more characters that need checking
        return rule_passes && s.val.length == 0;
    });
    return __spread(poop).length;
}
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
        return rules.get(self).pass(s, rules);
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
    var e_3, _a;
    try {
        for (var sequence_1 = __values(sequence), sequence_1_1 = sequence_1.next(); !sequence_1_1.done; sequence_1_1 = sequence_1.next()) {
            var step = sequence_1_1.value;
            if (!step_pass(step, s, rules)) {
                return false;
            }
        }
    }
    catch (e_3_1) { e_3 = { error: e_3_1 }; }
    finally {
        try {
            if (sequence_1_1 && !sequence_1_1.done && (_a = sequence_1["return"])) _a.call(sequence_1);
        }
        finally { if (e_3) throw e_3.error; }
    }
    return true;
}
