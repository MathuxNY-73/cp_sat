use std::collections::HashMap;
use std::time::Instant;

use cp_sat::builder::{print_solver_stats, BoolVar, CpModelBuilder, IntVar, LinearExpr};
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
    log::info!("{}", print_solver_stats(&response, true));

    if response.status() == CpSolverStatus::Optimal {
        log::info!("Optimal Schedule Length: {}", response.objective_value);
        log::info!("Task 0 starts at {}", start_0.solution_value(&response));
        log::info!("Task 1 starts at {}", start_1.solution_value(&response));
        log::info!("Task 2 starts at {}", start_2.solution_value(&response));
    }
}

fn nurse_scheduling() {
    const NUM_NURSES: i64 = 4;
    const NUM_SHIFTS: i64 = 3;
    const NUM_DAYS: i64 = 3;

    let all_nurses: Vec<_> = (0..NUM_NURSES).collect();
    let all_shifts: Vec<_> = (0..NUM_SHIFTS).collect();
    let all_days: Vec<_> = (0..NUM_DAYS).collect();

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

    let params = SatParameters {
        linearization_level: Some(0),
        enumerate_all_solutions: Some(true),
        ..Default::default()
    };
    // let response = cp_model.solve_with_parameters(&params);
    let mut cnt = 0;
    cp_model.solve_with_observer(|r| {
        log::info!("Solution {}" , cnt);
        for d in &all_days {
            log::info!("Day {}", d);
            for n in &all_nurses {
                let mut is_working = false;
                for s in &all_shifts {
                    if shifts[&(n, d, s)].solution_value(&r) {
                        is_working = true;
                        log::info!("  Nurse {} works shift {}", n, s);
                    }
                }
                if !is_working {
                    log::info!("  Nurse {} does not work", n);
                }
            }
        }
        cnt += 1;
        if cnt >= 5 {
            log::info!("Stop search after {} solutions.", cnt);
            false
        } else {
            true
        }
    }, Some(&params));
}

fn nurse_scheduling_with_requests() {
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

fn negate_bounded_span(
    works: impl IntoIterator<Item = cp_sat::builder::BoolVar>, start: usize, length: usize
) -> Vec<cp_sat::builder::BoolVar> {
    works.into_iter().enumerate().map(|(pos, var)| {
        if pos >= start && pos < start + length {
            !var
        } else {
            var
        }
    }).collect()
}

fn add_soft_sequence_constraint(
        model: &mut cp_sat::builder::CpModelBuilder,
        works: &Vec<BoolVar>,
        hard_min: i64,
        soft_min: i64,
        min_cost: i64,
        soft_max: i64,
        hard_max: i64,
        max_cost: i64,
        prefix: &str) -> (Vec<BoolVar>, Vec<i64>) {
    
    // Forbid sequences that are too short.
    for length in 1..(hard_min as usize) {
        for start in 0..(works.len() - length + 1) {
            model.add_or(negate_bounded_span(works.clone(), start, length));
        }
    }

    let mut cost_literals = Vec::<BoolVar>::new();
    let mut cost_coefficients = Vec::<i64>::new();

    // Penalize sequences that below the soft limit.
    if min_cost > 0 {
        for length in (hard_min as usize)..(soft_min as usize) {
            for start in 0..(works.len() - length + 1) {
                let mut span = negate_bounded_span(works.clone(), start, length);
                let name = format!("{}: under_span(start={}, length={})", prefix, start, length);
                let lit = model.new_bool_var_with_name(name);
                span.push(lit.clone());
                model.add_or(span);
                cost_literals.push(lit);
                // We filter exactly the sequence with a short length.
                // The penalty is proportional to the delta with soft min.
                cost_coefficients.push(min_cost * (soft_min - (length as i64)));
            }
        }
    }

    if max_cost > 0 {
        for length in ((soft_max + 1) as usize)..((hard_max + 1) as usize) {
            for start in 0..(works.len() - length + 1) {
                let mut span = negate_bounded_span(works.clone(), start, length);
                let name = format!("{}: over_span(start={}, length={})", prefix, start, length);
                let lit = model.new_bool_var_with_name(name);
                span.push(lit.clone());
                model.add_or(span);
                cost_literals.push(lit);
                // Cost paid is max_cost * excess length.
                cost_coefficients.push(max_cost * ((length as i64) - soft_max));
            }
        }
    }

    for start in 0..(works.len() - (hard_max as usize)) {
        model.add_or((&works[start..(start + ((hard_max + 1) as usize))])
            .iter().cloned().map(|v| !v));
    }
    (cost_literals, cost_coefficients)
}

fn add_soft_sum_constraint(
    model: &mut CpModelBuilder,
    works: &Vec<BoolVar>,
    hard_min: i64,
    soft_min: i64,
    min_cost: i64,
    soft_max: i64,
    hard_max: i64,
    max_cost: i64,
    prefix: &str
) -> (Vec<IntVar>, Vec<i64>) {
    let mut cost_variables = Vec::<IntVar>::new();
    let mut cost_coefficients = Vec::<i64>::new();
    let sum_var = model.new_int_var([(hard_min, hard_max)]);

    // This adds the hard constraints on the sum.
    model.add_eq(sum_var.clone(), LinearExpr::from_iter(works.clone()));

    // Penalize sums below the soft_min target.
    if soft_min > hard_min && min_cost > 0 {
        let delta = model.new_int_var([(-(works.len() as i64), works.len() as i64)]);
        model.add_eq(delta, - LinearExpr::from(sum_var.clone()) + soft_min);
        // TODO(user): Compare efficiency with only excess >= soft_min - sum_var.
        let excess = model.new_int_var_with_name([(0, 7)], prefix.to_owned() + ": under_sum");
        model.add_max_eq(excess.clone(), [(0, delta)]);
        cost_variables.push(excess);
        cost_coefficients.push(min_cost);
    }

    // Penalize sums above the soft_max target.
    if soft_max < hard_max && max_cost > 0 {
        let delta = model.new_int_var([(-7, 7)]);
        model.add_eq(delta, LinearExpr::from(sum_var) - soft_max);
        let excess = model.new_int_var_with_name([(0, 7)], prefix.to_owned() + ": over_sum");
        model.add_max_eq(excess.clone(), [(0, delta)]);
        cost_variables.push(excess);
        cost_coefficients.push(max_cost)
    }

    (cost_variables, cost_coefficients)
}

/// Solves the shift scheduling problem.
fn solve_shift_scheduling() {
    // Data
    const NUM_EMPLOYEES: i64 = 8;
    const NUM_WEEKS: i64 = 3;
    const SHIFTS: [&str; 4] = ["O", "M", "A", "N"];

    // Fixed assignment: (employee, shift, day).
    // This fixes the first 2 days of the schedule.
    let fixed_assignments = [
        (0, 0, 0),
        (1, 0, 0),
        (2, 1, 0),
        (3, 1, 0),
        (4, 2, 0),
        (5, 2, 0),
        (6, 2, 3),
        (7, 3, 0),
        (0, 1, 1),
        (1, 1, 1),
        (2, 2, 1),
        (3, 2, 1),
        (4, 2, 1),
        (5, 0, 1),
        (6, 0, 1),
        (7, 3, 1),
    ];

    // Request: (employee, shift, day, weight)
    // A negative weight indicates that the employee desire this assignment.
    let requests = [
        // Employee 3 does not want to work on the first Saturday (negative weight
        // for the Off shift).
        (3i64, 0i64, 5i64, -2i64),
        // Employee 5 wants a night shift on the second Thursday (negative weight).
        (5, 3, 10, -2),
        // Employee 2 does not want a night shift on the first Friday (positive
        // weight).
        (2, 3, 4, 4),
    ];

    // Shift constraints on continuous sequence :
    //     (shift, hard_min, soft_min, min_penalty,
    //             soft_max, hard_max, max_penalty)
    let shift_constraints = [
        // One or two consecutive days of rest, this is a hard constraint.
        (0i64, 1i64, 1i64, 0i64, 2i64, 2i64, 0i64),
        // between 2 and 3 consecutive days of night shifts, 1 and 4 are
        // possible but penalized.
        (3, 1, 2, 20, 3, 4, 5),
    ];

    // Weekly sum constraints on shifts days:
    //     (shift, hard_min, soft_min, min_penalty,
    //             soft_max, hard_max, max_penalty)
    let weekly_sum_constraints = [
        // Constraints on rests per week.
        (0, 1, 2, 7, 2, 3, 4),
        // At least 1 night shift per week (penalized). At most 4 (hard).
        (3, 0, 1, 3, 4, 4, 0),
    ];

    // Penalized transitions:
    //     (previous_shift, next_shift, penalty (0 means forbidden))
    let penalized_transitions = [
        // Afternoon to night has a penalty of 4.
        (2i64, 3i64, 4i64),
        // Night to morning is forbidden.
        (3, 1, 0),
    ];

    // daily demands for work shifts (morning, afternon, night) for each day
    // of the week starting on Monday.
    let weekly_cover_demands = [
        [2i64, 3i64, 1i64],  // Monday
        [2, 3, 1],  // Tuesday
        [2, 2, 2],  // Wednesday
        [2, 3, 1],  // Thursday
        [2, 2, 2],  // Friday
        [1, 2, 3],  // Saturday
        [1, 3, 1],  // Sunday
    ];

    // Penalty for exceeding the cover constraint per shift type.
    let excess_cover_penalties = [2, 2, 5];

    let num_days = NUM_WEEKS * 7;
    let num_shifts = SHIFTS.len();

    let mut model = CpModelBuilder::default();

    let mut work = HashMap::<(i64, i64, i64), BoolVar>::new();
    for e in 0..NUM_EMPLOYEES {
        for s in 0..(num_shifts as i64) {
            for d in 0..num_days {
                work.insert((e, s, d),
                    model.new_bool_var_with_name(format!("work{}_{}_{}", e, s, d)));
            }
        }
    }

    // Linear terms of the objective in a minimization context.
    let mut obj_int_vars = Vec::<IntVar>::new();
    let mut obj_int_coeffs = Vec::<i64>::new();
    let mut obj_bool_vars = Vec::<BoolVar>::new();
    let mut obj_bool_coeffs = Vec::<i64>::new();

    // Exactly one shift per day.
    for e in 0..NUM_EMPLOYEES {
        for d in 0..num_days {
            model.add_exactly_one(
                (0..(num_shifts as i64)).into_iter().map(|s| work[&(e, s, d)]));
        }
    }

    // Fixed assignments.
    for key in &fixed_assignments {
        model.add_eq(work[key], 1);
    }

    // Employee requests
    for (e, s, d, w) in requests {
        obj_bool_vars.push(work[&(e, s, d)]);
        obj_bool_coeffs.push(w);
    }

    // Shift constraints
    for ct in shift_constraints {
        let (shift, hard_min, soft_min, min_cost, soft_max, hard_max, max_cost) = ct;
        for e in 0..NUM_EMPLOYEES {
            let works = (0..num_days).into_iter().map(|d| work[&(e, shift, d)]).collect();
            let (variables, coeffs) =
                add_soft_sequence_constraint(
                    &mut model,
                    &works,
                    hard_min,
                    soft_min,
                    min_cost,
                    soft_max,
                    hard_max,
                    max_cost,
            &format!("shift_constraint(employee {}, shift {})", e, shift),
                );
            obj_bool_vars.extend(variables);
            obj_bool_coeffs.extend(coeffs);
        }
    }

    // Weekly sum constraints
    for ct in weekly_sum_constraints {
        let (shift, hard_min, soft_min, min_cost, soft_max, hard_max, max_cost) = ct;
        for e in 0..NUM_EMPLOYEES {
            for w in 0..NUM_WEEKS {
                let works = (0..7).into_iter().map(|d| work[&(e, shift, d + w * 7)]).collect();
                let (variables, coeffs) = add_soft_sum_constraint(
                    &mut model,
                    &works,
                    hard_min,
                    soft_min,
                    min_cost,
                    soft_max,
                    hard_max,
                    max_cost,
                    &format!("weekly_sum_constraint(employee {}, shift {}, week {})", e, shift, w),
                );
                obj_int_vars.extend(variables);
                obj_int_coeffs.extend(coeffs);
            }
        }
    }

    // Penalized transitions
    for (previous_shift, next_shift, cost) in penalized_transitions {
        for e in 0..NUM_EMPLOYEES {
            for d in 0..(num_days - 1) {
                let mut transition = vec![
                    !work[&(e, previous_shift, d)], !work[&(e, next_shift, d + 1)]];
                if cost == 0 {
                    model.add_or(transition);
                } else {
                    let trans_var = model.new_bool_var_with_name(
                        format!("transition (employee={}, day={})", e, d)
                    );
                    transition.push(trans_var.clone());
                    model.add_or(transition);
                    obj_bool_vars.push(trans_var);
                    obj_bool_coeffs.push(cost);
                }
            }
        }
    }

    // Cover constraints
    for s in 1..(num_shifts as i64) {
        for w in 0..NUM_WEEKS {
            for d in 0..7i64 {
                let works = (0..NUM_EMPLOYEES).into_iter().map(|e| work[&(e, s, w * 7 + d)]).collect::<Vec<_>>();
                // Ignore Off shift.
                let min_demand = weekly_cover_demands[d as usize][(s - 1) as usize];
                let worked = model.new_int_var([(min_demand, NUM_EMPLOYEES)]);
                model.add_eq(worked, LinearExpr::from_iter(works));
                let over_penalty = excess_cover_penalties[(s - 1) as usize];
                if over_penalty > 0 {
                    let name = format!("excess_demand(shift={}, week={}, day={})", s, w, d);
                    let excess = model.new_int_var_with_name([(0, NUM_EMPLOYEES - min_demand)], name);
                    model.add_eq(excess, LinearExpr::from(worked) - min_demand);
                    obj_int_vars.push(excess);
                    obj_int_coeffs.push(over_penalty);
                }
            }
        }
    }

    // Objective
    model.minimize(
        LinearExpr::from_iter((0..obj_bool_vars.len()).into_iter().map(|i| LinearExpr::from(IntVar::from(obj_bool_vars[i])) * obj_bool_coeffs[i]))
        + LinearExpr::from_iter((0..obj_int_vars.len()).into_iter().map(|i| LinearExpr::from(obj_int_vars[i]) * obj_int_coeffs[i])));

    let mut cnt = 0;
    let start = Instant::now();
    // Solve the model.
    let response = model.solve_with_observer(|r| {
        log::info!("Solution {}, time={}, objective={}", cnt, start.elapsed().as_millis(), r.objective_value);
        cnt += 1;
        true
    }, None);

    let status = response.status();
    // Print solution.
    if status == CpSolverStatus::Optimal || status == CpSolverStatus::Feasible {
        log::info!("");
        let mut header = String::from("          ");
        for _w in 0..NUM_WEEKS {
            header += "M T W T F S S ";
        }
        log::info!("{}", header);
        for e in 0..NUM_EMPLOYEES {
            let mut schedule = String::from("");
            for d in 0..num_days {
                for s in 0..(num_shifts as i64) {
                    if work[&(e, s, d)].solution_value(&response) {
                        schedule += SHIFTS[s as usize];
                        schedule += " ";
                    }
                }
            }
            log::info!("worker {}: {}", e, schedule);
        }
        log::info!("");
        log::info!("Penalties:");
        for (i, var) in obj_bool_vars.iter().enumerate() {
            if var.solution_value(&response) {
                let penalty = obj_bool_coeffs[i];
                let name = model.var_name(var.clone());
                if penalty > 0 {
                    log::info!("  {} violated, penalty={}", name, penalty);
                } else {
                    log::info!("  {} fulfilled, gain={}", name, -penalty);
                }
            }
        }

        for (i, var) in obj_int_vars.iter().enumerate() {
            if var.solution_value(&response) > 0 {
                let name = model.var_name(var.clone());
                println!(
                    "  {} violated by {}, linear penalty={}",
                    name, var.solution_value(&response), obj_int_coeffs[i]);
            }
        }
    }

    println!("");
    println!("Statistics");
    println!("  - status          : {}", response.status().as_str_name());
    println!("  - conflicts       : {}", response.num_conflicts);
    println!("  - branches        : {}", response.num_branches);
    println!("  - wall time       : {} s", response.wall_time);
}


fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    interval_sample_sat();
    nurse_scheduling();
    solve_shift_scheduling();
    nurse_scheduling_with_requests();
}