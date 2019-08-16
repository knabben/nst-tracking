import React from "react";
import App from "./App";
import Signin from './components/forms/Signin'
import DevTools from './containers/DevTools'
import Signup from './containers/Signup'
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
