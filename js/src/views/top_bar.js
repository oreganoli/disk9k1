const m = require("mithril");
const Instance = require("../models/instance");
const LoginIndicator = require("./login_indicator");
const CurrentUser = require("../models/current_user");
const name = {
    oninit: function () {
        Instance.load();
        CurrentUser.user = CurrentUser.me()
    },
    view: function () {
        return m("a.top_item", {href: "/#!/index"}, Instance.name);
    }
};
const left_half = {
    view: function () {
        return m(".left_half", m(name));
    }
};
const right_half = {
    view: function () {
        return m(".right_half", m(LoginIndicator));
    }
};
const TopBar = {
    view: function () {
        return m(".top_bar", [m(left_half), m(right_half)]);
    }
};
module.exports = TopBar;