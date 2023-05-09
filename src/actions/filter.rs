use crate::cli::FilterArgs;

use super::Action;

/// An [Action] responsible for filtering the input
pub struct FilterAction {}

impl Action<FilterArgs, ()> for FilterAction {
    fn execute<'a, 'b>(
        &mut self,
        context: &mut super::ActionContext<'a, FilterArgs>,
    ) -> crate::errors::ChiselResult<()> {
        todo!()
    }
}
