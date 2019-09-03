import React, { useEffect } from 'react';
import './App.css';
import { useSelector, useDispatch } from 'react-redux'
import { fetchProduct } from '../actions/product'
import { isAuthenticated, getToken } from '../selectors/user'
import { getProducts } from '../selectors/product'
import Product from './Product';
import { Redirect } from 'react-router';
import Typography from '@material-ui/core/Typography';

const Home = () => {
  const authenticated = useSelector(isAuthenticated);
  const products = useSelector(getProducts)
  const token = useSelector(getToken);
  const dispatch = useDispatch();

  useEffect(() => {
    if (authenticated) {
      dispatch(fetchProduct(token));
    }
  }, [authenticated]);

  return (
    !authenticated ? (
      <Redirect to="/signin" />
    ) : (
      <div className="App">
        <Typography variant="h5" component="h2"> Products </Typography>
        <div>
          { 
            products.map((item) => (
              <Product key={item.id} item={item} />
            )) 
          }
        </div>
      </div>
    )
  );
}

export default Home;