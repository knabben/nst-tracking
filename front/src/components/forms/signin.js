import React from 'react';
import Snackbar from '@material-ui/core/Snackbar';
import { Container } from '@material-ui/core';
import { Formik } from 'formik';
import { useStyles } from '../styles';
import { useDispatch, useSelector } from 'react-redux';
import { getNotifications } from '../../selectors/notification'
import { loginUser } from '../../actions/user'
import Input from '@material-ui/core/Input';
import Button from '@material-ui/core/Button';
import Typography from '@material-ui/core/Typography';
import CssBaseline from '@material-ui/core/CssBaseline';


const initialValues = {
  username: '',
  password: ''
};

const SigninForm = () => {
  const classes = useStyles();
  const dispatch = useDispatch()
  const notifications = useSelector(getNotifications)

  return (
    <div>
      { 
        notifications.map( (notification) => (
          <Snackbar
            key={notification.id}
            anchorOrigin={{
              vertical: 'bottom',
              horizontal: 'left',
            }}
            open={true}
            autoHideDuration={6000}
            ContentProps={{
              'aria-describedby': 'message-id',
            }}
            message={<span id="message-id">{notification.message}</span>}
          />
        ))
      }
      <Formik
        initialValues={initialValues}
        onSubmit={(values, actions) => {
          dispatch(loginUser(values))
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
          <Typography component="h1" variant="h5"> Signin </Typography>
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
              Submit
            </Button>
          </form>
        </div>
      </Container>
      )}
    </Formik>
  </div>
  )
}

export default SigninForm;