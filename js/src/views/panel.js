var m = require("mithril");
var TopBar = require("../views/top_bar");
var ModifyInstanceForm = require("../forms/modify_instance");

var Panel = {
    view: function () {
        return m("main.main_container", m(ModifyInstanceForm));
    }
};
module.exports = Panel;