import Home from '../components/Home'
import { connect } from 'react-redux'

const mapStateToProps = (state) => ({
  authenticated: false //state.login.hasOwnProperty("token")
})

const HomeContainer = connect(mapStateToProps)(Home);
export default HomeContainer;