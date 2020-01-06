import React, {useState} from 'react';
import {useDispatch, useSelector} from "react-redux";
import {createDir} from "../models/dir";

const accept = ({name, owner, parent}, dispatch) => {
    createDir({name, owner, parent}, dispatch).then((result) => {
            dispatch({type: "SET_RELOAD_DIR", payload: true});
            reject(dispatch);
            return result;
        }
    )
};

const reject = (dispatch) => {
    dispatch({type: "SET_CREATE_DIR", payload: null});
};

const CrDirModal = () => {
    let dispatch = useDispatch();
    let [dirName, setDirName] = useState("");
    let crDir = useSelector(state => state.crDir);
    if (crDir == null) {
        return null;
    } else {
        return <div className={"overlay"}>
            <div className={"modal_overlay"}/>
            <div className={"modal_window"}>
                <h1>Create directory</h1>
                <form>
                    <label>Name</label>
                    <input value={dirName} onChange={(e) => {
                        setDirName(e.target.value);
                    }} type={"text"}/>
                </form>
                <button onClick={() => {
                    accept({name: dirName, owner: crDir.owner, parent: crDir.parent}, dispatch);
                }}>Create
                </button>
                <button onClick={() => {
                    reject(dispatch);
                }}>Cancel
                </button>
            </div>
        </div>
    }
};

export default CrDirModal;