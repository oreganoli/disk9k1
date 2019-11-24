const m = require("mithril");
const Instance = require("../models/instance");

const ModifyInstanceForm = {
    data: {
        name: "",
        description: "",
        size_limit: -1
    },
    send: function () {
        m.request({
            method: "PUT",
            url: "/modify_instance",
            withCredentials: true,
            body: ModifyInstanceForm.data
        }).then(function () {
            Instance.load();
        })
    },
    view: function () {
        return m("form", {
            onsubmit: function (e) {
                e.preventDefault();
                ModifyInstanceForm.send();
            }
        }, [
            m("label", "Instance name"),
            m("input[type=text]", {
                oninput: function (e) {
                    ModifyInstanceForm.data.name = e.target.value;
                },
                value: Instance.name
            }),
            m("label", "Description"),
            m("textarea", {
                oninput: function (e) {
                    ModifyInstanceForm.data.description = e.target.value;
                },
                value: Instance.description
            }),
            m("label", "Size limit (B)"),
            m("input[type=number]", {
                oninput: function (e) {
                    ModifyInstanceForm.data.description = e.target.value;
                },
            })
        ]);
    }
};
module.exports = ModifyInstanceForm;