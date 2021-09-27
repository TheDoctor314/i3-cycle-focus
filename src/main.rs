use i3ipc::{
    reply::{Command, Node},
    I3Connection, MessageError,
};

mod iter;
mod node_ext;

use node_ext::NodeExt;

fn main() {
    let mut args = std::env::args();
    args.next();
    let mut select = args.next().expect("Expect cycle direction");
    select.make_ascii_lowercase();

    let mut connection = I3Connection::connect().unwrap();

    let active_workspace = connection
        .get_workspaces()
        .unwrap()
        .workspaces
        .into_iter()
        .find(|w| w.focused)
        .expect("Some workspace will always be focused");

    let active_workspace = find_workspace_by_name(&mut connection, active_workspace.name.as_str());
    let windows = active_workspace.leaves();

    let cur_win_idx = windows
        .iter()
        .position(|node| node.focused)
        .expect("Some window will be focused");

    let new_idx = match select.as_str().trim() {
        "c" | "cycle" => (cur_win_idx + 1) % windows.len(),
        "r" | "reverse" => (cur_win_idx - 1 + windows.len()) % windows.len(),
        _ => unreachable!(),
    };

    focus_container(&mut connection, windows[new_idx].id).unwrap();
}

fn find_workspace_by_name(conn: &mut I3Connection, name: &str) -> Node {
    conn.get_tree()
        .expect("Could not get tree")
        .workspaces()
        .iter()
        .find(|node| node.name.as_ref().map_or(false, |s| s == name))
        .copied()
        .cloned()
        .expect("Should have found the workspace")
}

#[inline(always)]
fn focus_container(conn: &mut I3Connection, id: i64) -> Result<Command, MessageError> {
    conn.run_command(&format!("[con_id={}] focus", id))
}
