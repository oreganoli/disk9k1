import RenameModal from "./components/RenameModal";
import {HashRouter as Router} from "react-router-dom";
import {Provider} from 'react-redux';
import {createStore} from 'redux';
import React from 'react';
import ReactDOM from 'react-dom';
import Home from "./components/Home";
import TopBar from "./components/TopBar";
import {LoginForm} from "./components/LoginForm";
import {DirView} from "./components/DirView";
import {Route, Switch} from "react-router";
import Footer from "./components/Footer";
import DelModal from "./components/DelModal";
import ErrorModal from "./components/ErrorModal";
import {Me} from "./components/Me";
import Register from "./components/Register";
import root from "./reducers/rootReducer";

const regeneratorRuntime = require('regenerator-runtime/runtime');

const title = 'React hello world';
export const store = createStore(root,
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
                <Route path="/register" component={Register}/>
                <Route path="/">
                    <Home/>
                </Route>
            </Switch>
            <Footer/>
            <ErrorModal/>
            <DelModal/>
            <RenameModal/>
        </Router>
    </Provider>,
    document.getElementById("root")
);