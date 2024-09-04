#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus_router::prelude::{
    Outlet,
    Routable,
    Router,
};
use freya::prelude::*;

fn main() {
    launch_with_props(app, "Router Example", (550.0, 400.0));
}

fn app() -> Element {
    rsx!(
        Router::<TabsRoute> {}
        Router::<DocumentRoute> {},
    )
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum TabsRoute {
    #[layout(TabLayout)]
    #[route("/")]
    Home,
    #[route("/document")]
    Document,
    #[end_layout]
    #[route("/..route")]
    PageNotFound { },
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum DocumentRoute {
    #[layout(Document)]
    #[route("/")]
    DocumentOverview,
    #[route("/details")]
    DocumentDetails,
    #[end_layout]
    #[route("/..route")]
    DocumentPageNotFound { },
}

#[allow(non_snake_case)]
#[component]
fn TabLayout() -> Element {
    rsx!(
        NativeRouter {
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
                    to: TabsRoute::Document,
                    ActivableRoute {
                        route: TabsRoute::Document,
                        Tab {
                            label {
                                "Document"
                            }
                        }
                    }
                },
                Link {
                    to: TabsRoute::Document,
                    ActivableRoute {
                        route: TabsRoute::Document,
                        Tab {
                            label {
                                "Another document"
                            }
                        }
                    }
                },
            }
            Body {
                rect {
                    main_align: "center",
                    cross_align: "center",
                    width: "100%",
                    height: "100%",
                    Outlet::<TabsRoute> {  }
                }
            }
        }
    )
}

#[allow(non_snake_case)]
#[component]
fn DocumentOverview() -> Element {
    rsx!(
        label {
            "Overview. (path: '/')"
        }
    )
}

#[allow(non_snake_case)]
#[component]
fn DocumentDetails() -> Element {
    rsx!(
        label {
            "Details (path: '/details')"
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
fn Document() -> Element {
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
                            },
                        }
                    },
                    Link {
                        to: DocumentRoute::DocumentDetails,
                        ActivableRoute {
                            route: DocumentRoute::DocumentDetails,
                            SidebarItem {
                                label {
                                    "Details"
                                }
                            },
                        }
                    },
                    SidebarItem {
                        onclick: |_| println!("Hello!"),
                        label {
                            "Print Hello! 👀"
                        }
                    },
                ),
                Body {
                    rect {
                        main_align: "center",
                        cross_align: "center",
                        width: "100%",
                        height: "100%",
                        Outlet::<DocumentRoute> {  }
                    }
                }
            }
        }
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
