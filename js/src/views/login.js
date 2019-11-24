var m = require("mithril");
var TopBar = require("./top_bar");
var LoginForm = require("../controllers/login_form");

var LoginView = {
    view: function () {
        return m(".main_container", m(LoginForm));
    }
};
module.exports = LoginView;