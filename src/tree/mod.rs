mod container;
mod obj;
mod path;

pub(crate) enum TreeNode {
    Container(container::Container),
    Leaf(obj::Obj),
}
