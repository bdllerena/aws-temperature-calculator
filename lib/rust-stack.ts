import { Duration, Stack, StackProps } from 'aws-cdk-lib';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import { Construct } from 'constructs';

export class RustStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    let fn = new lambda.Function(this, 'LambdaRustStack', {
      code: lambda.Code.fromAsset(
        'lambda/target/aarch64-unknown-linux-gnu/release/lambda'
      ),
      functionName: "temperature-calculator-dev",
      handler: 'main',
      memorySize: 1024,
      environment: {
        RUST_BACKTRACE: '1',
      },
      runtime: lambda.Runtime.PROVIDED_AL2,
      architecture: lambda.Architecture.ARM_64,
      timeout: Duration.seconds(300),
    });
    
    fn.addFunctionUrl({
      authType: lambda.FunctionUrlAuthType.NONE,
    });
  }
}
