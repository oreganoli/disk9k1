const m = require("mithril");
const Instance = require("../models/instance");

const ModifyInstanceForm = {
    data: {
        name: "",
        description: "",
        size_limit: -1
    },
    send: () => {
        m.request({
            method: "PUT",
            url: "/modify_instance",
            withCredentials: true,
            body: ModifyInstanceForm.data
        }).then(() => {
            Instance.load();
        }).catch((err) => alert(err))
    },
    oncreate: () => {
        Instance.load_then(() => {
            ModifyInstanceForm.data.name = Instance.name;
            ModifyInstanceForm.data.description = Instance.description;
            ModifyInstanceForm.data.size_limit = Instance.size_limit;
        })
    },
    view: () => {
        return [m("h1", "Modify instance settings"), m("form", {
            onsubmit: (e) => {
                e.preventDefault();
                ModifyInstanceForm.send();
            }
        }, [
            m("label", "Instance name"),
            m("input#inst_text_input[type=text]", {
                oninput: (e) => {
                    ModifyInstanceForm.data.name = e.target.value;
                },
                value: ModifyInstanceForm.data.name
            }),
            m("label", "Description"),
            m("textarea#inst_desc_input", {
                oninput: (e) => {
                    ModifyInstanceForm.data.description = e.target.value;
                },
                value: ModifyInstanceForm.data.description
            }),
            m("label", "Size limit (B)"),
            m("input#inst_size_input[type=number]", {
                oninput: (e) => {
                    ModifyInstanceForm.data.size_limit = parseInt(e.target.value);
                },
                value: ModifyInstanceForm.data.size_limit
            }),
            m("button", "Save settings")
        ])];
    }
};
module.exports = ModifyInstanceForm;