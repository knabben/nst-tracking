import React from "react";
import HomeContainer from "./containers/Home";

import DevTools from './containers/DevTools'
import Signup from './containers/Signup'
import SigninForm from './components/forms/signin'
import { Provider } from 'react-redux'
import { ConnectedRouter } from 'connected-react-router'

import Menu from './containers/Menu';
import {Route} from "react-router-dom";
import { history } from './store'

const Root = ({ store }) => (
  <Provider store={store}>
    <ConnectedRouter history={history}>
      <Menu />
      <Route path="/" exact component={HomeContainer} />
      <Route path="/signin" exact component={SigninForm} />
      {/* <Route path="/signin" exact component={Signin} /> */}
      <DevTools />
    </ConnectedRouter>
  </Provider>
)

export default Root;
