import React from 'react';
import {useState} from 'react';
import {Link} from "react-router-dom";
import {register} from "../models/user";

const Register = () => {
    let [registered, setRegistered] = useState(false);
    let [user, setUser] = useState({
        name: "",
        email: "",
        password: "",
        pass_con: ""
    });
    if (registered) {
        return <div>
            <h1>Registration successful</h1>
            <p style={{textAlign: "center"}}>You can now <Link to={"/login"}><strong> sign in.</strong></Link></p>
        </div>;
    } else {
        return <div>
            <h1>Register</h1>
            <form>
                <label>Username</label>
                <input type={"text"} onChange={(e) => {
                    setUser({...user, name: e.target.value});
                }} value={user.name}/>
                <label>Email address</label>
                <input type={"email"} onChange={(e) => {
                    setUser({...user, email: e.target.value});
                }} value={user.email}/>
                <label>Password</label>
                <input type={"password"} onChange={(e) => {
                    setUser({...user, password: e.target.value});
                }} value={user.password}/>
                <label>Confirm password</label>
                <input type={"password"} onChange={(e) => {
                    setUser({...user, pass_con: e.target.value});
                }} value={user.pass_con}/>
                <button className={"centeredButton"} onClick={() => {
                    register(user).then((result) => setRegistered(result));
                }}>Register!
                </button>
            </form>
        </div>;
    }
};

export default Register;