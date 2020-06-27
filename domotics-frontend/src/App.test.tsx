import React from 'react';
import { render } from '@testing-library/react';
import App from './App';

test('basic test', () => {
  const { getByText } = render(<App />);
  const light = getByText(/Lights/i);
  expect(light).toBeInTheDocument();
});
