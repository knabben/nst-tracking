import React from 'react';
import { Formik } from 'formik';
import { Container } from '@material-ui/core';
import { useStyles } from './styles';
import Input from '@material-ui/core/Input';
import Button from '@material-ui/core/Button';
import Typography from '@material-ui/core/Typography';
import CssBaseline from '@material-ui/core/CssBaseline';

export default function SignupForm({
  handleSubmit, 
  handleChange,
  handleBlur, 
  values, 
  errors,
  touched,
  isSubmitting
}) {
  const classes = useStyles();
  return (
    <Container component="main" maxWidth="xs">
      <CssBaseline />
      <div className={classes.paper}>
        <Typography component="h1" variant="h5">
          Sign up
        </Typography>
        <form onSubmit={handleSubmit} noValidate autoComplete="off">
          <Input
            fullWidth
            placeholder="Username"
            className={classes.input}
            onChange={handleChange}
            onBlur={handleBlur}
            value={values.username}
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
            Signup
          </Button>
        </form>
      </div>
    </Container>
  )
}