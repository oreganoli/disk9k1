export const renameFile = async ({id, name}, dispatch) => {
    let request = new Request(`/rename_file/${id}`, {
        method: "PUT",
        body: JSON.stringify({name: name})
    });
    let response = await fetch(request);
    if (response.status === 200) {
        console.log("File successfully renamed");
        return true;
    } else {
        let err = await response.json();
        console.log(err);
        dispatch({type: "SET_ERROR", payload: err});
        return false;
    }
};