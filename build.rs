// Copyright (c) Policy-Agent Authors.
// Licensed under the Apache 2.0 license.
use anyhow::Result;

fn main() -> Result<()> {
    // Copy hooks to appropriate location so that git will run them.
    std::fs::copy("./scripts/pre-commit", "./.git/hooks/pre-commit")?;
    std::fs::copy("./scripts/pre-push", "./.git/hooks/pre-push")?;

    tonic_build::compile_protos("./protos/policy_agent.proto")?;

    Ok(())
}
