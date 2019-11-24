const m = require("mithril");
const ModifyInstanceForm = require("../controllers/modify_instance");

const Panel = {
    view: function () {
        return m("main.main_container", m(ModifyInstanceForm));
    }
};
module.exports = Panel;