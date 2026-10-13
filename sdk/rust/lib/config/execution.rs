//! Global execution buffering configuration.

use microsandbox_protocol::exec::{EXEC_STDIN_WINDOW, MIN_EXEC_OUTPUT_WINDOW};
use microsandbox_types::ConfigPatch;
use serde::{Deserialize, Serialize};

use crate::{MicrosandboxError, MicrosandboxResult};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Host-side defaults for command input and output, shared by the CLI and SDKs.
#[derive(Debug, Clone, Serialize, Deserialize, ConfigPatch)]
#[serde(default)]
#[config_patch(serde)]
pub struct ExecConfig {
    /// Maximum unread output buffered per command, in bytes, including queue overhead.
    ///
    /// Defaults to 1 MiB; `None` disables the limit. A limit must be at least 64 KiB. Stdout and
    /// stderr share it; output retained by callers and transport buffers is excluded.
    pub output_limit: Option<u32>,

    /// Maximum stdin bytes awaiting delivery to the command, per execution.
    /// Defaults to 64 KiB; the supported range is 16–64 KiB. Large writes wait
    /// for capacity instead of being truncated.
    pub input_limit: u32,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl ExecConfig {
    pub(crate) fn validate(&self) -> MicrosandboxResult<()> {
        if self
            .output_limit
            .is_some_and(|bytes| bytes < MIN_EXEC_OUTPUT_WINDOW)
        {
            return Err(MicrosandboxError::InvalidConfig(format!(
                "exec.output_limit must be at least {MIN_EXEC_OUTPUT_WINDOW} bytes"
            )));
        }
        if !(16 * 1024..=EXEC_STDIN_WINDOW).contains(&self.input_limit) {
            return Err(MicrosandboxError::InvalidConfig(
                "exec.input_limit must be between 16384 and 65536 bytes".into(),
            ));
        }
        Ok(())
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for ExecConfig {
    fn default() -> Self {
        Self {
            output_limit: Some(1024 * 1024),
            input_limit: EXEC_STDIN_WINDOW,
        }
    }
}
