import React from "react";
import App from "./App";

import DevTools from './containers/DevTools'
import Signup from './containers/Signup'
import Signin from './containers/Signin'
import { Provider } from 'react-redux'

import {Route, Link} from "react-router-dom";

const Root = ({ store }) => (
  <Provider store={store}>
    <Route path="/" exact component={App} />
    <Route path="/signup" exact component={Signup} />
    <Route path="/signin" exact component={Signin} />
    <DevTools />
  </Provider>
)

export default Root;
