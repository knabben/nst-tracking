import React from 'react';

import './App.css';
import Button from '@material-ui/core/Button';
import { Redirect } from 'react-router'

const Home = ({authenticated}) => {
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