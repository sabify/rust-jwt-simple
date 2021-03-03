use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jwt_simple::prelude::*;
use std::collections::HashSet;

pub fn criterion_benchmark(c: &mut Criterion) {
    let claims = Claims::create(Duration::from_secs(86400))
        .with_issuer("z")
        .with_audiences(
            [
                "x".to_string(),
                "y".to_string(),
                "z".to_string(),
                "p".to_string(),
            ]
            .iter()
            .cloned()
            .collect(),
        );
    let options = VerificationOptions {
        allowed_issuers: Some(vec!["x".to_string(), "y".to_string(), "z".to_string()]),
        allowed_audiences: Some(vec![
            "x".to_string(),
            "y".to_string(),
            "z".to_string(),
            "p".to_string(),
        ]),
        ..Default::default()
    };

    c.bench_function("VerificationOptionsWithVec", |b| {
        b.iter(|| {
            if let Some(allowed_issuers) = &options.allowed_issuers {
                if let Some(issuer) = &claims.issuer {
                    allowed_issuers.contains(issuer);
                }
            }
            if let Some(allowed_audiences) = &options.allowed_audiences {
                if let Some(audiences) = &claims.audiences {
                    audiences.contains(allowed_audiences);
                }
            }
        })
    });

    let allowed_issuers: Option<HashSet<String>> = Some(
        ["x".to_string(), "y".to_string(), "z".to_string()]
            .iter()
            .cloned()
            .collect(),
    );
    let allowed_audiences: Option<HashSet<String>> = Some(
        [
            "x".to_string(),
            "y".to_string(),
            "z".to_string(),
            "p".to_string(),
        ]
        .iter()
        .cloned()
        .collect(),
    );

    c.bench_function("VerificationOptionsWithSet", |b| {
        b.iter(|| {
            if let Some(allowed_issuers) = &allowed_issuers {
                if let Some(issuer) = &claims.issuer {
                    allowed_issuers.contains(issuer);
                }
            }
            if let Some(allowed_audiences) = &allowed_audiences {
                if let Some(audiences) = &claims.audiences {
                    match audiences {
                        Audiences::AsString(audience) => allowed_audiences.contains(audience),
                        Audiences::AsSet(audiences) => {
                            audiences.intersection(allowed_audiences).next().is_some()
                        }
                    };
                }
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
