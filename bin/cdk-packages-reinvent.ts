#!/usr/bin/env node
import cdk = require('@aws-cdk/core');
import { CdkPackagesReinventStack } from '../lib/cdk-packages-reinvent-stack';

const app = new cdk.App();
new CdkPackagesReinventStack(app, 'CdkPackagesReinventStack');