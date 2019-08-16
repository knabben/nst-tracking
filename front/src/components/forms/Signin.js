import React from 'react';
import { Formik } from 'formik';
import { Container } from '@material-ui/core';
import { useStyles } from './styles';
import CssBaseline from '@material-ui/core/CssBaseline';
import Input from '@material-ui/core/Input';
import Button from '@material-ui/core/Button';
import Typography from '@material-ui/core/Typography';

const initialValues = {
  "username": "", 
  "password": ""
}

export default function Signin() {
  const classes = useStyles();
  return (
    <Container component="main" maxWidth="xs">
      <CssBaseline />
      <div className={classes.paper}>
        <Typography component="h1" variant="h5">
          Sign in
        </Typography>
        <Formik
          initialValues={initialValues}
          onSubmit={(values, {setSubmitting}) => {
              alert(JSON.stringify(values, null, 2));
              setSubmitting(false);
          }}
        >
          {({
            values,
            errors,
            touched,
            handleChange,
            handleBlur,
            handleSubmit,
            isSubmitting,
          }) => (
            <form onSubmit={handleSubmit} noValidate autoComplete="off">
              <Input
                fullWidth
                placeholder="Username"
                className={classes.input}
                onChange={handleChange}
                onBlur={handleBlur}
                value={values.email}
                id="username"
                label="Username"
              />
              {errors.username && touched.username && errors.username}
              <Input
                fullWidth
                placeholder="Password"
                className={classes.input}
                onChange={handleChange}
                onBlur={handleBlur}
                value={values.password}
                id="password"
                label="Password"
              />
              {errors.password && touched.password && errors.password}
              <Button
                fullWidth
                variant="contained" 
                className={classes.button} 
                disabled={isSubmitting}
                type="submit"
              >
                Signin
              </Button>
            </form>
          )}
        </Formik>
      </div>
    </Container>
  )
}