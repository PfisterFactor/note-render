pub mod neovim_handler  {
    use neovim_lib::*;
    use std::env;

    pub struct NeovimHandler {
        nvim: Neovim
    }

    impl NeovimHandler {
        pub fn try_new() -> Result<NeovimHandler,String> {
            let mut session = Session::new_parent();
            let nvim = Neovim::new(session.unwrap());
            Ok(NeovimHandler { nvim })
        }

        // Handle events
        fn recv() {
            // TODO
        }
    }
}