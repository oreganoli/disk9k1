var m = require("mithril");
var TopBar = require("./top_bar");
var LoginForm = require("../forms/login_form");

var LoginView = {
    view: function () {
        return m("main.main_container", [m(TopBar), m(LoginForm)]);
    }
};
module.exports = LoginView;