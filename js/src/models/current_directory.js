const m = require("mithril");

const CurrentDirectory = {
    info: null,
    directories: [],
    load: (id) => {
        m.request({
            method: "GET",
            withCredentials: true,
            url: id == null ? "/drive" : `/drive/${id}`
        }).then((result) => {
            CurrentDirectory.info = result.info;
            CurrentDirectory.directories = result.directories;
        }).catch((err) => (alert(err)))
    }
};

module.exports = CurrentDirectory;