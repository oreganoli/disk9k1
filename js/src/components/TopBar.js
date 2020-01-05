import React from 'react';
import {Link} from "react-router-dom";
import {connect} from "react-redux";

class TopBar extends React.Component {
    render() {
        return <nav>
            <div className={"left_half"}>
                <Link to="/"><strong>{this.props.instance.name}</strong></Link>
            </div>
            <div className={"right_half"}>
            </div>
        </nav>;
    }
}

const mapStateToProps = (state) => (
    {instance: state.instance}
);
const mapDispatchToProps = (dispatch) => ({});

export default connect(mapStateToProps, mapDispatchToProps)(TopBar);