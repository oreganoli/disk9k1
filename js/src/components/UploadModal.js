import React, {useState} from 'react';
import {useDispatch, useSelector} from "react-redux";

const uploadModal = () => {
    let dispatch = useDispatch();
    let reject = () => {
        dispatch({type: "SET_UPLOAD", payload: false});
    };
    let [filename, setFilename] = useState("");
    let upload = useSelector(state => state.upload);
    let dir = useSelector(state => state.dir);
    let dir_id = (dir == null ? null : dir.id);
    if (!upload || dir == null) {
        return null;
    } else {
        return <div className={"overlay"}>
            <div className={"modal_overlay"}/>
            <div className={"modal_window"}>
                <h1>Upload file</h1>
                <form id={"upload_form"} method={"post"} action={"/upload"} onSubmit={(e) => {
                    e.preventDefault();
                    let form = document.getElementById("upload_form");
                    let data = new FormData(form);
                    let request = new Request("/upload", {
                        method: "POST",
                        body: data
                    });
                    fetch(request).then(response => console.log(response.status));
                }}>
                    <label>Filename</label>
                    <input type={"text"} name={"filename"} value={filename} onChange={(e) => {
                        setFilename(e.target.value);
                    }}/>
                    <label>Public</label>
                    <input type={"checkbox"} name={"public"}/>
                    <input type={"hidden"} name={"directory"} value={dir_id}/>
                    <label>File:</label>
                    <input type={"file"} name={"data"} onChange={(e) => {
                        setFilename(e.target.value.split(/([\\/])/g).pop());
                    }}/>
                    <button type={"submit"}>Upload</button>
                </form>
                <button onClick={() => {
                    reject();
                }}>Cancel
                </button>
            </div>
        </div>;
    }
}
export default uploadModal;