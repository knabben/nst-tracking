import React, { useEffect } from 'react';
import './App.css';
import { useSelector, useDispatch } from 'react-redux'
import { fetchProduct } from '../actions/product'
import { isAuthenticated, getToken } from '../selectors/user'
import Button from '@material-ui/core/Button';
import { Redirect } from 'react-router'

const Home = () => {
  const authenticated = useSelector(isAuthenticated);
  const token = useSelector(getToken);
  const dispatch = useDispatch();

  useEffect(() => {
    if (authenticated) {
      console.log(token)
      dispatch(fetchProduct(token));
    }
  }, [authenticated]);

  return (
    !authenticated ? (
      <Redirect to="/signin" />
    ) : (
      <div className="App">
        <header className="App-header">
          ksajdflaskdjf
          <Button variant="contained" color="primary">Button</Button>
        </header>
      </div>
    )
  );
}

export default Home;