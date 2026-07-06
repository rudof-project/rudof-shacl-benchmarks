use serde::Serialize;

#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum IterationResultStatus {
    Ok, // Iteration completed successfully
    Error, // Some error occurred
    // Oom, // Out of memory error
    Dnf, // Run exceeded timeout
}

#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum ResultStatus {
    Success, // All runs are OK
    Partial, // More than `min_valid_iter` runs are OK
    Failed, // There are not enough OK cases
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IterationResults {
    pub iteration: usize,
    pub load_time: Option<f64>,
    pub validation_time: Option<f64>,
    pub iteration_results: IterationResultStatus,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Results {
    pub iteration_results: Vec<IterationResults>,
    pub status: ResultStatus,
    pub load_mean: Option<f64>,
    pub load_std: Option<f64>,
    pub validation_mean: Option<f64>,
    pub validation_std: Option<f64>,
}

pub struct ResultState {
    min_iter: usize,
    results: Vec<IterationResults>,
}

impl ResultState {
    pub fn new(min_iter: usize) -> Self {
        Self { min_iter, results: Vec::new() }
    }

    pub fn add_result(&mut self, result: IterationResults) {
        self.results.push(result);
    }

    fn mean_of(times: &[f64]) -> Option<f64> {
        if times.is_empty() {
            return None;
        }
        Some(times.iter().sum::<f64>() / times.len() as f64)
    }

    fn std_of(times: &[f64]) -> Option<f64> {
        if times.is_empty() {
            return None;
        }
        let mean = times.iter().sum::<f64>() / times.len() as f64;
        let variance = times.iter().map(|t| (t - mean).powi(2)).sum::<f64>() / times.len() as f64;
        Some(variance.sqrt())
    }

    pub fn generate_results(&self) -> Results {
        let ok_results: Vec<&IterationResults> = self
            .results
            .iter()
            .filter(|r| r.iteration_results == IterationResultStatus::Ok)
            .collect();

        let load_times: Vec<f64> = ok_results.iter().filter_map(|r| r.load_time).collect();
        let validation_times: Vec<f64> = ok_results.iter().filter_map(|r| r.validation_time).collect();

        let ok_count = ok_results.len();
        let status = if ok_count == self.results.len() {
            ResultStatus::Success
        } else if ok_count >= self.min_iter {
            ResultStatus::Partial
        } else {
            ResultStatus::Failed
        };

        Results {
            iteration_results: self.results.clone(),
            status,
            load_mean: Self::mean_of(&load_times),
            load_std: Self::std_of(&load_times),
            validation_mean: Self::mean_of(&validation_times),
            validation_std: Self::std_of(&validation_times),
        }
    }
}
