import React from 'react';
import {Link} from "react-router-dom";
import {connect} from "react-redux";

class TopBar extends React.Component {
    render() {
        return <nav>
            <Link to="/"><strong>{this.props.instance.data.name}</strong></Link>
        </nav>;
    }
}

const mapStateToProps = (state) => (
    {instance: state.instance}
);
const mapDispatchToProps = (dispatch) => ({});

export default connect(mapStateToProps, mapDispatchToProps)(TopBar);