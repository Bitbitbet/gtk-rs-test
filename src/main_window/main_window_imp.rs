use std::{
    cell::{OnceCell, RefCell},
    error::Error,
    fs,
    io::Write,
};

use gtk::{
    CompositeTemplate, CustomFilter, Entry, EntryIconPosition, FilterListModel, ListBox,
    ListBoxRow, ListItem, NoSelection, Stack, TemplateChild, Widget,
    gio::ListStore,
    glib::{self, Properties, WeakRef, clone::Downgrade, subclass::InitializingObject},
    subclass::{
        widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl},
        window::WindowImpl,
    },
};

use adw::{
    AboutDialog, ApplicationWindow, Banner, NavigationSplitView, Toast, ToastOverlay, prelude::*,
};

use adw::subclass::prelude::*;
use gtk_rs_test::{list_store_ser::ListStoreSer, watcher::Watcher};

use crate::{
    collection_object::{self, CollectionObject},
    data_path,
    task_object::{self, TaskObject},
};

use super::{
    collection_row::CollectionRow, collection_wizard::CollectionWizard, task_row::TaskRow,
};

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

    #[property(get, set)]
    task_page_title: RefCell<String>,

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

        let name = self.task_entry.text();
        let name = name.trim();
        if name.len() == 0 {
            return;
        }
        tasks.append(&TaskObject::new(name));
        self.task_entry.set_text("");

        self.toast.add_toast(
            Toast::builder()
                .title(&format!("Task Added: {name}"))
                .timeout(2)
                .build(),
        );
    }
    fn show_add_new_collection_dialog(&self) {
        CollectionWizard::new().present(Some(&*self.obj()));
    }

    /// Save state to filesystem
    pub(super) fn save(&self) {
        let mut path = data_path();

        let result = (|| -> Result<(), Box<dyn Error>> {
            fs::create_dir_all(&path)?;
            path.push("collections.json");
            let v = serde_json::to_vec(&ListStoreSer::<CollectionObject>::new(
                self.collections.clone(),
            ))?;

            fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(path)?
                .write_all(v.as_slice())?;

            Ok(())
        })();
        if let Err(err) = result {
            eprintln!("Error occurred trying to save collections: {err}")
        }
    }

    pub(super) fn remove_collection_by_id(&self, id: collection_object::IdType) {
        self.collections.retain(|c| {
            let c = c.downcast_ref::<CollectionObject>().unwrap();

            c.get_id() != id
        });
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
                    .title(&format!("Task Deleted: {}", task.name()))
                    .timeout(2)
                    .build(),
            );
        }
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

        self.set_focus_child(Some(self.task_entry.upcast_ref()));
    }
    pub(super) fn select_collection(&self, id: collection_object::IdType) {
        let (index, collection_object) = match self
            .collections
            .iter::<CollectionObject>()
            .map(|c| c.unwrap())
            .enumerate()
            .find(|(_, c)| c.get_id() == id)
        {
            Some(p) => p,
            None => return,
        };

        let list_box_row = self.collection_list_box.row_at_index(index as i32);
        self.collection_list_box.select_row(list_box_row.as_ref());

        *self.selected_collection.borrow_mut().borrow_mut() =
            Some(Downgrade::downgrade(&collection_object));
        self.split_view.set_show_content(true);
    }
}

impl Default for MainWindowImp {
    fn default() -> Self {
        Self {
            task_model: Default::default(),
            toast: Default::default(),
            filter_mode: RefCell::new(String::from("all")),
            banner: Default::default(),
            task_entry: Default::default(),
            stack: Default::default(),
            collection_list_box: Default::default(),
            selected_collection: Default::default(),
            split_view: Default::default(),
            task_page_title: RefCell::new(String::from("Tasks")),

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

        // Configure widget building for ListBox of collections
        self.collection_list_box
            .bind_model(Some(&self.collections), |collection_object| {
                let collection_object = collection_object
                    .downcast_ref::<CollectionObject>()
                    .unwrap()
                    .clone();

                CollectionRow::new(&collection_object).upcast::<Widget>()
            });

        // Watch the selected collection for changes
        {
            let window = self.downgrade();
            self.selected_collection
                .borrow_mut()
                .watch(move |collection_object| {
                    let window = window.upgrade().unwrap();
                    if let Some(c) = collection_object {
                        let c = c.upgrade().unwrap();
                        window.task_model.set_model(Some(&FilterListModel::new(
                            Some(c.tasks()),
                            Some(window.task_filter.get().unwrap().clone()),
                        )));
                        window
                            .obj()
                            .set_task_page_title(format!("Tasks of {}", c.title()));
                    } else {
                        window.task_model.set_model(None::<&FilterListModel>);
                        window.obj().set_task_page_title("Tasks");
                    }
                });
        }

        // Handle the outer stack page switching
        let stack = self.stack.clone();
        self.collections
            .connect_items_changed(move |collections, _, _, _| {
                stack.set_visible_child_name(if collections.n_items() == 0 {
                    "placeholder"
                } else {
                    "main"
                })
            });

        // Restore state from filesystem
        let mut path = data_path();

        let result = (|| -> Result<(), Box<dyn Error>> {
            fs::create_dir_all(&path)?;
            path.push("collections.json");

            let file = fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(path)?;
            let v: ListStoreSer<CollectionObject> = serde_json::from_reader(&file)?;

            for c in v.extract().iter::<CollectionObject>().map(Result::unwrap) {
                self.collections.append(&c);
            }

            Ok(())
        })();
        if let Err(err) = result {
            eprintln!("Error occurred trying to load collections: {err}")
        }

        self.obj().connect_close_request(|w| {
            WidgetExt::activate_action(w, "win.save", None).unwrap();

            return glib::Propagation::Proceed;
        });
    }
}
impl WidgetImpl for MainWindowImp {}
impl WindowImpl for MainWindowImp {}
impl ApplicationWindowImpl for MainWindowImp {}
impl AdwApplicationWindowImpl for MainWindowImp {}
