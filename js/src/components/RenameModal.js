import React, {useState} from 'react';
import {useDispatch, useSelector} from "react-redux";
import {renameDir} from "../models/dir";
import {renameFile} from "../models/file";

const accept = (action, dispatch) => {
    if (action.type === "directory") {
        renameDir(action, dispatch).then((result) => {
                dispatch({type: "SET_RELOAD_DIR", payload: result});
            }
        )
    } else if (action.type === "file") {
        renameFile(action, dispatch).then((result) => {
            dispatch({type: "SET_RELOAD_DIR", payload: result});
        })
    } else {
        return false;
    }
    reject(dispatch);
};

const reject = (dispatch) => {
    dispatch({type: "SET_RN_ITEM", payload: null});
};

const RenameModal = () => {
    let ren = useSelector(state => state.rename);
    let [name, setName] = useState("");
    let dispatch = useDispatch();
    if (ren == null) {
        return null;
    } else {
        return <div className={"overlay"}>
            <div className={"modal_overlay"}/>
            <div className={"modal_window"}>
                <h1>{`Rename ${ren.type}`}</h1>
                <input type={"text"} required={true} value={name} onChange={(e) => {
                    setName(e.target.value);
                }}/>
                <button onClick={() => {
                    accept({type: ren.type, name: name, id: ren.id}, dispatch);
                }}>Rename
                </button>
                <button onClick={() => {
                    reject(dispatch);
                }}>Cancel
                </button>
            </div>
        </div>
    }
};

export default RenameModal;