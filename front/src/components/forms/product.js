import React from 'react';
import { Container } from '@material-ui/core';

import { Formik } from 'formik';
import { useStyles } from '../styles';
import { useDispatch } from 'react-redux';
import { loginUser } from '../../actions/user'
import Input from '@material-ui/core/Input';
import Button from '@material-ui/core/Button';
import Typography from '@material-ui/core/Typography';
import CssBaseline from '@material-ui/core/CssBaseline';
import { registerProduct } from '../../actions/product';

const initialValues = {
  title: "",
  latitude: "",
  longitude: "",
};

const ProductForm = () => {
  const classes = useStyles();
  const dispatch = useDispatch()

  return (
    <Formik
      initialValues={initialValues}
      onSubmit={(values, actions) => {
        dispatch(registerProduct(values))
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