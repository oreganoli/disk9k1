import React, {useState} from 'react';
import {Redirect} from "react-router";
import {useDispatch, useSelector} from "react-redux";
import {deleteUser, loadMe, putPassword, putUsername} from "../models/user";

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
                                console.log("Username change succeeded!");
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
                <button onClick={() => {
                    putPassword({password: password, passCon: passCon})
                        .then((result) => {
                            if (result) {
                                console.log("Password change succeeded!");
                                loadMe().then((result) => {
                                    dispatch({type: "SET_USER", payload: result});
                                })
                            }
                        })
                }}>Change your password
                </button>
            </form>
            <button onClick={() => {
                if (confirm("Are you SURE you want to delete your account? This will delete all your content forever.")) {
                    deleteUser(user.id).then((result) => console.log(result ? "User deletion successful." : "User deletion failed."));
                }
            }} className={"centeredButton"}><strong>Delete your account</strong></button>
        </div>;
    } else {
        return <Redirect to={"/"}/>;
    }
};