var m = require("mithril");
var InstanceView = require("./views/instance");

m.route(document.body, "/index", {
    "/index": InstanceView
});