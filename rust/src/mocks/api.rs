use random_number::rand::distributions::Alphanumeric;
use random_number::rand::{thread_rng, Rng};

// Mock API that simulates returning some MPNs after search.

pub struct PartSearchAPI {}

#[derive(Debug)]
pub struct Part {
    pub mpn: String,
}

#[derive(Debug)]
pub struct ExternalAPIError(Option<String>);

impl PartSearchAPI {
    pub fn search(&self, search_query: &str) -> Result<Vec<Part>, ExternalAPIError> {
        let n: u8 = random!(..=10);
        let mut parts = Vec::new();
        for _ in 0..n {
            parts.push(generate_random_part_starting_with(&search_query))
        }

        info!(
            "List of mpns received from mock api: {:?}",
            parts.iter().map(|p| &p.mpn)
        );

        Ok(parts)
    }
}

fn generate_random_part_starting_with(start_string: &str) -> Part {
    Part {
        mpn: format!(
            "{}{}",
            start_string,
            thread_rng()
                .sample_iter(Alphanumeric)
                .take(5)
                .map(char::from)
                .collect::<String>()
        ),
    }
}

#[cfg(test)]
mod tests {
    mod unit {
        use crate::mocks::api::PartSearchAPI;

        #[test]
        fn test_mpn_querying() {
            let mock_api = PartSearchAPI {};
            let search_query = "abc";

            let returned_parts = mock_api.search(search_query).unwrap();

            assert!(returned_parts
                .iter()
                .all(|part| part.mpn.contains(search_query)))
        }
    }
}
