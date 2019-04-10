use crate::prelude::*;

/// # Application
impl<'a> Discord<'a> {
    // tested, returns "en-US" and similar
    pub fn current_locale(&mut self) -> String {
        let mut locale: sys::DiscordLocale = [0; size_of::<sys::DiscordLocale>()];

        unsafe {
            ffi!(self
                .get_application_manager()
                .get_current_locale(&mut locale as *mut _))
        }

        unsafe { string_from_cstr(&locale as *const _) }
    }

    // tested, returns "master" or whichever `dispatch` branch is in use
    pub fn current_branch(&mut self) -> String {
        let mut branch: sys::DiscordBranch = [0; size_of::<sys::DiscordBranch>()];

        unsafe {
            ffi!(self
                .get_application_manager()
                .get_current_branch(&mut branch as *mut _))
        }

        unsafe { string_from_cstr(&branch as *const _) }
    }

    // tested, hasn't failed yet
    pub fn validate_or_exit<F>(&mut self, callback: F)
    where
        F: FnMut(&mut Discord, Result<()>),
    {
        unsafe {
            ffi!(self
                .get_application_manager()
                .validate_or_exit(self.wrap_callback(callback), Some(callbacks::result::<F>)))
        }
    }

    // tested
    pub fn oauth2_token<F>(&mut self, callback: F)
    where
        F: FnMut(&mut Discord, Result<OAuth2Token>),
    {
        unsafe {
            ffi!(self.get_application_manager().get_oauth2_token(
                self.wrap_callback(callback),
                Some(callbacks::result_from_sys_ptr::<F, OAuth2Token>)
            ))
        }
    }

    // tested
    pub fn app_ticket<F>(&mut self, callback: F)
    where
        F: FnMut(&mut Discord, Result<String>),
    {
        unsafe {
            ffi!(self.get_application_manager().get_ticket(
                self.wrap_callback(callback),
                Some(callbacks::result_str::<F>)
            ))
        }
    }
}
