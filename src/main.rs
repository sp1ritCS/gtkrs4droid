mod window;
use window::Window;

use gtk::{
	prelude::*,
	Application,
	gio
};

use std::ffi::{c_char};

#[unsafe(no_mangle)]
pub extern "C" fn main(argc: i32, argv: *mut *mut c_char) -> i32 {
	let res_bytes = include_bytes!(env!("GRESOURCES_PATH"));
	let data = gtk::glib::Bytes::from(&res_bytes[..]);
	let resource = gio::Resource::from_data(&data).unwrap();
	gio::resources_register(&resource);

	let app = Application::builder()
		.application_id("arpa.sp1rit.android-rust-test")
		.build();

	app.connect_activate(|app| {
		let window = Window::new(app);
		window.present();
	});

	unsafe { gio::ffi::g_application_run(app.as_ptr() as *mut gio::ffi::GApplication, argc, argv) }
}
