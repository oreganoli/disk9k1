const regeneratorRuntime = require('regenerator-runtime/runtime');

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
}