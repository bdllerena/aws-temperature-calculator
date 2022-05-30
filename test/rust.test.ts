import * as cdk from 'aws-cdk-lib';
import { Template, Match } from 'aws-cdk-lib/assertions';
import * as Rust from '../lib/rust-stack';

test('Lambda Created', () => {
  const app = new cdk.App();
  // WHEN
  const stack = new Rust.RustStack(app, 'LambdaRustStack');
  // THEN

  const template = Template.fromStack(stack);

  template.hasResourceProperties('AWS::Lambda::Function', {
    MemorySize: 1024
  });
});
