const m = require("mithril");
const LoginForm = require("../controllers/login_form");

const LoginView = {
    view: function () {
        return m(".main_container", m(LoginForm));
    }
};
module.exports = LoginView;