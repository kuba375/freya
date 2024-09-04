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
        Router::<PrimaryRoute> {},
        Router::<DocumentRoute> {}
    )
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum PrimaryRoute {
    #[layout(AppSidebar)]
        #[route("/")]
        Home,
        #[route("/Document")]
        Document,
    #[end_layout]
    #[route("/..route")]
    PageNotFound { },
}

#[allow(non_snake_case)]
#[component]
fn AppSidebar() -> Element {
    rsx!(
        NativeRouter {
            Sidebar {
                sidebar: rsx!(
                    Link {
                        to: PrimaryRoute::Home,
                        ActivableRoute {
                            route: PrimaryRoute::Home,
                            exact: true,
                            SidebarItem {
                                label {
                                    "Go to Hey ! 👋"
                                }
                            },
                        }
                    },
                    Link {
                        to: PrimaryRoute::Document,
                        ActivableRoute {
                            route: PrimaryRoute::Document,
                            SidebarItem {
                                label {
                                    "Go to Document! 👈"
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
                        Outlet::<PrimaryRoute> {  }
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
            "Just some text 😗 in /"
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

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum DocumentRoute {
    #[layout(AppSidebar)]
    #[route("/")]
    DocumentHome,
    #[route("/details")]
    DocumentDetails,
    #[end_layout]
    #[route("/..route")]
    DocumentPageNotFound { },
}

#[allow(non_snake_case)]
#[component]
fn Document() -> Element {
    rsx!(
        NativeRouter {
            Tabsbar {
                Link {
                    to: DocumentRoute::DocumentHome,
                    ActivableRoute {
                        route: DocumentRoute::DocumentHome,
                        exact: true,
                        Tab {
                            label {
                                "Home"
                            }
                        }
                    }
                },
                Link {
                    to: DocumentRoute::DocumentDetails,
                    ActivableRoute {
                        route: DocumentRoute::DocumentDetails,
                        Tab {
                            label {
                                "Details"
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
                    Outlet::<DocumentRoute> {  }
                }
            }
        }
    )
}

#[allow(non_snake_case)]
#[component]
fn DocumentHome() -> Element {
    rsx!(
        label {
            "home. (path: '/')"
        }
    )
}

#[allow(non_snake_case)]
#[component]
fn DocumentDetails() -> Element {
    rsx!(
        label {
            "details (path: '/details')"
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
