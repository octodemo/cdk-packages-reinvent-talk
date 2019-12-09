import { expect as expectCDK, haveResource } from '@aws-cdk/assert';
import cdk = require('@aws-cdk/core');
import CdkPackagesReinvent = require('../lib/cdk-packages-reinvent-stack');

test('Lambda Function Created', () => {
  const app = new cdk.App();
  // WHEN
  const stack = new CdkPackagesReinvent.CdkPackagesReinventStack(app, 'MyTestStack');
  // THEN
  expectCDK(stack).to(haveResource("AWS::Lambda::Function"));
});

// test('SNS Topic Created', () => {
//   const app = new cdk.App();
//   // WHEN
//   const stack = new CdkPackagesReinvent.CdkPackagesReinventStack(app, 'MyTestStack');
//   // THEN
//   expectCDK(stack).to(haveResource("AWS::SNS::Topic"));
// });