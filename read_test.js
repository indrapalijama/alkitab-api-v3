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

// Your API access key - read from environment variable
const API_KEY = __ENV.SECRET || 'm9Op[3w4IRSgLVzv_}J>T5Y^q]_7?B';

// Test data for randomizing requests
const books = [
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

// Chapter ranges for each book (approximate)
const chapterRanges = {
  'Matius': 28,
  'Markus': 16,
  'Lukas': 24,
  'Yohanes': 21,
  'Roma': 16,
  'Galatia': 6,
  'Efesus': 6,
  'Filipi': 4,
  'Kolose': 4,
  'Titus': 3
};

export default function () {
  // Pick a random book
  const book = books[Math.floor(Math.random() * books.length)];
  
  // Get the chapter range for this book
  const maxChapter = chapterRanges[book] || 20;
  
  // Pick a random chapter (1 to maxChapter)
  const chapter = Math.floor(Math.random() * maxChapter) + 1;

  // Headers
  const params = {
    headers: {
      'accesskey': API_KEY,
      'Content-Type': 'application/json',
      'Accept': 'application/json'
    },
  };

  // Make the HTTP request
  const response = http.get(`http://0.0.0.0:8080/bible/read/${book}/${chapter}`, params);

  // Log request details
  console.log(`Making request to: /bible/read/${book}/${chapter}`);
  console.log(`Book: ${book}, Chapter: ${chapter}, Max Chapter: ${maxChapter}`);

  // Check if the request was successful
  check(response, {
    'status is 200': (r) => r.status === 200,
    'response body contains book name': (r) => {
      const body = JSON.parse(r.body);
      return body.book && body.book.includes(book);
    },
    'response body contains chapter': (r) => {
      const body = JSON.parse(r.body);
      return body.chapter === chapter;
    },
    'response has verses': (r) => {
      const body = JSON.parse(r.body);
      return body.verses && body.verses.length > 0;
    },
    'response time < 200ms': (r) => r.timings.duration < 200,
    'response time < 500ms': (r) => r.timings.duration < 500,
  });

  // Log response details
  if (response.status !== 200) {
    console.log(`Request failed with status ${response.status}`);
    console.log(`Response body: ${response.body}`);
    console.log(`Response headers: ${JSON.stringify(response.headers)}`);
  } else {
    const body = JSON.parse(response.body);
    console.log(`Request successful - Response time: ${response.timings.duration}ms`);
    console.log(`Book: ${body.book.join(', ')}`);
    console.log(`Chapter: ${body.chapter}`);
    console.log(`Total verses: ${body.total_verses}`);
    console.log(`Version: ${body.version || 'default'}`);
  }

  // Add some randomized sleep time between requests (100-500ms)
  sleep(Math.random() * 0.4 + 0.1);
}

// Setup function to test a known working endpoint
export function setup() {
  const params = {
    headers: {
      'accesskey': API_KEY,
      'Content-Type': 'application/json',
      'Accept': 'application/json'
    },
  };

  const testResponse = http.get('http://0.0.0.0:8080/bible/read/Matius/1', params);

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