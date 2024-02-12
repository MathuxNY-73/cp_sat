use std::collections::HashMap;

use cp_sat::builder::{solver_stats, BoolVar, CpModelBuilder, LinearExpr};
use cp_sat::proto::{CpSolverStatus, SatParameters};

use env_logger::Env;

fn interval_sample_sat() {
    let mut model_builder = CpModelBuilder::default();

    const HORIZON: i64 = 21;  // 3 weeks.
    const DOMAIN: [(i64, i64); 1] = [(0, HORIZON)];

    // Task 0, duration 2.
    let start_0 = model_builder.new_int_var(DOMAIN);
    let duration_0 = 2;
    let end_0 = model_builder.new_int_var(DOMAIN);
    let task_0 = model_builder.new_interval_var(start_0, duration_0, end_0);

    // Task 1, duration 4.
    let start_1 = model_builder.new_int_var(DOMAIN);
    let duration_1 = 4;
    let end_1 = model_builder.new_int_var(DOMAIN);
    let task_1 = model_builder.new_interval_var(start_1, duration_1, end_1);

    // Task 2, duration 3.
    let start_2 = model_builder.new_int_var(DOMAIN);
    let duration_2 = 3;
    let end_2 = model_builder.new_int_var(DOMAIN);
    let task_2 = model_builder.new_interval_var(start_2, duration_2, end_2);

    // Week ends.
    let weekend_0 = model_builder.new_interval_var(5, 2, 7);
    let weekend_1 = model_builder.new_interval_var(12, 2, 14);
    let weekend_2 = model_builder.new_interval_var(19, 2, 21);

    // No overlap constraint.
    model_builder.add_no_overlap([task_0, task_1, task_2, weekend_0, weekend_1, weekend_2]);

    // Makespan.
    let makespan = model_builder.new_int_var(DOMAIN);
    model_builder.add_le(end_0, makespan);
    model_builder.add_le(end_1, makespan);
    model_builder.add_le(end_2, makespan);

    model_builder.minimize(makespan);

    // Solving part.
    let response = model_builder.solve();
    log::info!("{}", solver_stats(&response, true));

    if response.status() == CpSolverStatus::Optimal {
        log::info!("Optimal Schedule Length: {}", response.objective_value);
        log::info!("Task 0 starts at {}", start_0.solution_value(&response));
        log::info!("Task 1 starts at {}", start_1.solution_value(&response));
        log::info!("Task 2 starts at {}", start_2.solution_value(&response));
    }
}

fn nurse_scheduling() {
    const NUM_NURSES: i64 = 5;
    const NUM_SHIFTS: i64 = 3;
    const NUM_DAYS: i64 = 7;

    let all_nurses: Vec<_> = (0..NUM_NURSES).collect();
    let all_shifts: Vec<_> = (0..NUM_SHIFTS).collect();
    let all_days: Vec<_> = (0..NUM_DAYS).collect();

    let shift_requests = [
        [
            [0, 0, 1],
            [0, 0, 0],
            [0, 0, 0],
            [0, 0, 0],
            [0, 0, 1],
            [0, 1, 0],
            [0, 0, 1],
        ],
        [
            [0, 0, 0],
            [0, 0, 0],
            [0, 1, 0],
            [0, 1, 0],
            [1, 0, 0],
            [0, 0, 0],
            [0, 0, 1],
        ],
        [
            [0, 1, 0],
            [0, 1, 0],
            [0, 0, 0],
            [1, 0, 0],
            [0, 0, 0],
            [0, 1, 0],
            [0, 0, 0],
        ],
        [
            [0, 0, 1],
            [0, 0, 0],
            [1, 0, 0],
            [0, 1, 0],
            [0, 0, 0],
            [1, 0, 0],
            [0, 0, 0],
        ],
        [
            [0, 0, 0],
            [0, 0, 1],
            [0, 1, 0],
            [0, 0, 0],
            [1, 0, 0],
            [0, 1, 0],
            [0, 0, 0],
        ],
    ];

    let mut cp_model = CpModelBuilder::default();

    let mut shifts = HashMap::<(&i64, &i64, &i64), BoolVar>::default();

    for nurse in &all_nurses {
        for day in &all_days {
            for shift in &all_shifts {
                shifts.insert(
                    (nurse, day, shift),
                    cp_model.new_bool_var_with_name(
                        format!("shift_n{}d{}s{}", nurse, day, shift)));
            }
        }
    }

    for day in &all_days {
        for shift in &all_shifts {
            cp_model.add_exactly_one(all_nurses.iter().map(|n| shifts[&(n, day, shift)]));
        }
    }

    for nurse in &all_nurses {
        for day in &all_days {
            cp_model.add_at_most_one(all_shifts.iter().map(|s| shifts[&(nurse, day, s)]));
        }
    }

    // Try to distribute the shifts evenly, so that each nurse works
    // min_shifts_per_nurse shifts. If this is not possible, because the total
    // number of shifts is not divisible by the number of nurses, some nurses will
    // be assigned one more shift.
    let min_shifts_per_nurse = (NUM_SHIFTS * NUM_DAYS) / NUM_NURSES;
    let max_shifts_per_nurse: i64 = if (NUM_SHIFTS * NUM_DAYS) % NUM_NURSES == 0 {
        min_shifts_per_nurse
    } else {
        min_shifts_per_nurse + 1
    };

    for nurse in &all_nurses {
        let mut shifts_worked = Vec::<BoolVar>::new();
        for day in &all_days {
            for shift in &all_shifts {
                shifts_worked.push(shifts[&(nurse, day, shift)]);
            }
        }
        cp_model.add_le(min_shifts_per_nurse, LinearExpr::from_iter(shifts_worked.clone()));//map(|v| cp_sat::builder::LinearExpr::from(*v)).sum());
        cp_model.add_le(LinearExpr::from_iter(shifts_worked), max_shifts_per_nurse);
    }

    let mut objective_expr = LinearExpr::default();
    for nurse in &all_nurses {
        for day in &all_days {
            for shift in &all_shifts {
                if shift_requests[*nurse as usize][*day as usize][*shift as usize] == 1 {
                    objective_expr += LinearExpr::from(shifts[&(nurse, day, shift)])
                        * shift_requests[*nurse as usize][*day as usize][*shift as usize];
                }
            }
        }
    }
    // let params = SatParameters {
    //     linearization_level: Some(0),
    //     enumerate_all_solutions: Some(true),
    //     ..Default::default()
    // };
    // let response = cp_model.solve_with_parameters(&params);
    cp_model.maximize(objective_expr);
    let response = cp_model.solve();

    if response.status() == CpSolverStatus::Optimal {
        log::info!("Solution:");
        for day in &all_days {
            log::info!("Day {}", day);
            for nurse in &all_nurses {
                for shift in &all_shifts {
                    if shifts[&(nurse, day, shift)].solution_value(&response) {
                        if shift_requests[*nurse as usize][*day as usize][*shift as usize] == 1 {
                            log::info!("  Nurse {} works shift {} (requested).", nurse, shift);
                        } else {
                            log::info!("  Nurse {} works shift {} (not requested).", nurse, shift);
                        }
                    }
                }
            }
            log::info!("");
        }
        log::info!("Number of shift requests met = {} (out of {})", response.objective_value, NUM_NURSES * min_shifts_per_nurse);
    } else {
        log::info!("No optimal solution found !");
    }
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    interval_sample_sat();
    nurse_scheduling();
}