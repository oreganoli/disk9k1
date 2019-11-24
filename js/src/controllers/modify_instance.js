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
        })
    },
    oncreate: () => {
        Instance.load_then(() => {
            document.getElementById("inst_text_input").setAttribute("value", Instance.name);
            document.getElementById("inst_desc_input").innerText = Instance.description;
            document.getElementById("inst_size_input").setAttribute("value", Instance.size_limit);
        })
    },
    view: () => {
        return m("form", {
            onsubmit: (e) => {
                e.preventDefault();
                ModifyInstanceForm.send();
            }
        }, [
            m("label", "Instance name"),
            m("input#inst_text_input[type=text]", {
                oninput: (e) => {
                    ModifyInstanceForm.data.name = e.target.value;
                }
            }),
            m("label", "Description"),
            m("textarea#inst_desc_input", {
                oninput: (e) => {
                    ModifyInstanceForm.data.description = e.target.value;
                }
            }),
            m("label", "Size limit (B)"),
            m("input#inst_size_input[type=number]", {
                oninput: (e) => {
                    ModifyInstanceForm.data.description = e.target.value;
                },
            })
        ]);
    }
};
module.exports = ModifyInstanceForm;