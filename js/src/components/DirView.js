import React, {useEffect} from 'react';
import {useDispatch, useSelector} from "react-redux";
import {useParams} from "react-router";
import {loadDir} from "../models/dir";
import {Link} from "react-router-dom";

const contents = (props) => {
    let upLink;
    if (props.parent == null && props.id !== 0) {
        upLink = <tr>
            <td><Link to={`/drive`}><strong>⬆ ../</strong></Link></td>
        </tr>;
    } else if (props.id === 0) {
        upLink = null;
    } else {
        upLink = <tr>
            <td><Link to={`/drive/${props.parent}`}><i>📁 ../</i></Link></td>
        </tr>;
    }
    let kids = props.children.map((each) => (
        <tr>
            <td><Link to={`/drive/${each.id}`}>{`📁 ${each.name}/`}</Link></td>
        </tr>
    ));
    return <table>
        {upLink}
        {kids}
    </table>;
};

export const DirView = () => {
    let {id} = useParams();
    let dispatch = useDispatch();
    useEffect(() => {
        if (typeof id === "undefined") {
            id = null;
        }
        loadDir(id, dispatch).then((result) => (console.log(result ? "Loading dir succeeded." : "Loading dir failed.")))
    }, [id]);
    let dir = useSelector(state => state.dir);
    if (dir == null) {
        return null;
    } else {
        return <div>
            <h1>{dir.name}</h1>
            {contents({id: dir.id, name: dir.name, children: dir.children})}
        </div>;
    }
};