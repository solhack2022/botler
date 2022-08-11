use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug)]
pub struct ConditionalJob {
    status: bool,
    
}

#[account]
#[derive(Default, Debug)]
pub struct TimebasedJob {

}


pub trait CommonJob {
    fn new(
        &mut self,
    ) -> Result<()>;

    fn execute(
        &mut self,
    ) -> Result<()>;

    fn cancel(
        &mut self,
    ) -> Result<()>;
}

impl CommonJob for ConditionalJob {
    fn new() -> Self {
        Self {
            
        }
    }
}

impl CommonJob for TimebasedJob {
    fn new() -> Self {
        Self {
            
        }
    }
}