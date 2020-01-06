export const putInstance = async ({name, description, size_limit}, dispatch) => {
    let request = new Request("/instance", {
        method: "PUT",
        body: JSON.stringify({name: name, description: description, size_limit: size_limit})
    });
    let response = await fetch(request);
    if (response.status === 200) {
        return true;
    } else {
        let err = await response.json();
        dispatch({type: "SET_ERR", payload: err});
        return false;
    }
};