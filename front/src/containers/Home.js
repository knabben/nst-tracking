import Home from '../components/Home'
import { isAuthenticated } from '../selectors/user'
import { connect } from 'react-redux'

const mapStateToProps = (state) => ({
  authenticated: isAuthenticated(state),
})

const HomeContainer = connect(mapStateToProps)(Home);
export default HomeContainer;