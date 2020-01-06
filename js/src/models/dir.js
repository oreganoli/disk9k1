const regeneratorRuntime = require('regenerator-runtime/runtime');

export const createDir = async ({name, owner, parent}, dispatch) => {
    let request = new Request("/drive", {
        method: "POST",
        body: JSON.stringify({name: name, owner: owner, parent: parent})
    });
    let response = await fetch(request).catch((err) => alert(err));
    if (response.status === 200) {
        return true;
    } else {
        let err = await response.json();
        dispatch({type: "SET_ERROR", payload: err});
        return false;
    }
};

export const loadDir = async (id, dispatch) => {
    let request = new Request(`/drive${id == null ? "" : `/${id}`}`, {
        method: "GET"
    });
    let response = await fetch(request).catch((err) => alert(err));
    if (response.status === 200) {
        let dir = await response.json();
        console.log(dir);
        dispatch({type: "SET_DIR", payload: dir});
        return true;
    } else {
        let err = await response.json();
        console.log(err);
        dispatch({type: "SET_ERROR", payload: err});
        return false;
    }
};

export const delDir = async (id, dispatch) => {
    let request = new Request("drive", {
        method: "DELETE",
        body: JSON.stringify({id: id})
    });
    let response = await fetch(request).catch((err) => alert(err));
    if (response.status === 200) {
        console.log("Dir deletion successful");
        return true;
    } else {
        let err = await response.json();
        console.log(err);
        dispatch({type: "SET_ERROR", payload: err});
        return false;
    }
};

export const renameDir = async ({id, name}, dispatch) => {
    let request = new Request("/rename_dir", {
        method: "PUT",
        body: JSON.stringify({id: id, name: name})
    });
    let response = await fetch(request).catch((err) => alert(err));
    if (response.status === 200) {
        console.log("Dir deletion successful");
        return true;
    } else {
        let err = await response.json();
        console.log(err);
        dispatch({type: "SET_ERROR", payload: err});
        return false;
    }
}