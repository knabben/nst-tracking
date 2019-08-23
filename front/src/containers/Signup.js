// import LoginForm from '../components/forms/signin'
// import { withFormik } from 'formik';
// import { registerUser } from '../actions'
// import { connect } from 'react-redux'

// const initialValues = {
//   username: '',
//   password: ''
// };

// const mapStateToProps = (state) => ({title: "Signup"})
// const mapDispatchToProps = dispatch => ({
//   registerUser(user, password) { 
//     dispatch(registerUser(user, password)) 
//   },
// });

// const Signup = connect(
//   mapStateToProps, 
//   mapDispatchToProps
// )(withFormik({
//   mapPropsToValues: () => initialValues,
//   handleSubmit: (values, actions) => {
//     actions.props.registerUser(
//       values.username, 
//       values.password
//     )
//     actions.setSubmitting(false)
//   }
// })(LoginForm))

// export default Signup;