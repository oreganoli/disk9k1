import React, {useState} from 'react';
import {Redirect} from "react-router";
import {useDispatch, useSelector} from "react-redux";
import {loadMe, putUsername} from "../models/user";

export const Me = () => {
    const user = useSelector(state => state.user);
    const dispatch = useDispatch();
    if (user == null) {
        console.log("No user found, redirecting...");
    }
    const [username, setUsername] = useState(user == null ? "placeholder" : user.name);
    const [password, setPassword] = useState("");
    const [passCon, setPassCon] = useState("");
    if (user != null) {
        return <div>
            <h1>Your account</h1>
            <form>
                <label>Username</label>
                <input type={"text"} value={username} onChange={(e) => {
                    setUsername(e.target.value);
                }}/>
                <button onClick={() => {
                    putUsername({username: username})
                        .then((result) => {
                            if (result) {
                                loadMe().then((result) => {
                                    dispatch({type: "SET_USER", payload: result});
                                });
                            }
                        });
                }}>Change your username
                </button>
                <label>Password</label>
                <input type={"password"} value={password} onChange={(e) => {
                    setPassword(e.target.value);
                }}/>
                <label>Confirm password</label>
                <input type={"password"} value={passCon} onChange={(e) => {
                    setPassCon(e.target.value);
                }}/>
                <button>Change your password</button>
            </form>
            <button className={"centeredButton"}><strong>Delete your account</strong></button>
        </div>;
    } else {
        return <Redirect to={"/"}/>;
    }
};