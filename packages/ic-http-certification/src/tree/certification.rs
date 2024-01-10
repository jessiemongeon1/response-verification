use crate::{
    request_hash, response_hash, DefaultCelBuilder, DefaultFullCelExpression,
    DefaultResponseOnlyCelExpression, HttpCertificationResult, HttpRequest, HttpResponse,
};
use ic_certification::Hash;
use ic_representation_independent_hash::hash;

/// A certified [request](crate::HttpResponse) and [response](crate::HttpResponse) pair.
///
/// It contains three variants:
///
/// - The [Skip](Certification::Skip) variant excludes both an [HTTP request](crate::HttpRequest) and the
/// corresponding [HTTP response](crate::HttpResponse) from certification. Create this variant using
/// the associated [skip()](Certification::skip()) function.
///
/// - The [ResponseOnly](Certification::ResponseOnly) variant includes an
/// [HTTP response](crate::HttpResponse) but excludes the corresponding [HTTP request](crate::HttpRequest)
/// from certification. Create this variant using the associated
/// [response_only()](Certification::response_only()) function.
///
/// - The [Full](Certification::Full) variant includes both an [HTTP response](crate::HttpResponse) and
/// the corresponding [HTTP request](crate::HttpRequest) in certification. Create this variant using
/// the [full()](Certification::full()) function.
#[derive(Debug)]
pub enum Certification {
    /// A certification that excludes both the [HTTP request](crate::HttpRequest) and
    /// the corresponding [HTTP response](crate::HttpResponse).
    ///
    /// The [cel_expr_hash](Certification::Skip::cel_expr_hash) property is the hash
    /// of a [CEL expression](crate::DefaultCelExpression::Skip) used to exclude both the
    /// [HTTP request](crate::HttpRequest) and the corresponding [HTTP response](crate::HttpResponse)
    /// from certification.
    Skip {
        /// The hash of a [CEL expression](crate::DefaultCelExpression::Skip) used to exclude both
        /// the [HTTP request](crate::HttpRequest) and [HTTP response](crate::HttpResponse) from
        /// certification.
        cel_expr_hash: Hash,
    },

    /// A certification that includes an [HTTP response](crate::HttpResponse), but excludes the
    /// corresponding [HTTP request](crate::HttpRequest).
    ///
    /// The [cel_expr_hash](Certification::ResponseOnly::cel_expr_hash) property is the hash
    /// of a [CEL expression](crate::DefaultCelExpression::ResponseOnly) used to include an
    /// [HTTP request](crate::HttpRequest) but exclude the corresponding
    /// [HTTP response](crate::HttpResponse) from certification.
    ///
    /// The [response_hash](Certification::ResponseOnly::response_hash) property is the
    /// hash of the [HTTP response](crate::HttpResponse) calculated according to a
    /// [CEL expression](crate::DefaultCelExpression::ResponseOnly).
    ///
    /// The [CEL expression](crate::DefaultCelExpression::ResponseOnly) used to produce
    /// [response_hash](Certification::ResponseOnly::response_hash)
    /// is also used to produce the
    /// [cel_expr_hash](Certification::ResponseOnly::cel_expr_hash).
    ResponseOnly {
        /// The hash of a [CEL expression](crate::DefaultCelExpression::ResponseOnly) used to include an
        /// [HTTP request](crate::HttpRequest) but exclude the corresponding
        /// [HTTP response](crate::HttpResponse) from certification.
        ///
        /// The [CEL expression](crate::DefaultCelExpression::ResponseOnly) that produces this hash
        /// is also used to produce the
        /// [HTTP response hash](Certification::ResponseOnly::response_hash).
        cel_expr_hash: Hash,

        /// The
        /// [Representation Independent Hash](https://internetcomputer.org/docs/current/references/ic-interface-spec/#hash-of-map)
        /// of an [HTTP response](crate::HttpResponse), calculated according to a
        /// [CEL expression](crate::DefaultCelExpression::ResponseOnly).
        ///
        /// The [CEL expression](crate::DefaultCelExpression::ResponseOnly) used to calculate the hash of
        /// this [response](crate::HttpResponse), is also used to produce the
        /// [cel_expr_hash](Certification::ResponseOnly::cel_expr_hash) property.
        response_hash: Hash,
    },

    /// A certification that includes both an [HTTP response](crate::HttpResponse) and the corresponding
    /// [HTTP request](crate::HttpRequest).
    ///
    /// The [cel_expr_hash](Certification::Full::cel_expr_hash) property is the hash
    /// of a [CEL expression](crate::DefaultCelExpression::Full) used to include both the
    /// [HTTP request](crate::HttpRequest) and the corresponding [HTTP response](crate::HttpResponse)
    /// in certification.
    ///
    /// The [response_hash](Certification::Full::response_hash) property is the
    /// hash of the [HTTP response](crate::HttpResponse) calculated according to a
    /// [CEL expression](crate::DefaultCelExpression::Full).
    ///
    /// The [request_hash](Certification::Full::request_hash) property is the hash of a
    /// [HTTP response](crate::HttpResponse) calculated according to a
    /// [CEL expression](crate::DefaultCelExpression::Full).
    ///
    /// The [CEL expression](crate::DefaultCelExpression::Full) used to produce both
    /// [response_hash](Certification::Full::response_hash) and
    /// [request_hash](Certification::Full::request_hash) is also used to produce the
    /// [cel_expr_hash](Certification::Full::cel_expr_hash).
    Full {
        /// The hash of a [CEL expression](crate::DefaultCelExpression::Full) used to include an
        /// [HTTP request](crate::HttpRequest) but exclude the corresponding
        /// [HTTP response](crate::HttpResponse) from certification.
        ///
        /// The [CEL expression](crate::DefaultCelExpression::Full) that produces this hash
        /// is also used to produce the
        /// [HTTP response hash](Certification::Full::response_hash) and the
        /// [HTTP request hash](Certification::Full::request_hash).
        cel_expr_hash: Hash,

        /// The
        /// [Representation Independent Hash](https://internetcomputer.org/docs/current/references/ic-interface-spec/#hash-of-map)
        /// of an [HTTP response](crate::HttpResponse), calculated according to a
        /// [CEL expression](crate::DefaultCelExpression::Full).
        ///
        /// The [CEL expression](crate::DefaultCelExpression::Full) used to calculate the hash of
        /// this [request](crate::HttpRequest), is also used to produce the
        /// [cel_expr_hash](Certification::Full::cel_expr_hash) property.
        request_hash: Hash,

        /// The
        /// [Representation Independent Hash](https://internetcomputer.org/docs/current/references/ic-interface-spec/#hash-of-map)
        /// of an [HTTP response](crate::HttpResponse), calculated according to a
        /// [CEL expression](crate::DefaultCelExpression::Full).
        ///
        /// The [CEL expression](crate::DefaultCelExpression::Full) used to calculate the hash of
        /// this [response](crate::HttpResponse), is also used to produce the
        /// [cel_expr_hash](Certification::Full::cel_expr_hash) property.
        response_hash: Hash,
    },
}

impl Certification {
    /// Creates the [Skip](Certification::Skip) variant of the [Certification] enum, excluding both an
    /// [HTTP request](crate::HttpRequest) and the corresponding [HTTP response](crate::HttpResponse)
    /// from certification.
    pub fn skip() -> HttpCertificationResult<Certification> {
        let cel_expr = DefaultCelBuilder::skip_certification().to_string();
        let cel_expr_hash = hash(&cel_expr.as_bytes());

        Ok(Certification::Skip { cel_expr_hash })
    }

    /// Creates the [ResponseOnly](Certification::ResponseOnly) variant of the [Certification] enum,
    /// including an [HTTP response](crate::HttpResponse) but excluding the corresponding
    /// [HTTP request](crate::HttpRequest) from certification.
    pub fn response_only(
        cel_expr: &DefaultResponseOnlyCelExpression,
        response: &HttpResponse,
        response_body_hash: Option<Hash>,
    ) -> HttpCertificationResult<Certification> {
        let cel_expr_hash = hash(cel_expr.to_string().as_bytes());
        let response_hash = response_hash(response, &cel_expr.response, response_body_hash);

        Ok(Certification::ResponseOnly {
            cel_expr_hash,
            response_hash,
        })
    }

    /// Creates the [Full](Certification::Full) variant of the [Certification] enum, including both an
    /// [HTTP request](crate::HttpRequest) and the corresponding [HTTP request](crate::HttpRequest)
    /// in certification.
    pub fn full(
        cel_expr: &DefaultFullCelExpression,
        request: &HttpRequest,
        response: &HttpResponse,
        response_body_hash: Option<Hash>,
    ) -> HttpCertificationResult<Certification> {
        let cel_expr_hash = hash(cel_expr.to_string().as_bytes());
        let request_hash = request_hash(request, &cel_expr.request)?;
        let response_hash = response_hash(response, &cel_expr.response, response_body_hash);

        Ok(Certification::Full {
            cel_expr_hash,
            request_hash,
            response_hash,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DefaultResponseCertification;
    use rstest::*;

    #[rstest]
    fn no_certification() {
        let cel_expr = DefaultCelBuilder::skip_certification().to_string();
        let expected_cel_expr_hash = hash(cel_expr.as_bytes());

        let result = Certification::skip().unwrap();

        assert!(matches!(
            result,
            Certification::Skip { cel_expr_hash } if cel_expr_hash == expected_cel_expr_hash
        ));
    }

    #[rstest]
    fn response_only_certification() {
        let cel_expr = DefaultCelBuilder::response_only_certification()
            .with_response_certification(DefaultResponseCertification::certified_response_headers(
                &["ETag", "Cache-Control"],
            ))
            .build();
        let expected_cel_expr_hash = hash(cel_expr.to_string().as_bytes());

        let response = &HttpResponse {
            status_code: 200,
            body: vec![],
            headers: vec![],
        };
        let expected_response_hash = response_hash(response, &cel_expr.response, None);

        let result = Certification::response_only(&cel_expr, response, None).unwrap();

        assert!(matches!(
            result,
            Certification::ResponseOnly {
                cel_expr_hash,
                response_hash
            } if cel_expr_hash == expected_cel_expr_hash &&
                response_hash == expected_response_hash
        ))
    }

    #[rstest]
    fn full_certification() {
        let cel_expr = DefaultCelBuilder::full_certification()
            .with_request_headers(&["If-Match"])
            .with_request_query_parameters(&["foo", "bar", "baz"])
            .with_response_certification(DefaultResponseCertification::certified_response_headers(
                &["ETag", "Cache-Control"],
            ))
            .build();
        let expected_cel_expr_hash = hash(cel_expr.to_string().as_bytes());

        let request = &HttpRequest {
            body: vec![],
            headers: vec![],
            method: "GET".to_string(),
            url: "/index.html".to_string(),
        };
        let expected_request_hash = request_hash(request, &cel_expr.request).unwrap();

        let response = &HttpResponse {
            status_code: 200,
            body: vec![],
            headers: vec![],
        };
        let expected_response_hash = response_hash(response, &cel_expr.response, None);

        let result = Certification::full(&cel_expr, request, response, None).unwrap();

        assert!(matches!(
            result,
            Certification::Full {
                cel_expr_hash,
                request_hash,
                response_hash
            } if cel_expr_hash == expected_cel_expr_hash &&
                request_hash == expected_request_hash &&
                response_hash == expected_response_hash
        ))
    }
}