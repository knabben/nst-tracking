import React from 'react';
import { Container } from '@material-ui/core';
import { Formik } from 'formik';
import { useStyles } from '../styles';
import { useDispatch, useSelector } from 'react-redux';
import Input from '@material-ui/core/Input';
import Button from '@material-ui/core/Button';
import Typography from '@material-ui/core/Typography';
import CssBaseline from '@material-ui/core/CssBaseline';
import { registerProduct } from '../../actions/product';
import { getToken } from '../../selectors/user';

const initialValues = {
  title: "",
  record_id: "",
  latitude: 0,
  longitude: 0,
  price: 0,
};

const ProductForm = () => {
  const classes = useStyles();
  const dispatch = useDispatch();
  const token = useSelector(getToken);

  return (
    <Formik
      initialValues={initialValues}
      onSubmit={(values, actions) => {
        dispatch(registerProduct(values, token))
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
        <Container component="main" maxWidth="xs">
        <CssBaseline />
        <div className={classes.paper}>
          <Typography component="h1" variant="h5"> Register Product </Typography>
          <form onSubmit={handleSubmit} noValidate autoComplete="off">
            <Input
              fullWidth
              placeholder="Title"
              className={classes.input}
              onChange={handleChange}
              onBlur={handleBlur}
              value={values.title}
              id="title"
              label="title"
            />
            {errors.title && touched.title && errors.title}
            <Input
              fullWidth
              placeholder="Identifier"
              className={classes.input}
              onChange={handleChange}
              onBlur={handleBlur}
              value={values.record_id}
              id="record_id"
              label="identifier"
            />
            {errors.record_id && touched.record_id && errors.record_id}
            <Input
              fullWidth
              type="number"
              placeholder="Initial price ($)"
              className={classes.input}
              onChange={handleChange}
              onBlur={handleBlur}
              value={values.price}
              id="price"
              label="price"
            />
            {errors.price && touched.price && errors.price}
            <Input
              fullWidth
              type="number"
              placeholder="Latitude"
              className={classes.input}
              onChange={handleChange}
              onBlur={handleBlur}
              value={values.latitude}
              id="latitude"
              label="latitude"
            />
            {errors.latitude && touched.latitude && errors.latitude}
            <Input
              fullWidth
              type="number"
              placeholder="Longitude"
              className={classes.input}
              onChange={handleChange}
              onBlur={handleBlur}
              value={values.longitude}
              id="longitude"
              label="longitude"
            />
            {errors.longitude && touched.longitude && errors.longitude}
            <Button
              fullWidth
              variant="contained" 
              className={classes.button} 
              disabled={isSubmitting}
              type="submit"
            >
              Submit
            </Button>
          </form>
        </div>
      </Container>
      )}
  </Formik>
  )
}

export default ProductForm;