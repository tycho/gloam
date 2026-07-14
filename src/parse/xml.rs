//! Extension trait over roxmltree nodes for the element-by-tag lookups the
//! parser does everywhere, replacing the repeated
//! `.children().find(|n| n.is_element() && n.tag_name().name() == ...)`
//! boilerplate.

use roxmltree::Node;

pub(super) trait NodeExt<'a, 'input: 'a> {
    /// First child element named `tag`.
    fn child(self, tag: &str) -> Option<Node<'a, 'input>>;

    /// All child elements named `tag`, in document order.
    fn children_named(self, tag: &str) -> impl Iterator<Item = Node<'a, 'input>>;
}

impl<'a, 'input: 'a> NodeExt<'a, 'input> for Node<'a, 'input> {
    fn child(self, tag: &str) -> Option<Node<'a, 'input>> {
        self.children()
            .find(|n| n.is_element() && n.tag_name().name() == tag)
    }

    fn children_named(self, tag: &str) -> impl Iterator<Item = Node<'a, 'input>> {
        self.children()
            .filter(move |n| n.is_element() && n.tag_name().name() == tag)
    }
}
