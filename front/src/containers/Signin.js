import React from 'react';
import LoginForm from '../components/forms/Login'
import { withFormik } from 'formik';
import { loginUser } from '../actions'
import { connect } from 'react-redux'

const initialValues = {
  username: '',
  password: ''
};

const mapStateToProps = undefined;
const mapDispatchToProps = dispatch => ({
  loginUser(user, password) { 
    dispatch(loginUser(user, password)) 
  },
});

const Signin = connect(
  mapStateToProps, 
  mapDispatchToProps
)(withFormik({
  title: "Signin",
  mapPropsToValues: () => initialValues,
  handleSubmit: (values, actions) => {
    actions.props.loginUser(
      values.username, 
      values.password
    )
    actions.setSubmitting(false)
  }
})(LoginForm))

export default Signin;