import React from 'react'
import { Container } from '@material-ui/core';
import { makeStyles } from '@material-ui/core/styles';
import Card from '@material-ui/core/Card';
import BidForm from './forms/bid';
import Typography from '@material-ui/core/Typography';

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

const Product = ({item}) => {
  const classes = useStyles();

  return (
    <Container component="main" maxWidth="xs">
      <Card className={classes.card}>
        <Typography variant="h5" component="h2">
          {item.title}
        </Typography>
        <Typography className={classes.title} color="textSecondary" gutterBottom>
          {item.auth_id} - {item.price}
        </Typography>
        <BidForm id={item.id} />
      </Card>
    </Container>
  )
}

export default Product;