use anchor_lang::{prelude::*, AnchorDeserialize};

#[account]
#[derive(Debug)]
pub struct Config {
    pub admin: Pubkey,
    pub whitelist: Vec<Pubkey>,
    /* 
        TODO: Configuration Struct for account
    */
}

/* 
    TODO: create Account which contains Config struct
*/

pub trait ConfigAccount {
    fn new(&mut self, admin: Pubkey, whitelist: Vec<Pubkey>) -> Result<()>;

    fn is_whitelisted(&self, key: &Pubkey) -> Result<bool>;

    fn add_whitelist(&mut self, key: &Pubkey) -> Result<()>;
}

impl ConfigAccount for Account<'_, Config> {
    fn new(
            &mut self,
            admin: Pubkey,
            whitelist: Vec<Pubkey>,
        ) -> Result<()> {
        self.admin = admin.key();
        self.whitelist = whitelist;
        Ok(())
    }
    
    fn is_whitelisted(&self, key: &Pubkey) -> Result<bool> {
        Ok(self.whitelist.contains(key))
    }

    // ADD reuqire to check if caller of this method is admin.
    fn add_whitelist(&mut self, key: &Pubkey) -> Result<()> {
        self.whitelist.push(*key);
        Ok(())
    }
}