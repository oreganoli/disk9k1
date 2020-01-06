import React from 'react';
import {useParams} from "react-router";

export const DirView = (props) => {
    let {id} = useParams();
    if (id == null) {
        id = 0;
    }
    return <div>
        <h1>Directory with id {id}</h1>
    </div>;
};