const regeneratorRuntime = require('regenerator-runtime/runtime');
import {HashRouter as Router} from "react-router-dom";
import {Provider} from 'react-redux';
import {combineReducers, createStore} from 'redux';
import React from 'react';
import ReactDOM from 'react-dom';
import dir from "./reducers/dirReducer";
import error from "./reducers/errorReducer";
import instance from "./reducers/instanceReducer";
import user from "./reducers/userReducer";
import Home from "./components/Home";
import TopBar from "./components/TopBar";
import {LoginForm} from "./components/LoginForm";
import {DirView} from "./components/DirView";
import {Route, Switch} from "react-router";
import Footer from "./components/Footer";
import ErrorWindow from "./components/ErrorWindow";
import {Me} from "./components/Me";

const title = 'React hello world';
const reducer = combineReducers({dir, error, instance, user});
export const store = createStore(reducer,
    window.__REDUX_DEVTOOLS_EXTENSION__ && window.__REDUX_DEVTOOLS_EXTENSION__());
ReactDOM.render(
    <Provider store={store}>
        <Router>
            <TopBar/>
            <Switch>
                <Route path="/login">
                    <LoginForm/>
                </Route>
                <Route path="/me">
                    <Me/>
                </Route>
                <Route path="/drive/:id" component={DirView}/>
                <Route path="/drive" component={DirView}/>
                <Route path="/">
                    <Home/>
                </Route>
            </Switch>
            <Footer/>
            <ErrorWindow/>
        </Router>
    </Provider>,
    document.getElementById("root")
);