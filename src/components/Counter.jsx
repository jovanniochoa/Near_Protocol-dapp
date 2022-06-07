import React from 'react';
import PropTypes from 'prop-types';

export default function Counter({ onIncrease }) {
  return (
    <form onIncrease={onIncrease}>
        <p>
            <button type = "counter submit"> 
                Counter Sign 
            </button>
        </p>
    </form>
  );
}

Counter.propTypes = {
  onIncrease: PropTypes.func.isRequired
};
