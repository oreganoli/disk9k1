import React from 'react';
import {useDispatch, useSelector} from "react-redux";

const reject = (dispatch) => {
    dispatch({type: "SET_CREATE_DIR", payload: null});
};

const CrDirModal = () => {
    let dispatch = useDispatch();
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
                    <input type={"text"}/>
                </form>
                <button onClick={() => {
                    reject(dispatch);
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