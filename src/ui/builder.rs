pub trait BuilderExtManualGetObjectExpect {
    fn get_object_expect<T: glib::object::IsA<glib::object::Object>>(&self, name: &str) -> T;
}

impl<U: gtk::prelude::BuilderExtManual> BuilderExtManualGetObjectExpect for U
where
    U: gtk::prelude::BuilderExtManual,
{
    fn get_object_expect<T: glib::object::IsA<glib::object::Object>>(&self, name: &str) -> T {
        self.get_object(name).expect(&format!(
            "Expected to get \"{}\" from the builder, but failed.",
            name
        ))
    }
}

pub fn create() -> gtk::Builder {
    let glade_src = include_str!("../../glade/ui.glade");
    gtk::Builder::new_from_string(glade_src)
}

