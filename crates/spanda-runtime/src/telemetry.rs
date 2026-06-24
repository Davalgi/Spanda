//! Runtime telemetry for deterministic scheduler and task execution.

use serde::{Deserialize, Serialize};
use spanda_ast::foundations::TaskPriority;
use std::collections::HashMap;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct TaskMetrics {
    pub name: String,
    pub priority: String,
    pub interval_ms: f64,
    pub ticks: u64,
    pub skipped: u64,
    pub missed_deadlines: u64,
    pub budget_violations: u64,
    pub last_duration_ms: f64,
    pub max_duration_ms: f64,
    pub max_jitter_ms: f64,
    pub jitter_violations: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct SchedulerMetrics {
    pub multiplexed_tasks: u64,
    pub scheduler_ticks: u64,
    pub base_tick_ms: f64,
    pub emergency_stops: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ExecutionMetrics {
    pub spawns: u64,
    pub joins: u64,
    pub parallel_blocks: u64,
    pub fire_and_forget_spawns: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct TriggerMetrics {
    pub name: String,
    pub category: String,
    pub priority: String,
    pub executions: u64,
    pub failures: u64,
    pub missed_deadlines: u64,
    pub last_duration_ms: f64,
    pub max_duration_ms: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct PipelineMetrics {
    pub name: String,
    pub budget_ms: f64,
    pub executions: u64,
    pub total_duration_ms: f64,
    pub deadline_misses: u64,
    pub slow_stages: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct WatchdogMetrics {
    pub name: String,
    pub timeouts: u64,
    pub last_timeout_ms: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct TopicMetrics {
    pub path: String,
    pub deadline_misses: u64,
    pub last_elapsed_ms: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ProviderMetrics {
    pub provider_key: String,
    pub category: String,
    pub calls: u64,
    pub failures: u64,
    pub last_duration_ms: f64,
    pub max_duration_ms: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct RuntimeTelemetry {
    pub tasks: HashMap<String, TaskMetrics>,
    pub triggers: HashMap<String, TriggerMetrics>,
    pub pipelines: HashMap<String, PipelineMetrics>,
    pub watchdogs: HashMap<String, WatchdogMetrics>,
    pub topics: HashMap<String, TopicMetrics>,
    pub providers: HashMap<String, ProviderMetrics>,
    pub scheduler: SchedulerMetrics,
    pub execution: ExecutionMetrics,
    pub replay_frames: u64,
}

impl RuntimeTelemetry {
    pub fn task_mut(
        &mut self,
        name: &str,
        priority: TaskPriority,
        interval_ms: f64,
    ) -> &mut TaskMetrics {
        // Description:
        //     Task mut.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     name: &str
        //         Caller-supplied name.
        //     priority: TaskPriority
        //         Caller-supplied priority.
        //     interval_ms: f64
        //         Caller-supplied interval ms.
        //
        // Outputs:
        //     result: &mut TaskMetrics
        //         Return value from `task_mut`.
        //
        // Example:
        //     let result = spanda_runtime::telemetry::task_mut(&mut self, name, priority, interval_ms);

        // Call tasks on the current instance.
        self.tasks
            .entry(name.to_string())
            .or_insert_with(|| TaskMetrics {
                name: name.to_string(),
                priority: priority_label(priority),
                interval_ms,
                ..Default::default()
            })
    }

    pub fn record_task_tick(&mut self, name: &str, priority: TaskPriority, interval_ms: f64) {
        // Description:
        //     Record task tick.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     name: &str
        //         Caller-supplied name.
        //     priority: TaskPriority
        //         Caller-supplied priority.
        //     interval_ms: f64
        //         Caller-supplied interval ms.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_runtime::telemetry::record_task_tick(&mut self, name, priority, interval_ms);

        // Call task mut on the current instance.
        self.task_mut(name, priority, interval_ms).ticks += 1;
    }

    pub fn record_task_duration(
        &mut self,
        name: &str,
        priority: TaskPriority,
        interval_ms: f64,
        duration_ms: f64,
    ) {
        // Description:
        //     Record task duration.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     name: &str
        //         Caller-supplied name.
        //     priority: TaskPriority
        //         Caller-supplied priority.
        //     interval_ms: f64
        //         Caller-supplied interval ms.
        //     duration_ms: f64
        //         Caller-supplied duration ms.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_runtime::telemetry::record_task_duration(&mut self, name, priority, interval_ms, duration_ms);

        // Compute entry for the following logic.
        let entry = self.task_mut(name, priority, interval_ms);
        entry.last_duration_ms = duration_ms;

        // Take this path when duration ms > entry.max duration ms.
        if duration_ms > entry.max_duration_ms {
            entry.max_duration_ms = duration_ms;
        }
    }

    pub fn record_budget_violation(
        &mut self,
        name: &str,
        priority: TaskPriority,
        interval_ms: f64,
    ) {
        // Description:
        //     Record budget violation.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     name: &str
        //         Caller-supplied name.
        //     priority: TaskPriority
        //         Caller-supplied priority.
        //     interval_ms: f64
        //         Caller-supplied interval ms.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_runtime::telemetry::record_budget_violation(&mut self, name, priority, interval_ms);

        // Call task mut on the current instance.
        self.task_mut(name, priority, interval_ms).budget_violations += 1;
    }

    pub fn record_task_jitter(
        &mut self,
        name: &str,
        priority: TaskPriority,
        interval_ms: f64,
        jitter_ms: f64,
        max_jitter_ms: f64,
    ) {
        // Description:
        //     Record task jitter.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     name: &str
        //         Caller-supplied name.
        //     priority: TaskPriority
        //         Caller-supplied priority.
        //     interval_ms: f64
        //         Caller-supplied interval ms.
        //     jitter_ms: f64
        //         Caller-supplied jitter ms.
        //     ax_jitter_ms: f64
        //         Caller-supplied ax jitter ms.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_runtime::telemetry::record_task_jitter(&mut self, name, priority, interval_ms, jitter_ms, ax_jitter_ms);

        // Update jitter peaks and violation counts.
        let entry = self.task_mut(name, priority, interval_ms);
        if jitter_ms > entry.max_jitter_ms {
            entry.max_jitter_ms = jitter_ms;
        }
        if jitter_ms > max_jitter_ms {
            entry.jitter_violations += 1;
        }
    }

    pub fn record_task_skip(&mut self, name: &str, priority: TaskPriority, interval_ms: f64) {
        // Description:
        //     Record task skip.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     name: &str
        //         Caller-supplied name.
        //     priority: TaskPriority
        //         Caller-supplied priority.
        //     interval_ms: f64
        //         Caller-supplied interval ms.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_runtime::telemetry::record_task_skip(&mut self, name, priority, interval_ms);

        // Call task mut on the current instance.
        self.task_mut(name, priority, interval_ms).skipped += 1;
    }

    pub fn record_missed_deadline(&mut self, name: &str, priority: TaskPriority, interval_ms: f64) {
        // Description:
        //     Record missed deadline.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     name: &str
        //         Caller-supplied name.
        //     priority: TaskPriority
        //         Caller-supplied priority.
        //     interval_ms: f64
        //         Caller-supplied interval ms.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_runtime::telemetry::record_missed_deadline(&mut self, name, priority, interval_ms);

        // Call task mut on the current instance.
        self.task_mut(name, priority, interval_ms).missed_deadlines += 1;
    }

    pub fn record_scheduler_start(&mut self, task_count: u64, base_tick_ms: f64) {
        // Description:
        //     Record scheduler start.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     ask_coun: u64
        //         Caller-supplied ask coun.
        //     base_tick_ms: f64
        //         Caller-supplied base tick ms.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_runtime::telemetry::record_scheduler_start(&mut self, ask_coun, base_tick_ms);

        // Call multiplexed tasks = task count; on the current instance.
        self.scheduler.multiplexed_tasks = task_count;
        self.scheduler.base_tick_ms = base_tick_ms;
    }

    pub fn record_scheduler_tick(&mut self) {
        // Description:
        //     Record scheduler tick.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_runtime::telemetry::record_scheduler_tick(&mut self);

        // Call scheduler ticks += 1; on the current instance.
        self.scheduler.scheduler_ticks += 1;
    }

    pub fn record_emergency_stop(&mut self) {
        // Description:
        //     Record emergency stop.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_runtime::telemetry::record_emergency_stop(&mut self);

        // Call emergency stops += 1; on the current instance.
        self.scheduler.emergency_stops += 1;
    }

    pub fn record_spawn(&mut self) {
        // Description:
        //     Record spawn.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_runtime::telemetry::record_spawn(&mut self);

        // Call spawns += 1; on the current instance.
        self.execution.spawns += 1;
    }

    pub fn record_fire_and_forget_spawn(&mut self) {
        // Description:
        //     Record fire and forget spawn.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_runtime::telemetry::record_fire_and_forget_spawn(&mut self);

        // Call fire and forget spawns += 1; on the current instance.
        self.execution.fire_and_forget_spawns += 1;
    }

    pub fn record_join(&mut self) {
        // Description:
        //     Record join.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_runtime::telemetry::record_join(&mut self);

        // Call joins += 1; on the current instance.
        self.execution.joins += 1;
    }

    pub fn record_parallel_block(&mut self) {
        // Description:
        //     Record parallel block.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_runtime::telemetry::record_parallel_block(&mut self);

        // Call parallel blocks += 1; on the current instance.
        self.execution.parallel_blocks += 1;
    }

    pub fn record_replay_frames(&mut self, count: u64) {
        // Description:
        //     Record replay frames.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     coun: u64
        //         Caller-supplied coun.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_runtime::telemetry::record_replay_frames(&mut self, coun);

        // Call replay frames = count; on the current instance.
        self.replay_frames = count;
    }

    pub fn trigger_mut(
        &mut self,
        name: &str,
        category: &str,
        priority: TaskPriority,
    ) -> &mut TriggerMetrics {
        // Description:
        //     Trigger mut.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     name: &str
        //         Caller-supplied name.
        //     category: &str
        //         Caller-supplied category.
        //     priority: TaskPriority
        //         Caller-supplied priority.
        //
        // Outputs:
        //     result: &mut TriggerMetrics
        //         Return value from `trigger_mut`.
        //
        // Example:
        //     let result = spanda_runtime::telemetry::trigger_mut(&mut self, name, category, priority);

        // Call triggers on the current instance.
        self.triggers
            .entry(name.to_string())
            .or_insert_with(|| TriggerMetrics {
                name: name.to_string(),
                category: category.to_string(),
                priority: priority_label(priority),
                ..Default::default()
            })
    }

    pub fn record_trigger_execution(
        &mut self,
        name: &str,
        category: &str,
        priority: TaskPriority,
        duration_ms: f64,
        failed: bool,
    ) {
        // Description:
        //     Record trigger execution.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     name: &str
        //         Caller-supplied name.
        //     category: &str
        //         Caller-supplied category.
        //     priority: TaskPriority
        //         Caller-supplied priority.
        //     duration_ms: f64
        //         Caller-supplied duration ms.
        //     failed: bool
        //         Caller-supplied failed.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_runtime::telemetry::record_trigger_execution(&mut self, name, category, priority, duration_ms, failed);

        // Compute entry for the following logic.
        let entry = self.trigger_mut(name, category, priority);
        entry.executions += 1;

        // Take this path when failed.
        if failed {
            entry.failures += 1;
        }
        entry.last_duration_ms = duration_ms;

        // Take this path when duration ms > entry.max duration ms.
        if duration_ms > entry.max_duration_ms {
            entry.max_duration_ms = duration_ms;
        }
    }

    pub fn record_trigger_missed_deadline(
        &mut self,
        name: &str,
        category: &str,
        priority: TaskPriority,
    ) {
        // Description:
        //     Record trigger missed deadline.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     name: &str
        //         Caller-supplied name.
        //     category: &str
        //         Caller-supplied category.
        //     priority: TaskPriority
        //         Caller-supplied priority.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_runtime::telemetry::record_trigger_missed_deadline(&mut self, name, category, priority);

        // Call trigger mut on the current instance.
        self.trigger_mut(name, category, priority).missed_deadlines += 1;
    }

    pub fn pipeline_mut(&mut self, name: &str, budget_ms: f64) -> &mut PipelineMetrics {
        // Description:
        //     Pipeline mut.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     name: &str
        //         Caller-supplied name.
        //     budget_ms: f64
        //         Caller-supplied budget ms.
        //
        // Outputs:
        //     result: &mut PipelineMetrics
        //         Return value from `pipeline_mut`.
        //
        // Example:

        //     let result = spanda_runtime::telemetry::pipeline_mut(&mut self, name, budget_ms);

        self.pipelines
            .entry(name.to_string())
            .or_insert_with(|| PipelineMetrics {
                name: name.to_string(),
                budget_ms,
                ..Default::default()
            })
    }

    pub fn record_pipeline_execution(
        &mut self,
        name: &str,
        budget_ms: f64,
        duration_ms: f64,
        slow_stage: bool,
    ) {
        // Description:
        //     Record pipeline execution.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     name: &str
        //         Caller-supplied name.
        //     budget_ms: f64
        //         Caller-supplied budget ms.
        //     duration_ms: f64
        //         Caller-supplied duration ms.
        //     slow_stage: bool
        //         Caller-supplied slow stage.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_runtime::telemetry::record_pipeline_execution(&mut self, name, budget_ms, duration_ms, slow_stage);

        let metrics = self.pipeline_mut(name, budget_ms);
        metrics.executions += 1;
        metrics.total_duration_ms += duration_ms;
        if duration_ms > budget_ms {
            metrics.deadline_misses += 1;
        }
        if slow_stage {
            metrics.slow_stages += 1;
        }
    }

    pub fn watchdog_mut(&mut self, name: &str) -> &mut WatchdogMetrics {
        // Description:
        //     Watchdog mut.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     name: &str
        //         Caller-supplied name.
        //
        // Outputs:
        //     result: &mut WatchdogMetrics
        //         Return value from `watchdog_mut`.
        //
        // Example:

        //     let result = spanda_runtime::telemetry::watchdog_mut(&mut self, name);

        self.watchdogs
            .entry(name.to_string())
            .or_insert_with(|| WatchdogMetrics {
                name: name.to_string(),
                ..Default::default()
            })
    }

    pub fn record_watchdog_timeout(&mut self, name: &str, sim_time_ms: f64) {
        // Description:
        //     Record watchdog timeout.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     name: &str
        //         Caller-supplied name.
        //     sim_time_ms: f64
        //         Caller-supplied sim time ms.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_runtime::telemetry::record_watchdog_timeout(&mut self, name, sim_time_ms);

        let metrics = self.watchdog_mut(name);
        metrics.timeouts += 1;
        metrics.last_timeout_ms = sim_time_ms;
    }

    pub fn record_topic_deadline_miss(&mut self, path: &str, elapsed_ms: f64, deadline_ms: f64) {
        // Description:
        //     Record topic deadline miss.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     path: &str
        //         Caller-supplied path.
        //     elapsed_ms: f64
        //         Caller-supplied elapsed ms.
        //     deadline_ms: f64
        //         Caller-supplied deadline ms.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_runtime::telemetry::record_topic_deadline_miss(&mut self, path, elapsed_ms, deadline_ms);

        // Update per-topic miss counters and last observed lateness.
        let entry = self
            .topics
            .entry(path.to_string())
            .or_insert_with(|| TopicMetrics {
                path: path.to_string(),
                ..Default::default()
            });
        entry.deadline_misses += 1;
        entry.last_elapsed_ms = elapsed_ms;
        let _ = deadline_ms;
    }

    pub fn record_provider_call(
        &mut self,
        provider_key: &str,
        category: &str,
        duration_ms: f64,
        failed: bool,
    ) {
        // Description:
        //     Record provider call.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     provider_key: &str
        //         Caller-supplied provider key.
        //     category: &str
        //         Caller-supplied category.
        //     duration_ms: f64
        //         Caller-supplied duration ms.
        //     failed: bool
        //         Caller-supplied failed.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_runtime::telemetry::record_provider_call(&mut self, provider_key, category, duration_ms, failed);

        // Accumulate per-provider call counts and latency peaks.
        let entry = self
            .providers
            .entry(provider_key.to_string())
            .or_insert_with(|| ProviderMetrics {
                provider_key: provider_key.to_string(),
                category: category.to_string(),
                ..Default::default()
            });
        entry.calls += 1;
        if failed {
            entry.failures += 1;
        }
        entry.last_duration_ms = duration_ms;
        if duration_ms > entry.max_duration_ms {
            entry.max_duration_ms = duration_ms;
        }
    }
}

fn priority_label(priority: TaskPriority) -> String {
    // Description:
    //     Priority label.
    //
    // Inputs:
    //     priority: TaskPriority
    //         Caller-supplied priority.
    //
    // Outputs:
    //     result: String
    //         Return value from `priority_label`.
    //
    // Example:
    //     let result = spanda_runtime::telemetry::priority_label(priority);

    // Match on priority and handle each case.
    match priority {
        TaskPriority::Critical => "critical".into(),
        TaskPriority::High => "high".into(),
        TaskPriority::Normal => "normal".into(),
        TaskPriority::Low => "low".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aggregates_task_and_scheduler_metrics() {
        // Description:
        //     Aggregates task and scheduler metrics.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_runtime::telemetry::aggregates_task_and_scheduler_metrics();

        let mut telemetry = RuntimeTelemetry::default();
        telemetry.record_scheduler_start(2, 50.0);
        telemetry.record_scheduler_tick();
        telemetry.record_task_tick("sense", TaskPriority::High, 50.0);
        telemetry.record_missed_deadline("sense", TaskPriority::High, 50.0);
        telemetry.record_spawn();
        telemetry.record_join();

        assert_eq!(telemetry.scheduler.scheduler_ticks, 1);
        assert_eq!(telemetry.tasks["sense"].ticks, 1);
        assert_eq!(telemetry.tasks["sense"].missed_deadlines, 1);
        assert_eq!(telemetry.execution.spawns, 1);
        assert_eq!(telemetry.execution.joins, 1);
    }
}
