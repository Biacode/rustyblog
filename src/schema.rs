// Production schema
#[cfg(not(test))]
infer_schema!("dotenv:DATABASE_URL");

// Test schema
#[cfg(test)]
infer_schema!("dotenv:DATABASE_URL_TEST");