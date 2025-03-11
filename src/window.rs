use gtk::{
	gio,
	glib::{self, prelude::*, Object}
};

mod imp {
	use super::*;
	use gtk::{
		prelude::*,
		CompositeTemplate, TemplateChild,
		subclass::{prelude::*},
		ApplicationWindow, Button, Label,
		glib::subclass::InitializingObject
	};
	use std::cell::Cell;

	#[derive(CompositeTemplate, Default)]
	#[template(resource = "/arpa/sp1rit/gtkrs4droid/ui/window.ui")]
	pub struct Window {
		#[template_child]
		pub label: TemplateChild<Label>,
		#[template_child]
		pub dec: TemplateChild<Button>,
		#[template_child]
		pub inc: TemplateChild<Button>,

		counter: Cell<i32>
	}

	impl Window {
		fn update_label(self: &Window) {
			self.label.set_label(&format!("Counter: {}", self.counter.get()));
		}
	}

	#[glib::object_subclass]
	impl ObjectSubclass for Window {
		type ParentType = ApplicationWindow;
		type Type = super::Window;
		
		const NAME: &'static str = "GtkRsDroidWindow";


		fn class_init(klass: &mut Self::Class) {
			Self::bind_template(klass);
		}

		fn instance_init(obj: &InitializingObject<Self>) {
			obj.init_template();
		}
	}

	impl ObjectImpl for Window {
		fn constructed(&self) {
			self.dec.connect_clicked(glib::clone!(#[weak(rename_to=this)] self, move |_| {
				this.counter.set(this.counter.get() - 1);
				this.update_label();
			}));
			self.inc.connect_clicked(glib::clone!(#[weak(rename_to=this)] self, move |_| {
				this.counter.set(this.counter.get() + 1);
				this.update_label();
			}));
			self.update_label();

			self.parent_constructed();
		}
	}

	impl WidgetImpl for Window {}
	impl WindowImpl for Window {}
	impl ApplicationWindowImpl for Window {}
}

glib::wrapper! {
	pub struct Window(ObjectSubclass<imp::Window>)
		@extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
		@implements gio::ActionMap, gio::ActionGroup;
}

impl Window {
	pub fn new<P: IsA<gtk::Application>>(app: &P) -> Self {
		return Object::builder().property("application", app).build()
	}
}
