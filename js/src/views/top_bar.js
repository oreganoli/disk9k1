var m = require("mithril");
var Instance = require("../models/instance");
var LoginIndicator = require("./login_indicator");
var CurrentUser = require("../models/current_user");
var name = {
    oninit: function () {
        Instance.load();
        CurrentUser.user = CurrentUser.me()
    },
    view: function () {
        return m("b", m("a", {href: "/#!/index"}, Instance.name));
    }
};
var left_half = {
    view: function () {
        return m(".left_half", m(name));
    }
};
var right_half = {
    view: function () {
        return m(".right_half", m(LoginIndicator));
    }
};
var TopBar = {
    view: function () {
        return m(".top_bar", [m(left_half), m(right_half)]);
    }
};
module.exports = TopBar;