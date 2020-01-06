import React, {useState} from 'react';
import {useDispatch} from 'react-redux'
import {logOut} from "../models/user";
import {Redirect} from "react-router";

export const LogoutButton = () => {
    const dispatch = useDispatch();
    const [toHome, setToHome] = useState(false);
    if (toHome) {
        return <Redirect to={"/"}/>;
    } else {
        return <button onClick={() => {
            logOut().then(dispatch({type: "LOGOUT"}))
        }}>Log out</button>
    }
};