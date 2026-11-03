pub mod main_content;
pub mod space_content;

mod mock {
    use std::collections::HashMap;

    use context::TabData;
    use context::api::tab::{ApiRequestData, ApiRequestParams};
    use context::node::{Node, NodeData, NodeId};
    use context::space::{ProfileContext, ProfileId, SidebarContext, SpaceContext};
    use http::Method;

    // ---------- builders ----------

    fn call(method: Method, url: &str) -> ApiRequestParams {
        ApiRequestParams {
            url: url.to_string(),
            method,
            ..Default::default()
        }
    }

    fn req(url: &str) -> ApiRequestParams {
        call(Method::GET, url)
    }

    fn json_headers() -> HashMap<String, String> {
        HashMap::from([("Content-Type".to_string(), "application/json".to_string())])
    }

    fn tab_node(name: &str, position: u8, open: bool, p: ApiRequestParams) -> Node {
        Node::new(
            name.to_string(),
            NodeData::new_tab_node(TabData::new::<ApiRequestData>(p), open),
            position,
        )
    }

    fn folder_node(name: &str, position: u8, expand: bool, children: &[&Node]) -> Node {
        Node::new(
            name.to_string(),
            NodeData::new_folder_node(ids(children), expand),
            position,
        )
    }

    fn ids(nodes: &[&Node]) -> Vec<NodeId> {
        nodes.iter().map(|n| n.id.clone()).collect()
    }

    /// Takes ids first (borrowing), then consumes the nodes into the map.
    fn sidebar(favorites: Vec<NodeId>, folders: Vec<NodeId>, all: Vec<Node>) -> SidebarContext {
        let nodes: HashMap<NodeId, Node> = all.into_iter().map(|n| (n.id.clone(), n)).collect();

        SidebarContext::new()
            .with_nodes(nodes)
            .with_favorite(favorites)
            .with_folder(folders)
    }

    // ---------- sidebars ----------

    pub fn personal_sidebar() -> SidebarContext {
        // favorites
        let fav1 = tab_node("fav1", 1, false, req("localhost/api/ping"));
        let fav2 = tab_node("fav2", 2, false, req("localhost/api/health"));
        let fav3 = tab_node(
            "fav3",
            3,
            true,
            ApiRequestParams {
                params: HashMap::from([("page".into(), "1".into())]),
                headers: HashMap::from([("Accept".into(), "application/json".into())]),
                ..req("localhost/api/users")
            },
        );

        // folder1 (expanded) and folder2 (collapsed)
        let f1_t1 = tab_node(
            "request1",
            1,
            true,
            ApiRequestParams {
                params: HashMap::from([("page".into(), "1".into())]),
                ..req("localhost/api/users")
            },
        );
        let f1_t2 = tab_node("request2", 2, false, req("localhost/api/teams"));
        let f1_t3 = tab_node(
            "create_user",
            3,
            false,
            ApiRequestParams {
                body: r#"{"name": "mock_user"}"#.into(),
                auth: "Bearer mock-token".into(),
                headers: json_headers(),
                ..call(Method::POST, "localhost/api/users")
            },
        );
        let f2_t1 = tab_node("request3", 1, false, req("localhost/api/matches"));
        let folder1 = folder_node("folder1", 1, true, &[&f1_t1, &f1_t2, &f1_t3]);
        let folder2 = folder_node("folder2", 2, false, &[&f2_t1]);

        // nested: folder3 -> subfolder1 -> subsubfolder
        let get_user = tab_node("get_user", 1, false, req("localhost/api/users/1"));
        let delete_user = tab_node(
            "delete_user",
            1,
            false,
            ApiRequestParams {
                auth: "Bearer mock-token".into(),
                ..call(Method::DELETE, "localhost/api/users/1")
            },
        );
        let update_user = tab_node(
            "update_user",
            2,
            false,
            ApiRequestParams {
                body: r#"{"name": "renamed_user"}"#.into(),
                auth: "Bearer mock-token".into(),
                headers: json_headers(),
                ..call(Method::PATCH, "localhost/api/users/1")
            },
        );
        let subsub = folder_node("subsubfolder", 2, true, &[&delete_user, &update_user]);
        let sub1 = folder_node("subfolder1", 1, true, &[&get_user, &subsub]);
        let folder3 = folder_node("folder3", 3, true, &[&sub1]);

        sidebar(
            ids(&[&fav1, &fav2, &fav3]),
            ids(&[&folder1, &folder2, &folder3]),
            vec![
                fav1,
                fav2,
                fav3,
                f1_t1,
                f1_t2,
                f1_t3,
                f2_t1,
                folder1,
                folder2,
                get_user,
                delete_user,
                update_user,
                subsub,
                sub1,
                folder3,
            ],
        )
    }

    pub fn work_sidebar() -> SidebarContext {
        let fav1 = tab_node("list_repos", 1, false, req("api.github.com/user/repos"));
        let fav2 = tab_node(
            "create_issue",
            2,
            false,
            call(Method::POST, "api.github.com/repos/ddeedev/silly/issues"),
        );
        let t1 = tab_node(
            "get_repo",
            1,
            false,
            req("api.github.com/repos/ddeedev/silly"),
        );
        let t2 = tab_node(
            "list_commits",
            2,
            false,
            req("api.github.com/repos/ddeedev/silly/commits"),
        );
        let github = folder_node("github", 1, true, &[&t1, &t2]);

        sidebar(
            ids(&[&fav1, &fav2]),
            ids(&[&github]),
            vec![fav1, fav2, t1, t2, github],
        )
    }

    pub fn lab_sidebar() -> SidebarContext {
        let fav = tab_node("get_ip", 1, false, req("httpbin.org/ip"));
        let t1 = tab_node(
            "post_echo",
            1,
            false,
            call(Method::POST, "httpbin.org/post"),
        );
        let httpbin = folder_node("httpbin", 1, false, &[&t1]);

        sidebar(ids(&[&fav]), ids(&[&httpbin]), vec![fav, t1, httpbin])
    }

    // ---------- spaces / profiles ----------

    pub fn profile(pseudonym: &str) -> ProfileContext {
        ProfileContext {
            id: ProfileId::new(),
            pseudonym: pseudonym.to_string(),
            email: None,
        }
    }

    fn space(name: &str, position: usize, profile: &str, sidebar: SidebarContext) -> SpaceContext {
        let mut space = SpaceContext::new(name.to_string(), position, profile.to_string());
        space.sidebar = sidebar;
        space
    }

    pub fn spaces() -> Vec<SpaceContext> {
        vec![
            space("personal", 1, "profile1", personal_sidebar()),
            space("work", 2, "profile2", work_sidebar()),
            space("lab", 3, "profile3", lab_sidebar()),
        ]
    }
}
