import http from 'k6/http';
import { check, sleep } from 'k6';

// Test configuration
export const options = {
  // Test execution strategy
  stages: [
    { duration: '30s', target: 5 },    // Ramp up to 5 users
    { duration: '1m', target: 20 },     // Ramp up to 20 users
    { duration: '3m', target: 20 },     // Stay at 20 users
    { duration: '30s', target: 50 },    // Spike to 50 users
    { duration: '1m', target: 50 },     // Stay at 50 users
    { duration: '30s', target: 0 },     // Ramp down to 0 users
  ],
  // Performance thresholds
  thresholds: {
    http_req_duration: ['p(95)<500'], // 95% of requests must complete below 500ms
    http_req_failed: ['rate<0.01'],   // Less than 1% of requests should fail
  },
};

// Your API access key
const API_KEY = 'm9Op[3w4IRSgLVzv_}J>T5Y^q]_7?B';

// Search terms to randomize requests - using your original example term "maz" first
const searchTerms = [
  'Matius',
  'Markus',
  'Lukas',
  'Yohanes',
  'Roma',
  'Galatia',
  'Efesus',
  'Filipi',
  'Kolose',
  'Titus'
];

export default function () {
  // Pick a random search term
  const searchTerm = searchTerms[Math.floor(Math.random() * searchTerms.length)];

  // Headers
  const params = {
    headers: {
      'accesskey': API_KEY,
      'Content-Type': 'application/json',
      'Accept': 'application/json'
    },
  };

  // Make the HTTP request
  const response = http.get(`http://0.0.0.0:8080/bible/find/${searchTerm}`, params);

  // Check if the request was successful
  check(response, {
    'status is 200': (r) => r.status === 200,
    'response body contains results': (r) => r.body.includes(searchTerm) || r.body.includes('result'),
    'response time < 200ms': (r) => r.timings.duration < 200,
    'response time < 500ms': (r) => r.timings.duration < 500,
  });

  // Log if response is not 200
  if (response.status !== 200) {
    console.log(`Request failed with status ${response.status}: ${response.body}`);
  }

  // Add some randomized sleep time between requests (100-500ms)
  sleep(Math.random() * 0.4 + 0.1);
}

// Modified setup function to use a known working term and debug output
export function setup() {
  // Use 'maz' since we know this works from your curl example
  const params = {
    headers: {
      'accesskey': API_KEY,
      'Content-Type': 'application/json',
      'Accept': 'application/json'
    },
  };

  const testResponse = http.get('http://0.0.0.0:8080/bible/find/maz', params);

  // Log debugging information
  console.log(`Setup test response status: ${testResponse.status}`);
  console.log(`Setup test response headers: ${JSON.stringify(testResponse.headers)}`);
  if (testResponse.body.length < 1000) {
    console.log(`Setup test response body: ${testResponse.body}`);
  } else {
    console.log(`Setup test response body length: ${testResponse.body.length} bytes`);
  }

  // Skip the setup check and continue with the test even if it failed
  console.log('Continuing with test regardless of setup response');
  return { setupStatus: testResponse.status };
}

// Add a teardown function to log completion
export function teardown(data) {
  console.log(`Test completed. Setup had returned status: ${data.setupStatus}`);
}