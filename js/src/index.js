import {HashRouter as Router} from "react-router-dom";
import {Provider} from 'react-redux';
import {combineReducers, createStore} from 'redux';
import React from 'react';
import ReactDOM from 'react-dom';
import instance from "./reducers/instanceReducer";
import user from "./reducers/userReducer";
import Home from "./components/Home";
import TopBar from "./components/TopBar";
import {Route, Switch} from "react-router";
import Footer from "./components/Footer";

const regeneratorRuntime = require('regenerator-runtime/runtime');
const title = 'React hello world';
const reducer = combineReducers({instance, user});
const store = createStore(reducer,
    window.__REDUX_DEVTOOLS_EXTENSION__ && window.__REDUX_DEVTOOLS_EXTENSION__());
ReactDOM.render(
    <Provider store={store}>
        <Router>
            <TopBar/>
            <Switch>
                <Route path="/">
                    <Home/>
                </Route>
            </Switch>
            <Footer/>
        </Router>
    </Provider>,
    document.getElementById("root")
);