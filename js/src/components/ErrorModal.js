import React from 'react';
import {useDispatch, useSelector} from "react-redux";

const acceptError = (dispatch) => {
    dispatch({type: "SET_ERROR", payload: null});
};
const ErrorModal = () => {
    let error = useSelector(state => state.error);
    let dispatch = useDispatch();
    if (error != null) {
        return <div className={"overlay"}>
            <div className={"modal_overlay"}/>
            <div className={"modal_window"}>
                <h1 className={"error_reason"}>Error</h1>
                <h2 className={"error_reason"}>{error.name}</h2>
                <button className={"central centeredButton"} onClick={() => {
                    acceptError(dispatch)
                }}>OK
                </button>
            </div>
        </div>;
    } else {
        return null;
    }
};
export default ErrorModal;