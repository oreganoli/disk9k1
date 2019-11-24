const m = require("mithril");
const Instance = require("../models/instance");
const LoginIndicator = require("./login_indicator");
const CurrentUser = require("../models/current_user");
const name = {
    oninit: () => {
        Instance.load();
        CurrentUser.user = CurrentUser.me()
    },
    view: () => {
        return m("a.top_item", {href: "/#!/index"}, Instance.name);
    }
};
const left_half = {
    view: () => {
        return m(".left_half", m(name));
    }
};
const right_half = {
    view: () => {
        return m(".right_half", m(LoginIndicator));
    }
};
const TopBar = {
    view: () => {
        return m(".top_bar", [m(left_half), m(right_half)]);
    }
};
module.exports = TopBar;