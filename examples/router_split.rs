#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::HashMap;
use freya::prelude::*;

fn main() {
    launch_with_props(app, "Split Routing Example", (550.0, 400.0));
}

struct Document {
    name: String,
    content: String,
}

// FIXME Temporarily using a `static mut` and unsafe code, maybe use once_cell or similar for this example
static mut DOCUMENTS: Option<HashMap<String, Document>> = None;

pub fn load_documents() {
    let map = HashMap::from([
        ("document_1".to_string(), Document { name: "Document 1".to_string(), content: "Content 1".to_string() }),
        ("document_2".to_string(), Document { name: "Document 2".to_string(), content: "Content 2".to_string() }),
    ]);

    unsafe { DOCUMENTS.replace(map); }
}

fn app() -> Element {

    load_documents();

    rsx!(
        tabbed_ui::TabContainer { }
    )
}

mod tabbed_ui {
    use dioxus_router::prelude::{Outlet, Routable, Router};
    use freya::components::*;
    use freya::prelude::*;

    // Tabbed UI has dependencies on the tabs it uses
    use crate::document::DocumentContainer;
    use crate::DOCUMENTS;

    #[derive(Routable, Clone, PartialEq)]
    #[rustfmt::skip]
    pub enum TabsRoute {
        #[layout(TabLayout)]
        #[route("/")]
        HomeTab,
        #[route("/document/:id")]
        DocumentTab { id: String },
        #[end_layout]
        #[route("/..route")]
        PageNotFound { },
    }

    /// Note: this is the ONLY public function
    #[allow(non_snake_case)]
    #[component]
    pub fn TabContainer() -> Element {

        let _document_ids = use_context_provider(|| {
            let document_ids: Vec<(String, String)> = unsafe { DOCUMENTS.as_ref().unwrap() }.iter().map(|(key, value)|{
                (key.clone(), value.name.clone())
            }).collect();
            Signal::new(document_ids)
        });

        rsx!(
            Router::<TabsRoute> {}
        )
    }

    #[allow(non_snake_case)]
    #[component]
    fn TabLayout() -> Element {
        let document_ids_signal: Signal<Vec<(String, String)>> = use_context();
        let document_ids = document_ids_signal.read();
        let mut sorted_document_ids = document_ids.clone();
        sorted_document_ids.sort();

        rsx!(
            NativeRouter {
                rect {
                    background: "#444444",
                    width: "fill",
                    Tabsbar {
                        Link {
                            to: TabsRoute::HomeTab,
                            ActivableRoute {
                                route: TabsRoute::HomeTab,
                                exact: true,
                                Tab {
                                    label {
                                        "Home"
                                    }
                                }
                            }
                        },
                        for (id, name) in sorted_document_ids.iter() {
                            Link {
                                key: "{id.clone()}",
                                to: TabsRoute::DocumentTab { id: id.clone() },
                                ActivableRoute {
                                    route: TabsRoute::DocumentTab { id: id.clone() },
                                    Tab {
                                        label {
                                            "{name}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                Body {
                    rect {
                        main_align: "center",
                        cross_align: "center",
                        width: "fill",
                        height: "fill",
                        Outlet::<TabsRoute> {  }
                    }
                }
            }
        )
    }

    #[allow(non_snake_case)]
    #[component]
    fn DocumentTab(id: String) -> Element {
        println!("DocumentTab. id: {}", id);

        rsx!(
            DocumentContainer { id }
        )
    }

    #[allow(non_snake_case)]
    #[component]
    fn HomeTab() -> Element {
        rsx!(
            label {
                "Home Tab Content"
            }
        )
    }

    #[allow(non_snake_case)]
    #[component]
    fn PageNotFound() -> Element {
        rsx!(
            label {
                "404!! 😵"
            }
        )
    }
}

///
/// Document UI - should have know knowledge or dependencies on the tabbed UI
///
mod document {
    use dioxus_router::{hooks::use_route, prelude::{Outlet, Routable, Router, RouterConfig}};
    use freya::prelude::*;
    use crate::{DOCUMENTS};

    struct DocumentItem {
        id: String,
    }

    static DOCUMENTS_ROUTER: GlobalSignal<Option<DocumentRoute>> = GlobalSignal::new(|| None);

    #[derive(Routable, Clone, PartialEq)]
    #[rustfmt::skip]
    enum DocumentRoute {
        #[nest("/:id")]
            #[layout(DocumentLayout)]
            #[route("/")]
            DocumentOverview { id: String },
            #[route("/content")]
            DocumentContent { id: String },
            #[end_layout]
        #[end_nest]
        #[route("/..route")]
        DocumentPageNotFound {},
    }

    impl DocumentRoute {
        pub fn id(&self) -> String {
            let mut id_signal: Signal<String> = use_context();
            id_signal()
        }
    }

    /// Note: this is the ONLY public function
    #[allow(non_snake_case)]
    #[component]
    pub fn DocumentContainer(id: String) -> Element {
        println!("DocumentContainer. id: {}", id);

        // Use a signal to store the id, so it can be used by DocumentContent and DocumentOverview
        let id_signal = use_context_provider(|| Signal::new(id.clone()));

        // Update the signal with a potentially new id.
        let id = match id_signal() {
            id_from_signal if id.ne(&id_from_signal) => {
                println!("id from signal is different. id: {}, id_from_signal: {}", &id, &id_from_signal);
                id.clone()
            },
            id_from_signal => {
                println!("id from signal is the same. id: {}, id_from_signal: {}", &id, &id_from_signal);
                id_from_signal.clone()
            }
        };

        let id_for_hook = id.clone();
        use_effect(move || {
            let mut id_signal: Signal<String> = use_context();
            id_signal.set(id_for_hook.clone());
        });

        *DOCUMENTS_ROUTER.write_unchecked() = Some(DocumentRoute::DocumentOverview { id: id.clone() });

        rsx!(
            Router::<DocumentRoute> {
                config: || RouterConfig::default().initial_route(DOCUMENTS_ROUTER().unwrap())
            }
        )
    }

    #[allow(non_snake_case)]
    #[component]
    fn DocumentLayout(id: String) -> Element {
        let route = use_route::<DocumentRoute>();

        println!("DocumentLayout. id: {}", id);

        use_effect(use_reactive!(|route| {
            *DOCUMENTS_ROUTER.write_unchecked() = Some(route);
            println!("UPDATED");
        }));

        rsx!(
            NativeRouter {
                Sidebar {
                    sidebar: rsx!(
                        Link {
                            to: DocumentRoute::DocumentOverview { id: id.clone() },
                            ActivableRoute {
                                route: DocumentRoute::DocumentOverview { id: id.clone() },
                                exact: true,
                                SidebarItem {
                                    label {
                                        "Overview"
                                    }
                                }
                            }
                        },
                        Link {
                            to: DocumentRoute::DocumentContent { id: id.clone() },
                            ActivableRoute {
                                route: DocumentRoute::DocumentContent { id: id.clone() },
                                SidebarItem {
                                    label {
                                        "Content"
                                    }
                                }
                            }
                        },
                    ),
                    Body {
                        rect {
                            main_align: "center",
                            cross_align: "center",
                            width: "fill",
                            height: "fill",
                            Outlet::<DocumentRoute> {  }
                        }
                    }
                }
            }
        )
    }

    #[allow(non_snake_case)]
    #[component]
    fn DocumentOverview(id: String) -> Element {
        println!("DocumentOverview. id: {}", id);

        rsx!(
            label {
                "Overview. (path: '/', id: {id:})"
            }
        )
    }

    #[allow(non_snake_case)]
    #[component]
    fn DocumentContent(id: String) -> Element {
        println!("DocumentContent. id: {}", id);

        let document_resource = use_resource(move || {
            let id = id.clone();
            async move {
                // FIXME: Using a static mut (but in read-only mode and which is only written once on app startup)
                let result = unsafe { DOCUMENTS.as_ref().unwrap().get(&id) };

                result
            }
        });
        let document = document_resource.read();

        match &*document {
            Some(Some(document)) => {
                rsx!(
                    label {
                        { format!("{}", document.content)}
                    }
                )
            },
            _ => {
                rsx!(
                    label {
                        "Error"
                    }
                )
            }
        }
    }

    #[allow(non_snake_case)]
    #[component]
    fn DocumentPageNotFound() -> Element {
        rsx!(
            label {
                "Document 404!! 😵"
            }
        )
    }
}