import dir from "./dirReducer";
import error from "./errorReducer";
import instance from "./instanceReducer";
import user from "./userReducer";
import {combineReducers} from "redux";

const root = combineReducers({dir, error, instance, user});
export default root;