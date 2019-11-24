const m = require("mithril");
const LoginForm = require("../controllers/login_form");

const LoginView = {
    view: () => {
        return [m(LoginForm)];
    }
};
module.exports = LoginView;