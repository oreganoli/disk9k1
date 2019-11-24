const m = require("mithril");
const ModifyInstanceForm = require("../controllers/modify_instance");

const Panel = {
    view: () => {
        return m(ModifyInstanceForm);
    }
};
module.exports = Panel;