const regeneratorRuntime = require('regenerator-runtime/runtime');
import {Provider} from 'react-redux';
import {combineReducers, createStore} from 'redux';
import React from 'react';
import ReactDOM from 'react-dom';
import instance from "./reducers/instanceReducer";
import VisibleHome from "./components/Home.jsx";

const title = 'React hello world';
const reducer = combineReducers({instance});
const store = createStore(reducer,
    window.__REDUX_DEVTOOLS_EXTENSION__ && window.__REDUX_DEVTOOLS_EXTENSION__());
ReactDOM.render(
    <Provider store={store}>
        <VisibleHome/>
    </Provider>,
    document.getElementById("root")
);