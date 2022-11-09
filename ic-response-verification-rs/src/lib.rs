#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
extern crate console_error_panic_hook;
#[cfg(target_arch = "wasm32")]
use std::panic;

use certificate_header::CertificateHeader;
use request::Request;
use response::Response;

pub mod request;
pub mod response;

mod certificate_header;
mod certificate_header_field;
mod logger;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = verifyRequestResponsePair)]
pub fn verify_request_response_pair(request: JsValue, response: JsValue) -> bool {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let request = Request::from(request);
    let response = Response::from(response);

    verify_request_response_pair_impl(request, response)
}

#[cfg(not(target_arch = "wasm32"))]
pub use verify_request_response_pair_impl as verify_request_response_pair;

pub fn verify_request_response_pair_impl(request: Request, response: Response) -> bool {
    log!("Rust Request: {:?}", request);
    log!("Rust Response: {:?}", response);

    if let Some((_, value)) = response
        .headers
        .iter()
        .find(|(name, _)| name == "Ic-Certificate")
    {
        let certificate_header = CertificateHeader::from(value);

        return certificate_header.certificate.is_some() && certificate_header.tree.is_some();
    }

    false
}