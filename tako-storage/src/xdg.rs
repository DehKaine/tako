use std::{env, path::PathBuf}

pub struct Xdg;

impl Xdg  {
    pub fn config_dir() -> PathBuf {

        #[cfg(windows)]

        #[cfg(unix)] 
        {

        }

    }

    pub fn state_dir() -> PathBuf {
        #[cfg(windows)]
         {
            dirs::data_dir().map(|p| p.join("tako").join("state")).expect("Failed to get state directory")
         }
        #[cfg(unix)] 
         {

         }
    }

    #[inline]
    pub fn cache_dir -> PathBuf  {
        #[cfg(unix)]
        #[cfg(windows)]
    }
}
