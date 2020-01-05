import {connect, Provider} from 'react-redux';
import {createStore} from 'redux';
import React from 'react';
import ReactDOM from 'react-dom';

const title = 'React hello world';
const reducer = (state, action) => (state);
const store = createStore(reducer, {text: "Hello world from React!"});

class Element extends React.Component {
    render() {
        return <h1>{this.props.text}</h1>;
    }
}

const mapStateToProps = (state) => ({text: state.text});
const mapDispatchToProps = (dispatch) => ({});
const ConnectedElement = connect(mapStateToProps, mapDispatchToProps)(Element);


ReactDOM.render(
    <Provider store={store}>
        <ConnectedElement/>
    </Provider>,
    document.getElementById("root")
);