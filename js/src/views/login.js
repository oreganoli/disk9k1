const m = require("mithril");
const LoginForm = require("../controllers/login_form");
const RegisterForm = require("../controllers/register_form");
const LoginView = {
    view: () => {
        return [m(LoginForm), m("h2", "or"), m(RegisterForm)];
    }
};
module.exports = LoginView;