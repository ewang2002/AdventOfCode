use std::collections::HashMap;

use common::{
    constants::TWO_NEWLINE,
    problem::day::{AoCProblem, Solution},
};

pub type Part = (usize, usize, usize, usize);

pub struct Day19 {
    workflows: HashMap<String, Vec<WorkflowRule>>,
    part_ratings: Vec<Part>,
}

impl AoCProblem for Day19 {
    fn prepare(input: String) -> Self {
        let mut groups = input.split(TWO_NEWLINE);
        let raw_workflows = groups.next().unwrap();
        let raw_part_ratings = groups.next().unwrap();

        Self {
            workflows: raw_workflows
                .lines()
                .map(|l| {
                    let idx_left_brace = l.find('{').unwrap();
                    let idx_right_brace = l.find('}').unwrap();
                    let workflow_name = l[..idx_left_brace].to_string();

                    let mut rules = vec![];
                    for raw_rule in l[idx_left_brace + 1..idx_right_brace].split(',') {
                        if raw_rule.contains(':') {
                            let mut rule_split = raw_rule.split(':');
                            let conditional = rule_split.next().unwrap();
                            let raw_if_true = rule_split.next().unwrap();

                            let part = match &conditional[..1] {
                                "x" => PartCategory::X,
                                "m" => PartCategory::M,
                                "a" => PartCategory::A,
                                "s" => PartCategory::S,
                                c => panic!("Invalid part category '{c}'"),
                            };

                            let op = match &conditional[1..2] {
                                "<" => RuleOp::Lt,
                                ">" => RuleOp::Gt,
                                c => panic!("Invalid rule op '{c}'"),
                            };

                            let value = conditional[2..].parse().unwrap();
                            let if_true = RuleNextStep::from(raw_if_true);

                            rules.push(WorkflowRule::If {
                                part,
                                op,
                                value,
                                if_true,
                            });
                        } else {
                            rules.push(WorkflowRule::End {
                                next_step: RuleNextStep::from(raw_rule),
                            });
                        }
                    }

                    (workflow_name, rules)
                })
                .collect(),

            part_ratings: raw_part_ratings
                .lines()
                .map(|l| {
                    let mut inner = l[1..l.len() - 1].split(',').map(|s| &s[2..]);
                    let x = inner.next().unwrap().parse().unwrap();
                    let m = inner.next().unwrap().parse().unwrap();
                    let a = inner.next().unwrap().parse().unwrap();
                    let s = inner.next().unwrap().parse().unwrap();
                    (x, m, a, s)
                })
                .collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        self.part_ratings
            .iter()
            .filter(|part| self.validate_part(part))
            .map(|part| part.0 + part.1 + part.2 + part.3)
            .sum::<usize>()
            .into()
    }

    fn part2(&mut self) -> Solution {
        0.into()
    }

    fn day() -> u32 {
        19
    }

    fn year() -> u32 {
        2023
    }
}

impl Day19 {
    /// Validates a part against the workflows.
    ///
    /// # Parameters
    /// - `part`: The part to validate.
    ///
    /// # Returns
    /// Whether the part is valid.
    pub fn validate_part(&self, part: &Part) -> bool {
        let mut workflow = &self.workflows["in"];
        loop {
            for rule in workflow {
                match rule.execute_rule(*part) {
                    RuleNextStep::Accept => return true,
                    RuleNextStep::Reject => return false,
                    RuleNextStep::Continue => continue,
                    RuleNextStep::Workflow(workflow_name) => {
                        workflow = &self.workflows[workflow_name];
                        break;
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum WorkflowRule {
    /// Whether the rule has an associated conditional.
    If {
        part: PartCategory,
        op: RuleOp,
        value: usize,
        if_true: RuleNextStep,
    },
    /// Whether the rule is the last rule in the workflow.
    End { next_step: RuleNextStep },
}

impl WorkflowRule {
    /// Executes the rule on a part.
    ///
    /// # Parameters
    /// - `part`: The part to execute the rule on.
    ///
    /// # Returns
    /// The result of the rule execution.
    pub fn execute_rule(&self, part: Part) -> &RuleNextStep {
        match self {
            WorkflowRule::If {
                part: part_category,
                op,
                value,
                if_true,
            } => {
                let part_value = match part_category {
                    PartCategory::X => part.0,
                    PartCategory::M => part.1,
                    PartCategory::A => part.2,
                    PartCategory::S => part.3,
                };

                match op {
                    RuleOp::Lt => {
                        if part_value < *value {
                            if_true
                        } else {
                            &RuleNextStep::Continue
                        }
                    }
                    RuleOp::Gt => {
                        if part_value > *value {
                            if_true
                        } else {
                            &RuleNextStep::Continue
                        }
                    }
                }
            }
            WorkflowRule::End { next_step } => next_step,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PartCategory {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RuleOp {
    Lt,
    Gt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RuleNextStep {
    /// The name of the next workflow to execute.
    Workflow(String),
    /// Whether to immediately accept the part.
    Accept,
    /// Whether to immediately reject the part.
    Reject,
    /// Whether to continue to the next rule in the current workflow.
    Continue,
}

impl From<&str> for RuleNextStep {
    fn from(s: &str) -> Self {
        match s {
            "A" => RuleNextStep::Accept,
            "R" => RuleNextStep::Reject,
            _ => Self::Workflow(s.to_string()),
        }
    }
}
