pub mod main_content;
pub mod space_content;

mod mock {
    use context::node::{Node, NodeData, NodeId};
    use context::space::{ProfileContext, ProfileId, SidebarContext, SpaceContext};
    use context::{BrowserData, BrowserParams, TabData};

    // ---------- builders ----------

    fn tab_node(name: &str, position: u8, open: bool, url: &str) -> Node {
        let params = BrowserParams {
            url: url.to_string(),
            favicon: url.to_string(),
        };
        Node::new(
            name.to_string(),
            NodeData::new_tab_node(TabData::new::<BrowserData>(params), open),
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

    fn sidebar(favorites: Vec<NodeId>, folders: Vec<NodeId>, all: Vec<Node>) -> SidebarContext {
        let nodes = all.into_iter().map(|n| (n.id.clone(), n)).collect();

        SidebarContext::new()
            .with_nodes(nodes)
            .with_favorite(favorites)
            .with_folder(folders)
    }

    // ---------- sidebars ----------

    pub fn personal_sidebar() -> SidebarContext {
        // favorites
        let fav1 = tab_node("Google", 1, false, "https://google.com");
        let fav2 = tab_node("YouTube", 2, false, "https://youtube.com");
        let fav3 = tab_node("Hacker News", 3, true, "https://news.ycombinator.com");

        // folder1 (expanded) and folder2 (collapsed)
        let f1_t1 = tab_node("Rust docs", 1, true, "https://doc.rust-lang.org");
        let f1_t2 = tab_node("crates.io", 2, false, "https://crates.io");
        let f1_t3 = tab_node("docs.rs", 3, false, "https://docs.rs");
        let f2_t1 = tab_node("Wikipedia", 1, false, "https://wikipedia.org");
        let folder1 = folder_node("dev", 1, true, &[&f1_t1, &f1_t2, &f1_t3]);
        let folder2 = folder_node("reading", 2, false, &[&f2_t1]);

        // nested: folder3 -> subfolder -> subsubfolder
        let blog = tab_node("Blog", 1, false, "https://blog.rust-lang.org");
        let issues = tab_node(
            "Issues",
            1,
            false,
            "https://github.com/rust-lang/rust/issues",
        );
        let pulls = tab_node(
            "Pull requests",
            2,
            false,
            "https://github.com/rust-lang/rust/pulls",
        );
        let subsub = folder_node("github", 2, true, &[&issues, &pulls]);
        let sub1 = folder_node("rust", 1, true, &[&blog, &subsub]);
        let folder3 = folder_node("projects", 3, true, &[&sub1]);

        sidebar(
            ids(&[&fav1, &fav2, &fav3]),
            ids(&[&folder1, &folder2, &folder3]),
            vec![
                fav1, fav2, fav3, f1_t1, f1_t2, f1_t3, f2_t1, folder1, folder2, blog, issues,
                pulls, subsub, sub1, folder3,
            ],
        )
    }

    pub fn work_sidebar() -> SidebarContext {
        let fav1 = tab_node("Mail", 1, false, "https://mail.google.com");
        let fav2 = tab_node("Calendar", 2, false, "https://calendar.google.com");
        let t1 = tab_node("Repo", 1, false, "https://github.com/ddeedev/silly");
        let t2 = tab_node(
            "Commits",
            2,
            false,
            "https://github.com/ddeedev/silly/commits",
        );
        let github = folder_node("github", 1, true, &[&t1, &t2]);

        sidebar(
            ids(&[&fav1, &fav2]),
            ids(&[&github]),
            vec![fav1, fav2, t1, t2, github],
        )
    }

    pub fn lab_sidebar() -> SidebarContext {
        let fav = tab_node("Example", 1, false, "https://example.com");
        let t1 = tab_node("httpbin", 1, false, "https://httpbin.org");
        let tools = folder_node("tools", 1, false, &[&t1]);

        sidebar(ids(&[&fav]), ids(&[&tools]), vec![fav, t1, tools])
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
