const m = require("mithril");
const CurrentUser = require("../models/current_user");
const ChangeEmailForm = {
    data: {
        email: ""
    },
    send: () => {
        m.request({method: "PUT", url: "/change_email", withCredentials: true, body: ChangeEmailForm.data})
            .then()
            .catch((err) => (alert(err)));
        CurrentUser.me();
    },
    view: () => (
        m("div",
            [
                m("h1", "Change your email"),
                m("form", {
                    onsubmit: (e) => {
                        e.preventDefault();
                        ChangeEmailForm.send();
                    }
                }, [
                    m("label", "New email address"),
                    m("input[type=email]", {
                        oninput: (e) => {
                            ChangeEmailForm.data.email = e.target.value;
                        }
                    }),
                    m("button[type=submit]", "Change your email")
                ]),
            ]
        )
    )
};
module.exports = ChangeEmailForm;