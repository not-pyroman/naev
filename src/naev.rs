use formatx::formatx;
use sdl2 as sdl;
use std::ffi::{CStr, CString};
use std::io::{Error, ErrorKind, Result};
use std::os::raw::{c_char, c_int};

#[link(name = "naev")]
extern "C" {
    /// Main function in C
    pub fn naev_main() -> c_int;
}

mod array;
mod env;
mod gettext;
mod linebreak;
mod log;
mod ndata;
mod ntime;
mod slots;
mod version;

use crate::gettext::gettext;

static APPNAME: &str = "Naev";

use std::sync::atomic::AtomicBool;
static _QUIT: AtomicBool = AtomicBool::new(false);

pub fn naev() -> Result<()> {
    /* Load up the argv and argc for the C main. */
    let args: Vec<String> = std::env::args().collect();
    let mut cargs = vec![];
    for a in args {
        cargs.push(CString::new(a).unwrap())
    }
    let mut argv = cargs.into_iter().map(|s| s.into_raw()).collect::<Vec<_>>();
    argv.shrink_to_fit();

    /* Begin logging infrastructure. */
    log::init();

    /* Start up PHYSFS. */
    unsafe {
        let argv0 = CString::new(env::ENV.argv0.clone()).unwrap();
        if naevc::PHYSFS_init(argv0.as_ptr() as *const c_char) == 0 {
            let err = ndata::physfs_error_as_io_error();
            println!("{}", err);
            return Err(Error::new(ErrorKind::Other, err));
            /* TODO probably move the error handling to the "real" main, when shit hits the
                * fan. Below depends on sdl3
            SDL_ShowSimpleMessageBox( SDL_MESSAGEBOX_ERROR,
                _( "Naev Critical Error" ), buf,
                gl_screen.window );
            */
        }
        naevc::PHYSFS_permitSymbolicLinks(1);
    }

    /* Set up locales. */
    linebreak::init();
    gettext::init();

    /* Print the version */
    log::log(&version::VERSION_HUMAN);
    if cfg!(target_os = "linux") {
        match env::ENV.is_appimage {
            true => {
                log::log(format!("AppImage detected. Running from: {}", env::ENV.appdir).as_str())
            }
            false => log::debug("AppImage not detected."),
        }
    }

    /* Initialize SDL. */
    let sdlctx = match sdl::init() {
        Ok(s) => s,
        Err(e) => panic!("Unable to initialize SDL: {}", e),
    };

    let sdltime = match sdlctx.timer() {
        Ok(s) => s,
        Err(e) => panic!("Unable to initialize SDL Timer: {}", e),
    };
    let _starttime = sdltime.ticks();

    unsafe {
        naevc::threadpool_init();
        naevc::debug_sigInit();
    }

    if cfg!(unix) {
        /* Set window class and name. */
        std::env::set_var("SDL_VIDEO_X11_WMCLASS", APPNAME);
    }

    let _sdlvid = match sdlctx.video() {
        Ok(s) => s,
        Err(e) => panic!("Unable to initialize SDL Video: {}", e),
    };

    unsafe {
        naevc::nxml_init(); /* We'll be parsing XML. */
        naevc::input_init(); /* input has to be initialized for config to work. */
        naevc::lua_init(); /* initializes lua. */
        naevc::conf_setDefaults(); /* set the default config values. */

        /*
         * Attempts to load the data path from datapath.lua
         * At this early point in the load process, the binary path
         * is the only place likely to be checked.
         */
        naevc::conf_loadConfigPath();
    }

    /* Create the home directory if needed. */
    let cpath = unsafe { naevc::nfile_configPath() };
    unsafe {
        if naevc::nfile_dirMakeExist(cpath) != 0 {
            log::warn(&formatx!(gettext("Unable to create config directory '{}'"), "foo").unwrap());
        }
    }

    /* Set up the configuration. */
    let conf_file_path = unsafe {
        let rpath = CStr::from_ptr(cpath).to_str().unwrap();
        let conf_file = CStr::from_ptr(naevc::CONF_FILE.as_ptr() as *const c_char)
            .to_str()
            .unwrap();
        format!("{}{}", rpath, conf_file)
    };

    unsafe {
        let cconf_file_path = CString::new(conf_file_path).unwrap();
        naevc::conf_loadConfig(cconf_file_path.as_ptr()); /* Lua to parse the configuration file */
        naevc::conf_parseCLI(argv.len() as c_int, argv.as_mut_ptr()); /* parse CLI arguments */
    }

    unsafe {
        naev_main();
    };
    Ok(())
}
