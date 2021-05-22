const process = require('process');

const includes_fail = process.argv.includes('fail');
if (includes_fail) {
  process.exit(1);
}
else {
  console.log('hello world!');
}
