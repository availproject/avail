use crate::{DaSamplingRequest, DaSamplingResponse};
use prost::Message;

pub const DA_SAMPLING_PROTOCOL: &str = "/avail/da-sampling/1";

pub fn encode_request(req: &DaSamplingRequest) -> Vec<u8> {
	let mut buf = Vec::with_capacity(req.encoded_len());
	req.encode(&mut buf).expect("prost encode");
	buf
}

pub fn decode_request(bytes: &[u8]) -> Result<DaSamplingRequest, String> {
	DaSamplingRequest::decode(bytes).map_err(|e| e.to_string())
}

pub fn encode_response(resp: &DaSamplingResponse) -> Vec<u8> {
	let mut buf = Vec::with_capacity(resp.encoded_len());
	resp.encode(&mut buf).expect("prost encode");
	buf
}

pub fn decode_response(bytes: &[u8]) -> Result<DaSamplingResponse, String> {
	DaSamplingResponse::decode(bytes).map_err(|e| e.to_string())
}
