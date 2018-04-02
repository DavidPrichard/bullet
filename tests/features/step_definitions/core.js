const {Then, When, setWorldConstructor} = require('cucumber');
const assert = require('assert');
const {exec} = require('child-process-promise');

function World() {
  this.exitcode = null;
  this.stderr = "???";
  this.stdout = "???";
}

setWorldConstructor(World);


When('I run the command {string}', function (cmd) {
    // Write code here that turns the phrase above into concrete actions
    let self = this;
    return exec(cmd).then( 
      result => {
        self.stdout = result.stdout.toString();
        self.stderr = result.stderr.toString();
        self.exitcode = 0;
      },
      error => {
        self.stdout = error.stdout.toString();
        self.stderr = error.stderr.toString();
        self.exitcode = error.code;
      }
  );
});

Then('I should see the following output:', function (docString) {
    // Write code here that turns the phrase above into concrete actions
    assert.equal(docString, this.stdout);
  });

Then('I should see the following error:', function (docString) {
    // Write code here that turns the phrase above into concrete actions
    assert.equal(docString, this.stderr);
  });

  Then('have an exit code of {string}', function (exitcode) {
    // Write code here that turns the phrase above into concrete actions
    assert.equal(this.exitcode, new Number(exitcode));
  });
