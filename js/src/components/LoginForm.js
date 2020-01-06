import React from 'react';
import {useState} from 'react';

export const LoginForm = () => {
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");
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
            <button className={"centeredButton"}>Sign in</button>
        </form>
    </div>;
};