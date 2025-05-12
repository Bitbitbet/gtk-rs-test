use adw::ActionRow;
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::Button;
use gtk::CompositeTemplate;
use gtk::GestureClick;
use gtk::PopoverMenu;
use gtk::gdk::BUTTON_SECONDARY;
use gtk::gdk::Rectangle;
use gtk::gio::Menu;
use gtk::gio::MenuItem;
use gtk::glib;

use crate::collection_object::CollectionObject;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/me/gtk-rs-test/test/collection_row.ui")]
pub struct CollectionRowImp {
    #[template_child]
    pub(super) suffix_button: TemplateChild<Button>,

    #[template_child]
    pub(super) rightclick_menu_model: TemplateChild<Menu>,
    #[template_child]
    rightclick_menu: TemplateChild<PopoverMenu>,
}

impl CollectionRowImp {
    pub(super) fn associate(&self, collection_object: &CollectionObject) {
        self.suffix_button
            .set_action_target(Some(collection_object.get_id().to_variant()));
        self.obj().set_title(&collection_object.title());

        let menu_item = MenuItem::new(Some("Delete"), None);
        menu_item.set_action_and_target_value(
            Some("win.remove-collection"),
            Some(&collection_object.get_id().to_variant()),
        );
        self.rightclick_menu_model.append_item(&menu_item);
    }
}

#[glib::object_subclass]
impl ObjectSubclass for CollectionRowImp {
    const NAME: &'static str = "GtkRsTestCollectionRow";
    type Type = super::CollectionRow;
    type ParentType = ActionRow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for CollectionRowImp {
    fn constructed(&self) {
        self.parent_constructed();

        self.rightclick_menu.set_parent(&*self.obj());

        let gesture_click = GestureClick::builder().button(BUTTON_SECONDARY).build();
        {
            let popover_menu = self.rightclick_menu.clone();
            gesture_click.connect_pressed(move |_, _, x, y| {
                popover_menu.set_pointing_to(Some(&Rectangle::new(x as i32, y as i32, 1, 1)));
                popover_menu.popup();
            });
        }
        self.obj().add_controller(gesture_click);
    }
    fn dispose(&self) {
        self.rightclick_menu.unparent();
    }
}

impl WidgetImpl for CollectionRowImp {}
impl ListBoxRowImpl for CollectionRowImp {}
impl PreferencesRowImpl for CollectionRowImp {}
impl ActionRowImpl for CollectionRowImp {}
