import React, {useState} from 'react';
import {Redirect} from "react-router";
import {useDispatch, useSelector} from "react-redux";
import {loadMe, signIn} from "../models/user";

export const LoginForm = () => {
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");
    const userSelector = useSelector(state => state.user);
    const dispatch = useDispatch();
    const submit = () => {
        let data = {
            username: username,
            password: password
        };
        signIn(data).then((result) => {
            if (result) {
                loadMe().then((me) => {
                    dispatch({type: "SET_USER", payload: me})
                }).catch((err) => alert(err));
            }
        })
    };
    if (userSelector != null) {
        return <Redirect to={"/"}/>;
    } else {
        return <div>
            <h1>Sign in</h1>
            <form>
                <label>Username</label>
                <input type={"text"} value={username} required={true} onChange={(e) => {
                    setUsername(e.target.value);
                }}/>
                <label>Password</label>
                <input type={"password"} value={password} required={true} onChange={(e) => {
                    setPassword(e.target.value);
                }}/>
                <button className={"centeredButton"} onClick={submit}>Sign in</button>
            </form>
        </div>;
    }
};