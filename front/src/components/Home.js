import React from 'react';
import './App.css';
import { useSelector } from 'react-redux'
import { isAuthenticated } from '../selectors/user'
import Button from '@material-ui/core/Button';
import { Redirect } from 'react-router'

const Home = () => {
  const authenticated = useSelector(isAuthenticated);

  return (
    !authenticated ? (
      <Redirect to="/signin" />
    ) : (
      <div className="App">
        <header className="App-header">
          <Button variant="contained" color="primary">Button</Button>
        </header>
      </div>
    )
  );
}

export default Home;