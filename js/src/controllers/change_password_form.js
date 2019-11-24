const m = require("mithril");
const CurrentUser = require("../models/current_user");

const ChangePasswordForm = {
    send: () => {
        m.request({method: "PUT", url: "/change_password", withCredentials: true, body: ChangePasswordForm.data})
            .then(() => {
                CurrentUser.authenticate(CurrentUser.user.name, ChangePasswordForm.data.new)
            })
            .catch((err) => alert(err))
    },
    data: {
        new: "",
        con: ""
    },
    view: () => (
        m("div", [
                m("h1", "Change your password"),
                m("form", {
                    onsubmit: (e) => {
                        e.preventDefault();
                        ChangePasswordForm.send();
                    }
                }, [
                    m("label", "New password"),
                    m("input[type=password][autocomplete=off]", {
                        oninput: (e) => {
                            ChangePasswordForm.data.new = e.target.value
                        }
                    }),
                    m("label", "Confirm new password"),
                    m("input[type=password]", {
                        oninput: (e) => {
                            ChangePasswordForm.data.con = e.target.value
                        }
                    }),
                    m("button[type=submit]", "Change your password")
                ])
            ]
        ))
};

module.exports = ChangePasswordForm;