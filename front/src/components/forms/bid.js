import React from 'react';
import { Formik } from 'formik';
import { registerBid } from '../../actions/bid';
import { getToken } from '../../selectors/user'
import { useSelector, useDispatch } from 'react-redux'
import Input from '@material-ui/core/Input';
import Button from '@material-ui/core/Button';
import { useStyles } from '../styles';

const BidForm = ({id}) => {
  const classes = useStyles();
  const token = useSelector(getToken);
  const dispatch = useDispatch();

  return (
    <Formik
      initialValues={{product_id: id, price: 0}}
      onSubmit={(values, actions) => {
        dispatch(registerBid(values, token))
        actions.setSubmitting(false)
      }}
    >
      {({
        handleSubmit,
        handleChange,
        handleBlur,
        values,
        errors,
        touched,
        isSubmitting,
      }) => (
        <form onSubmit={handleSubmit} noValidate autoComplete="off">
          <Input
            placeholder="Price"
            type="number"
            className={classes.input}
            onChange={handleChange}
            onBlur={handleBlur}
            value={values.price}
            id="price"
            label="Price"
          />
          <input type="hidden" id="product_id" name="product" value={values.id} />
          <Button type="submit" variant="contained" color="primary">BID</Button>
        </form>
      )}
    </Formik>
  )   
}
export default BidForm;