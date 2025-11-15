pub struct User;
pub struct Html;

mod admin {
    use crate::website::User;

    pub struct Admin;

    impl User {
        pub fn try_admin(&self) -> Option<Admin> {
            Some(Admin)
        }
    }
}

fn render_admin_panel(_admin: admin::Admin) -> Html {
    Html
}

fn admin_panel_route_ok() -> Html {
    if let Some(admin) = User.try_admin() {
        render_admin_panel(admin)
    } else {
        render_404()
    }
}

fn render_404() -> Html {
    Html
}

fn admin_panel_route_whoops() -> Html {
    render_admin_panel(User.try_admin().unwrap());

    render_admin_panel(admin::Admin {})
}
