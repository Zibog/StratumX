use engine_core::{EngineCoreError, EngineCoreResult};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferConfig {
    pub max_inflight_decodes: usize,
    pub max_inflight_uploads: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferRequest {
    pub asset_key: u64,
    pub compressed_bytes: usize,
    pub decoded_bytes: usize,
    pub upload_bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferResult {
    pub accepted: bool,
    pub inflight_decodes: usize,
    pub inflight_uploads: usize,
}

#[derive(Debug)]
pub struct TransferControlService {
    config: TransferConfig,
    decode_queue: VecDeque<TransferRequest>,
    upload_queue: VecDeque<TransferRequest>,
}

impl TransferControlService {
    pub fn new(config: TransferConfig) -> Self {
        Self {
            config,
            decode_queue: VecDeque::new(),
            upload_queue: VecDeque::new(),
        }
    }
    pub fn submit(&mut self, request: TransferRequest) -> EngineCoreResult<TransferResult> {
        if request.compressed_bytes == 0 || request.upload_bytes == 0 {
            return Err(EngineCoreError::InvalidDescriptor(
                "transfer request requires non-zero compressed and upload bytes",
            ));
        }
        if self.decode_queue.len() >= self.config.max_inflight_decodes {
            return Err(EngineCoreError::InvalidDescriptor(
                "decode queue exceeds configured ceiling",
            ));
        }
        if self.upload_queue.len() >= self.config.max_inflight_uploads {
            return Err(EngineCoreError::InvalidDescriptor(
                "upload queue exceeds configured ceiling",
            ));
        }
        self.decode_queue.push_back(request.clone());
        self.upload_queue.push_back(request);
        Ok(TransferResult {
            accepted: true,
            inflight_decodes: self.decode_queue.len(),
            inflight_uploads: self.upload_queue.len(),
        })
    }
    pub fn complete_decode(&mut self) -> Option<TransferRequest> {
        self.decode_queue.pop_front()
    }
    pub fn complete_upload(&mut self) -> Option<TransferRequest> {
        self.upload_queue.pop_front()
    }
}
