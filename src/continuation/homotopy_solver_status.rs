use crate::core::ExitStatus;
/// Solver status of the homotopy method
///
#[derive(Debug)]
pub struct HomotopySolverStatus {
    exit_status: ExitStatus,
    num_outer_iterations: usize,
    num_inner_iterations: usize,
    last_problem_norm_fpr: f64,
    max_constraint_violation: f64,
    solve_time: std::time::Duration,
}

// TODO: add: time
impl HomotopySolverStatus {
    pub fn new(
        exit_status: ExitStatus,
        num_outer_iterations: usize,
        num_inner_iterations: usize,
        last_problem_norm_fpr: f64,
        max_constraint_violation: f64,
        solve_time: std::time::Duration,
    ) -> HomotopySolverStatus {
        HomotopySolverStatus {
            exit_status: exit_status,
            num_outer_iterations: num_outer_iterations,
            num_inner_iterations: num_inner_iterations,
            last_problem_norm_fpr: last_problem_norm_fpr,
            max_constraint_violation: max_constraint_violation,
            solve_time: solve_time,
        }
    }
}
