import React from 'react';
import {useDispatch, useSelector} from "react-redux";
import {delDir} from "../models/dir";

const accept = (action, dispatch) => {
    if (action.type === "directory") {
        delDir(action.id, dispatch).then((result) => {
                dispatch({type: "SET_RELOAD_DIR", payload: true});
                reject(dispatch);
                return result;
            }
        )
    } else if (action.type === "file") {
        let request = new Request(`/file/${action.id}`, {
            method: "DELETE"
        });
        fetch(request).then((response) => {
            if (response.status === 200) {
                dispatch({type: "SET_RELOAD_DIR", payload: true});
                reject(dispatch);
            } else {
                response.json().then(json => dispatch({type: "SET_ERROR", payload: json}));
                reject(dispatch);
            }
        });
    } else {
        return false;
    }
};

const reject = (dispatch) => {
    dispatch({type: "SET_DEL_ITEM", payload: null});
};

const DelModal = () => {
    let del = useSelector(state => state.del);
    let dispatch = useDispatch();
    if (del == null) {
        return null;
    } else {
        return <div className={"overlay"}>
            <div className={"modal_overlay"}/>
            <div className={"modal_window"}>
                <h1>Delete item</h1>
                <p>{`Are you sure you want to delete the ${del.type} at ID ${del.id}?`}</p>
                <button onClick={() => {
                    accept({type: del.type, id: del.id}, dispatch);
                }}>Yes
                </button>
                <button onClick={() => {
                    reject(dispatch);
                }}>No
                </button>
            </div>
        </div>
    }
};

export default DelModal;