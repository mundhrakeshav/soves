use super::simulator_task::SimulateTask;
use std::collections::VecDeque;

pub struct WorkerNode {
    pub id: u32,
    task_queue: VecDeque<SimulateTask>,
}
