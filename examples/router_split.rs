#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::HashMap;
use dioxus_router::prelude::{
    Outlet,
    Routable,
    Router,
};
use freya::prelude::*;

fn main() {
    launch_with_props(app, "Router Example", (550.0, 400.0));
}

struct Document {
    content: String,
}

fn app() -> Element {

    let _documents = use_context_provider(|| {
        let mut documents_map = HashMap::<String, Document>::new();
        documents_map.insert("document_1".to_string(), Document { content: "Document 1".to_string() });
        documents_map.insert("document_2".to_string(), Document { content: "Document 2".to_string() });

        Signal::new(documents_map)
    });

    let _active_tab = use_context_provider(|| Signal::new(Option::<String>::None));

    rsx!(
        Router::<TabsRoute> {}
    )
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum TabsRoute {
    #[layout(TabLayout)]
    #[route("/")]
    Home,
    #[route("/document/:id")]
    DocumentContainer { id: String },
    #[end_layout]
    #[route("/..route")]
    PageNotFound { },
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum DocumentRoute {
    #[layout(DocumentLayout)]
    #[route("/")]
    DocumentOverview,
    #[route("/content")]
    DocumentContent,
    #[end_layout]
    #[route("/..route")]
    DocumentPageNotFound { },
}

#[allow(non_snake_case)]
#[component]
fn TabLayout() -> Element {
    rsx!(
        NativeRouter {
            rect {
                background: "#444444",
                width: "fill",
                Tabsbar {
                    Link {
                        to: TabsRoute::Home,
                        ActivableRoute {
                            route: TabsRoute::Home,
                            exact: true,
                            Tab {
                                label {
                                    "Home"
                                }
                            }
                        }
                    },
                    Link {
                        to: TabsRoute::DocumentContainer { id: "document_1".to_string() },
                        ActivableRoute {
                            route: TabsRoute::DocumentContainer { id: "document_1".to_string() },
                            Tab {
                                label {
                                    "Document 1"
                                }
                            }
                        }
                    },
                    Link {
                        to: TabsRoute::DocumentContainer { id: "document_2".to_string() },
                        ActivableRoute {
                            route: TabsRoute::DocumentContainer { id: "document_2".to_string() },
                            Tab {
                                label {
                                    "Document 2"
                                }
                            }
                        }
                    },
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
fn DocumentLayout() -> Element {
    let active_tab: Signal<Option::<String>> = use_context();
    let id = active_tab.clone().unwrap();

    println!("document layout. id: {}", id);

    rsx!(
        NativeRouter {
            Sidebar {
                sidebar: rsx!(
                    Link {
                        to: DocumentRoute::DocumentOverview,
                        ActivableRoute {
                            route: DocumentRoute::DocumentOverview,
                            exact: true,
                            SidebarItem {
                                label {
                                    "Overview"
                                }
                            }
                        }
                    },
                    Link {
                        to: DocumentRoute::DocumentContent,
                        ActivableRoute {
                            route: DocumentRoute::DocumentContent,
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
fn DocumentOverview() -> Element {
    let active_tab: Signal<Option::<String>> = use_context();
    let id = active_tab.clone().unwrap();

    println!("overview. id: {}", id);

    rsx!(
        label {
            "Overview. (path: '/', id: {id:})"
        }
    )
}

#[allow(non_snake_case)]
#[component]
fn DocumentContent() -> Element {
    let active_tab: Signal<Option::<String>> = use_context();
    let id = active_tab.clone().unwrap();

    println!("content. id: {}", id);

    let documents_signal: Signal<HashMap<String, Document>> = use_context();
    let documents = documents_signal.read();
    let document = documents.get(&id).unwrap();

    rsx!(
        label {
            { format!("{}", document.content)}
        }
    )
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


#[allow(non_snake_case)]
#[component]
fn DocumentContainer(id: String) -> Element {
    println!("id: {}", id);

    let mut active_tab: Signal<Option::<String>> = use_context();
    active_tab.replace(Some(id));

    rsx!(
        Router::<DocumentRoute> {}
    )
}

#[allow(non_snake_case)]
#[component]
fn Home() -> Element {
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