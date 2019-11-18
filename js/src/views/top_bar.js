var m = require("mithril");
var Instance = require("../models/instance");

var name = {
    oninit: function () {
        Instance.load();
    },
    view: function () {
        return m("b", Instance.name);
    }
};
var left_half = {
    view: function () {
        return m(".left_half", m(name));
    }
};
var right_half = {
    view: function () {
        return m(".right_half", []);
    }
};
var TopBar = {
    view: function () {
        return m(".top_bar", [m(left_half), m(right_half)]);
    }
};
module.exports = TopBar;