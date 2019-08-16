import React from "react";
import App from "./App";
import Signin from './components/forms/Signin'
import Signup from './components/forms/Signup'

import { BrowserRouter as Router, Route, Link} from "react-router-dom";

function AppRouter() {
  return (
    <Router>
      <div>
        <Link to="/">Home</Link> | 
        <Link to="/signin">Signin</Link> | 
        <Link to="/signup">Signup</Link>
      </div>

      <Route path="/" exact component={App} />
      <Route path="/signup" exact component={Signup} />
      <Route path="/signin" exact component={Signin} />
    </Router>
  )
}

export default AppRouter;
