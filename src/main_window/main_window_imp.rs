use std::cell::{OnceCell, RefCell};

use gtk::{
    CompositeTemplate, CustomFilter, Entry, EntryIconPosition, FilterListModel, Image, ListBox,
    ListBoxRow, ListItem, NoSelection, Stack, TemplateChild, Widget,
    gio::ListStore,
    glib::{self, Properties, WeakRef, subclass::InitializingObject},
    subclass::{
        widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl},
        window::WindowImpl,
    },
};

use adw::{
    AboutDialog, ActionRow, ApplicationWindow, Banner, NavigationSplitView, Toast, ToastOverlay,
    prelude::*,
};

use adw::subclass::prelude::*;
use gtk_rs_test::watcher::Watcher;

use crate::{
    collection_object::CollectionObject,
    task_object::{self, TaskObject},
};

use super::{collection_wizard::CollectionWizard, task_row::TaskRow};

#[derive(PartialEq, Default)]
pub enum FilterMode {
    #[default]
    All,
    Unresolved,
    Done,
}

impl From<&str> for FilterMode {
    fn from(value: &str) -> Self {
        match value {
            "all" => FilterMode::All,
            "unresolved" => FilterMode::Unresolved,
            "done" => FilterMode::Done,
            _ => panic!("Impossible"),
        }
    }
}

impl Into<&str> for FilterMode {
    fn into(self) -> &'static str {
        match self {
            FilterMode::All => "all",
            FilterMode::Unresolved => "unresolved",
            FilterMode::Done => "done",
        }
    }
}

#[derive(CompositeTemplate, Properties)]
#[properties(wrapper_type = super::MainWindow)]
#[template(resource = "/me/gtk-rs-test/test/main_window.ui")]
pub struct MainWindowImp {
    #[template_child]
    task_model: TemplateChild<NoSelection>,
    #[template_child]
    toast: TemplateChild<ToastOverlay>,
    #[template_child]
    banner: TemplateChild<Banner>,
    #[template_child]
    task_entry: TemplateChild<Entry>,
    #[template_child]
    stack: TemplateChild<Stack>,
    #[template_child]
    collection_list_box: TemplateChild<ListBox>,
    #[template_child]
    split_view: TemplateChild<NavigationSplitView>,

    #[property(get, set)]
    filter_mode: RefCell<String>,

    selected_collection: RefCell<Watcher<'static, Option<WeakRef<CollectionObject>>>>,
    collections: ListStore,
    task_filter: OnceCell<CustomFilter>,
}

#[gtk::template_callbacks]
impl MainWindowImp {
    #[template_callback]
    fn handle_task_add_clicked(&self, _: EntryIconPosition, _entry: &Entry) {
        self.add_new_task();
    }
    #[template_callback]
    fn handle_task_entry_activated(&self, _entry: &Entry) {
        self.add_new_task();
    }
    #[template_callback]
    fn handle_task_list_factory_setup(&self, list_item: &ListItem) {
        list_item.set_child(Some(&TaskRow::new()));
    }
    #[template_callback]
    fn handle_task_list_factory_bind(&self, list_item: &ListItem) {
        let task_object = list_item.item().and_downcast::<TaskObject>().unwrap();
        let task_row = list_item.child().and_downcast::<TaskRow>().unwrap();
        task_row.bind(&task_object);
    }
    #[template_callback]
    fn handle_task_list_factory_unbind(&self, list_item: &ListItem) {
        let task_row = list_item.child().and_downcast::<TaskRow>().unwrap();
        task_row.unbind();
    }
    #[template_callback]
    fn handle_banner_button_clicked(&self) {
        self.obj()
            .set_filter_mode(Into::<&str>::into(FilterMode::All).to_string());
    }
    #[template_callback]
    fn handle_new_collection_button_clicked(&self) {
        self.show_add_new_collection_dialog();
    }
    #[template_callback]
    fn handle_collection_row_selected(&self, list_box_row: Option<&ListBoxRow>) {
        *self.selected_collection.borrow_mut().borrow_mut() = list_box_row.map(|list_box_row| {
            ObjectExt::downgrade(
                &self
                    .collections
                    .item(list_box_row.index().try_into().unwrap())
                    .and_downcast::<CollectionObject>()
                    .unwrap(),
            )
        });
    }
    #[template_callback]
    fn handle_collection_row_activated(&self) {
        self.split_view.set_show_content(true);
    }

    fn update_banner(&self) {
        match self.filter_mode.borrow().as_str().into() {
            FilterMode::All => self.banner.set_revealed(false),
            FilterMode::Unresolved => {
                self.banner.set_title("Filter: Displaying unresolved tasks");
                self.banner.set_revealed(true);
            }
            FilterMode::Done => {
                self.banner.set_title("Filter: Displaying done tasks");
                self.banner.set_revealed(true);
            }
        }
    }

    fn add_new_task(&self) {
        let tasks = match &**self.selected_collection.borrow() {
            Some(t) => t.upgrade().unwrap().tasks(),
            None => return,
        };

        let task_name = self.task_entry.text();
        let task_name = task_name.trim();
        if task_name.len() == 0 {
            return;
        }
        tasks.append(&TaskObject::new(task_name));
        self.task_entry.set_text("");

        self.toast.add_toast(
            Toast::builder()
                .title(&format!("Task Added: {task_name}"))
                .timeout(2)
                .build(),
        );
    }

    pub(super) fn remove_done_tasks(&self) {
        let tasks = match &**self.selected_collection.borrow() {
            Some(t) => t.upgrade().unwrap().tasks(),
            None => return,
        };

        tasks.retain(|task_object| !task_object.downcast_ref::<TaskObject>().unwrap().checked());
        self.toast.add_toast(
            Toast::builder()
                .title("Removed all done tasks")
                .timeout(1)
                .build(),
        );
    }
    pub(super) fn show_about_dialog(&self) {
        AboutDialog::builder()
            .application_name("To-Do List")
            .application_icon("application-x-executable")
            .version("0.1.0")
            .developers([":)"])
            .build()
            .present(Some(&*self.obj()));
    }
    pub(super) fn remove_task_by_id(&self, id: task_object::IdType) {
        let tasks = match **self.selected_collection.borrow() {
            Some(ref t) => t.upgrade().unwrap().tasks(),
            None => return,
        };

        let mut selected_task = None;
        tasks.retain(|task| {
            let task = task.downcast_ref::<TaskObject>().unwrap();

            if task.get_id() == id {
                selected_task = Some(task.clone());
            }

            task.get_id() != id
        });

        if let Some(task) = selected_task {
            self.toast.add_toast(
                Toast::builder()
                    .title(&format!("Task Deleted: {}", task.task_name()))
                    .timeout(2)
                    .build(),
            );
        }
    }

    fn show_add_new_collection_dialog(&self) {
        CollectionWizard::new().present(Some(&*self.obj()));
    }

    pub(super) fn add_collection(&self, title: &str) {
        let new_collection = CollectionObject::new(title);
        self.collections.append(&new_collection);
        *self.selected_collection.borrow_mut().borrow_mut() =
            Some(ObjectExt::downgrade(&new_collection));

        self.collection_list_box.select_row(Some(
            &self
                .collection_list_box
                .row_at_index((self.collections.n_items() - 1) as i32)
                .unwrap(),
        ));

        self.stack.set_visible_child_name("main");
        self.set_focus_child(Some(self.task_entry.upcast_ref()));
    }

    fn set_currently_displayed_model(&self, model: ListStore) {
        self.task_model.set_model(Some(&FilterListModel::new(
            Some(model),
            Some(self.task_filter.get().unwrap().clone()),
        )));
    }
}

impl Default for MainWindowImp {
    fn default() -> Self {
        Self {
            task_model: Default::default(),
            toast: Default::default(),
            filter_mode: Default::default(),
            banner: Default::default(),
            task_entry: Default::default(),
            stack: Default::default(),
            collection_list_box: Default::default(),
            selected_collection: Default::default(),
            split_view: Default::default(),

            task_filter: Default::default(),
            collections: ListStore::new::<CollectionObject>(),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for MainWindowImp {
    const NAME: &'static str = "GtkRsTestMainWindow";
    type Type = super::MainWindow;
    type ParentType = ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for MainWindowImp {
    fn constructed(&self) {
        self.parent_constructed();

        self.obj().setup_actions();
        // Initialize the fiter
        let filter = self.task_filter.get_or_init(|| {
            let window = self.downgrade();
            CustomFilter::new(move |task_object| {
                let checked = task_object.downcast_ref::<TaskObject>().unwrap().checked();
                let window = window.upgrade().unwrap();
                let filter_mode: FilterMode = window.filter_mode.borrow().as_str().into();

                filter_mode == FilterMode::All
                    || (filter_mode == FilterMode::Unresolved && !checked)
                    || (filter_mode == FilterMode::Done && checked)
            })
        });

        // Notify about changes and change banner state when filter mode is changed
        {
            let filter = ObjectExt::downgrade(filter);
            self.obj().connect_filter_mode_notify(move |window| {
                let filter = filter.upgrade().unwrap();
                filter.changed(gtk::FilterChange::Different);

                window.imp().update_banner();
            });
        }

        self.collection_list_box
            .bind_model(Some(&self.collections), |collection_object| {
                let collection_object = collection_object
                    .downcast_ref::<CollectionObject>()
                    .unwrap()
                    .clone();

                let row = ActionRow::builder()
                    .title(collection_object.title())
                    .activatable(true)
                    .build();
                row.add_suffix(&Image::builder().icon_name("right-smaller-symbolic").build());

                row.upcast::<Widget>()
            });

        {
            let window = self.downgrade();
            self.selected_collection
                .borrow_mut()
                .watch(move |collection_object| {
                    let window = window.upgrade().unwrap();
                    let collection_object = if let Some(c) = collection_object {
                        c.upgrade().unwrap()
                    } else {
                        return;
                    };
                    window.set_currently_displayed_model(collection_object.tasks());
                });
        }
    }
}
impl WidgetImpl for MainWindowImp {}
impl WindowImpl for MainWindowImp {}
impl ApplicationWindowImpl for MainWindowImp {}
impl AdwApplicationWindowImpl for MainWindowImp {}
