import React, { useEffect } from 'react';
import './App.css';

import { Container } from '@material-ui/core';
import { makeStyles } from '@material-ui/core/styles';
import Card from '@material-ui/core/Card';
import Typography from '@material-ui/core/Typography';
import { useSelector, useDispatch } from 'react-redux';
import { isAuthenticated } from '../selectors/user';
import { fetchBid } from '../actions/bid';
import { getBids } from '../selectors/bid';
import { getToken } from '../selectors/user';

const useStyles = makeStyles({
  card: {
    minWidth: 275,
    marginTop: 20,
    padding: 10,
  },
  bullet: {
    display: 'inline-block',
    margin: '0 2px',
    transform: 'scale(0.8)',
  },
  title: {
    fontSize: 14,
  },
  pos: {
    marginBottom: 12,
  },
});

const Bids = () => {
  const classes = useStyles();
  const bids = useSelector(getBids);
  const authenticated = useSelector(isAuthenticated);
  const token = useSelector(getToken);
  const dispatch = useDispatch();

  useEffect(() => {
    if (authenticated) {
      dispatch(fetchBid(token));
    }
  }, [authenticated]);

  return (
    <div className="App">
    <Container component="main" maxWidth="xs">
      <Typography variant="h5" component="h2"> Bids </Typography>
      <div>
      {
        bids && bids.map( (item) => (
          <Card className={classes.card}>
            <Typography variant="h5" component="h2">
              id {item.id} - product-id {item.product_id} 
            </Typography>
            <Typography className={classes.title} color="textSecondary" gutterBottom>
              bid price - {item.price}
            </Typography>
          </Card>
        ))
      }
      </div>
    </Container>
    </div>
  )
}

export default Bids;