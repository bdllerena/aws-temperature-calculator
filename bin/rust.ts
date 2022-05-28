#!/usr/bin/env node
import * as cdk from 'aws-cdk-lib';
import { RustStack } from '../lib/rust-stack';

const app = new cdk.App();
new RustStack(app, 'RustStack');
