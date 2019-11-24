const m = require("mithril");
const CurrentUser = require("../models/current_user");

const DelAccountForm = {
    data: {
        id: 0
    },
    oninit: () => {
        CurrentUser.me();
        DelAccountForm.data.id = CurrentUser.user.id;
    },
    send: () => {
        DelAccountForm.data.id = CurrentUser.user.id;
        m.request({method: "DELETE", url: "/delete_account", withCredentials: true, body: DelAccountForm.data})
            .then(() => {
                alert("Account deleted.");
                CurrentUser.logout();
            })
            .catch((err) => (alert(err)))
    },
    view: () => (
        m("div", [
            m("h1", "Delete your account"),
            m("form", {
                onsubmit: (e) => {
                    e.preventDefault();
                    if (confirm("Do you really want to delete your account?")) {
                        DelAccountForm.send();
                    }
                }
            }, [
                m("button[type=submit]", "Delete")
            ])
        ])
    )
};
module.exports = DelAccountForm;

