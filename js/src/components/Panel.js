import React, {useState} from 'react';
import {useDispatch, useSelector} from "react-redux";
import {Redirect} from "react-router";
import {putInstance} from "../models/instance";
import {loadInstance} from "../reducers/instanceReducer";

const Panel = () => {
    let dispatch = useDispatch();
    let instance = useSelector(state => state.instance);
    let user = useSelector(state => state.user);
    let [data, setData] = useState(instance);

    if (user == null || !user.is_admin) {
        return <Redirect to={"/login"}/>;
    } else {
        return <div>
            <h1>Global Disk9k1 settings</h1>
            <form>
                <label>Instance name</label>
                <input type={"text"} value={data.name} onChange={(e) => {
                    setData({...data, name: e.target.value})
                }}/>
                <label>Instance description</label>
                <textarea onChange={(e) => {
                    setData({...data, description: e.target.value})
                }} cols={80} rows={3} value={data.description}/>
                <label>Size limit for files (in bytes)</label>
                <input onChange={(e) => {
                    setData({...data, size_limit: e.target.value})
                }} type={"number"} min={0} step={1} value={data.size_limit}/>
                <button onClick={() => {
                    putInstance(data, dispatch).then(
                        (result) => {
                            if (result) {
                                loadInstance().then((inst) => {dispatch({type: "LOAD_INSTANCE", payload: inst})});
                            }
                        });
                }}>Save</button>
            </form>
        </div>
    }
};

export default Panel;