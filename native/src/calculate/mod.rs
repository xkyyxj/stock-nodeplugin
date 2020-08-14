mod calculate_low;

pub use calculate_low::calculate_in_low;
use neon::prelude::{Task, Context};
use neon::result::JsResult;
use neon::context::TaskContext;
use futures::executor;
use neon::types::JsBoolean;

pub struct CalculateLowAsyncTask;

impl Task for CalculateLowAsyncTask {
    type Output = bool;
    type Error = String;
    type JsEvent = JsBoolean;

    fn perform(&self) -> Result<Self::Output, Self::Error> {
        Ok(executor::block_on(calculate_in_low()))
    }

    fn complete(self, mut cx: TaskContext, result: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> {
        Ok(cx.boolean(result.unwrap()))
    }
}