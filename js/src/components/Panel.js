import React, {useState} from 'react';
import {useDispatch, useSelector} from "react-redux";
import {Redirect} from "react-router";

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
                <input type={"text"} value={data.name}/>
                <label>Instance description</label>
                <textarea cols={80} rows={3} value={data.description}/>
                <label>Size limit for files (in bytes)</label>
                <input type={"number"} min={0} step={1} value={data.size_limit}/>
                <button>Save</button>
            </form>
        </div>
    }
};

export default Panel;