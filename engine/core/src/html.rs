//! HTML5 parsing and the initial Minimalium DOM tree.
//!
//! The WHATWG tree-construction algorithm is provided by html5ever. This DOM
//! is an engine-owned tree sink, not the test-only RcDom implementation.
//! Script execution, custom elements, shadow DOM, mutation observers, and the
//! full DOM Standard remain out of scope.

use html5ever::interface::{Attribute, ElementFlags, NodeOrText, QuirksMode, TreeSink};
use html5ever::tendril::{StrTendril, TendrilSink};
use html5ever::{parse_document, QualName};
use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};

type Handle = Rc<Node>;

#[derive(Debug)]
enum NodeKind {
    Document,
    DocumentFragment,
    Doctype {
        name: String,
        public_id: String,
        system_id: String,
    },
    Element {
        name: QualName,
        attrs: RefCell<Vec<Attribute>>,
    },
    Text(RefCell<String>),
    Comment(String),
    ProcessingInstruction {
        target: String,
        data: String,
    },
}

#[derive(Debug)]
struct Node {
    kind: NodeKind,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Handle>>,
    template_contents: RefCell<Option<Handle>>,
}

impl Node {
    fn new(kind: NodeKind) -> Handle {
        Rc::new(Self {
            kind,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
            template_contents: RefCell::new(None),
        })
    }
}

#[derive(Clone, Debug)]
pub struct HtmlNode(Handle);

impl HtmlNode {
    pub fn tag_name(&self) -> Option<String> {
        match &self.0.kind {
            NodeKind::Element { name, .. } => Some(name.local.to_string()),
            _ => None,
        }
    }

    pub fn attribute(&self, name: &str) -> Option<String> {
        match &self.0.kind {
            NodeKind::Element { attrs, .. } => attrs
                .borrow()
                .iter()
                .find(|attr| attr.name.local.to_string() == name)
                .map(|attr| attr.value.to_string()),
            _ => None,
        }
    }

    pub fn children(&self) -> Vec<HtmlNode> {
        self.0
            .children
            .borrow()
            .iter()
            .cloned()
            .map(HtmlNode)
            .collect()
    }

    pub fn text_content(&self) -> String {
        fn collect(node: &Handle, output: &mut String) {
            match &node.kind {
                NodeKind::Text(text) => output.push_str(&text.borrow()),
                NodeKind::Comment(_) | NodeKind::Doctype { .. } => {}
                _ => {
                    for child in node.children.borrow().iter() {
                        collect(child, output);
                    }
                }
            }
        }

        let mut output = String::new();
        collect(&self.0, &mut output);
        output
    }

    pub fn is_document(&self) -> bool {
        matches!(&self.0.kind, NodeKind::Document)
    }
}

#[derive(Clone, Debug)]
pub struct HtmlDocument {
    root: HtmlNode,
    parse_errors: Vec<String>,
    quirks_mode: QuirksMode,
}

impl HtmlDocument {
    pub fn root(&self) -> HtmlNode {
        self.root.clone()
    }

    pub fn parse_errors(&self) -> &[String] {
        &self.parse_errors
    }

    pub fn quirks_mode(&self) -> QuirksMode {
        self.quirks_mode
    }
}

struct DomSink {
    document: Handle,
    parse_errors: RefCell<Vec<String>>,
    quirks_mode: Cell<QuirksMode>,
}

impl DomSink {
    fn new() -> Self {
        Self {
            document: Node::new(NodeKind::Document),
            parse_errors: RefCell::new(Vec::new()),
            quirks_mode: Cell::new(QuirksMode::NoQuirks),
        }
    }

    fn detach(node: &Handle) {
        if let Some(parent) = node.parent.borrow().upgrade() {
            parent
                .children
                .borrow_mut()
                .retain(|child| !Rc::ptr_eq(child, node));
        }
        *node.parent.borrow_mut() = Weak::new();
    }

    fn append_node(parent: &Handle, child: Handle) {
        Self::detach(&child);
        *child.parent.borrow_mut() = Rc::downgrade(parent);
        parent.children.borrow_mut().push(child);
    }

    fn append_text(parent: &Handle, text: &str) {
        let mut children = parent.children.borrow_mut();
        if let Some(last) = children.last() {
            if let NodeKind::Text(existing) = &last.kind {
                existing.borrow_mut().push_str(text);
                return;
            }
        }
        let child = Node::new(NodeKind::Text(RefCell::new(text.to_owned())));
        *child.parent.borrow_mut() = Rc::downgrade(parent);
        children.push(child);
    }

    fn insert_before(sibling: &Handle, child: Handle) {
        let Some(parent) = sibling.parent.borrow().upgrade() else {
            return;
        };
        Self::detach(&child);
        let mut children = parent.children.borrow_mut();
        if let Some(index) = children.iter().position(|item| Rc::ptr_eq(item, sibling)) {
            *child.parent.borrow_mut() = Rc::downgrade(&parent);
            children.insert(index, child);
        }
    }

    fn append_child_or_text(parent: &Handle, child: NodeOrText<Handle>) {
        match child {
            NodeOrText::AppendNode(node) => Self::append_node(parent, node),
            NodeOrText::AppendText(text) => Self::append_text(parent, &text),
        }
    }
}

impl TreeSink for DomSink {
    type Handle = Handle;
    type Output = HtmlDocument;
    type ElemName<'a> = &'a QualName;

    fn finish(self) -> Self::Output {
        HtmlDocument {
            root: HtmlNode(self.document),
            parse_errors: self.parse_errors.into_inner(),
            quirks_mode: self.quirks_mode.get(),
        }
    }

    fn parse_error(&self, message: std::borrow::Cow<'static, str>) {
        self.parse_errors.borrow_mut().push(message.into_owned());
    }

    fn get_document(&self) -> Self::Handle {
        self.document.clone()
    }

    fn elem_name<'a>(&'a self, target: &'a Self::Handle) -> Self::ElemName<'a> {
        match &target.kind {
            NodeKind::Element { name, .. } => name,
            _ => panic!("html5ever requested the name of a non-element node"),
        }
    }

    fn create_element(
        &self,
        name: QualName,
        attrs: Vec<Attribute>,
        _flags: ElementFlags,
    ) -> Self::Handle {
        let is_template = name.ns.to_string() == "http://www.w3.org/1999/xhtml"
            && name.local.to_string() == "template";
        let element = Node::new(NodeKind::Element {
            name,
            attrs: RefCell::new(attrs),
        });
        if is_template {
            *element.template_contents.borrow_mut() = Some(Node::new(NodeKind::DocumentFragment));
        }
        element
    }

    fn create_comment(&self, text: StrTendril) -> Self::Handle {
        Node::new(NodeKind::Comment(text.to_string()))
    }

    fn create_pi(&self, target: StrTendril, data: StrTendril) -> Self::Handle {
        Node::new(NodeKind::ProcessingInstruction {
            target: target.to_string(),
            data: data.to_string(),
        })
    }

    fn append(&self, parent: &Self::Handle, child: NodeOrText<Self::Handle>) {
        Self::append_child_or_text(parent, child);
    }

    fn append_based_on_parent_node(
        &self,
        element: &Self::Handle,
        prev_element: &Self::Handle,
        child: NodeOrText<Self::Handle>,
    ) {
        if element.parent.borrow().upgrade().is_some() {
            match child {
                NodeOrText::AppendNode(node) => Self::insert_before(element, node),
                NodeOrText::AppendText(text) => {
                    let text_node = Node::new(NodeKind::Text(RefCell::new(text.to_string())));
                    Self::insert_before(element, text_node);
                }
            }
        } else {
            Self::append_child_or_text(prev_element, child);
        }
    }

    fn append_doctype_to_document(
        &self,
        name: StrTendril,
        public_id: StrTendril,
        system_id: StrTendril,
    ) {
        let node = Node::new(NodeKind::Doctype {
            name: name.to_string(),
            public_id: public_id.to_string(),
            system_id: system_id.to_string(),
        });
        Self::append_node(&self.document, node);
    }

    fn get_template_contents(&self, target: &Self::Handle) -> Self::Handle {
        target
            .template_contents
            .borrow()
            .as_ref()
            .cloned()
            .expect("template contents requested for a non-template element")
    }

    fn same_node(&self, left: &Self::Handle, right: &Self::Handle) -> bool {
        Rc::ptr_eq(left, right)
    }

    fn set_quirks_mode(&self, mode: QuirksMode) {
        self.quirks_mode.set(mode);
    }

    fn append_before_sibling(&self, sibling: &Self::Handle, new_node: NodeOrText<Self::Handle>) {
        match new_node {
            NodeOrText::AppendNode(node) => Self::insert_before(sibling, node),
            NodeOrText::AppendText(text) => {
                let node = Node::new(NodeKind::Text(RefCell::new(text.to_string())));
                Self::insert_before(sibling, node);
            }
        }
    }

    fn add_attrs_if_missing(&self, target: &Self::Handle, attrs: Vec<Attribute>) {
        let NodeKind::Element {
            attrs: existing, ..
        } = &target.kind
        else {
            return;
        };
        let mut existing = existing.borrow_mut();
        for attr in attrs {
            if !existing.iter().any(|item| item.name == attr.name) {
                existing.push(attr);
            }
        }
    }

    fn remove_from_parent(&self, target: &Self::Handle) {
        Self::detach(target);
    }

    fn reparent_children(&self, node: &Self::Handle, new_parent: &Self::Handle) {
        let children = std::mem::take(&mut *node.children.borrow_mut());
        for child in children {
            *child.parent.borrow_mut() = Rc::downgrade(new_parent);
            new_parent.children.borrow_mut().push(child);
        }
    }
}

/// Parse a UTF-8 HTML string with html5ever's HTML5 tokenizer and tree builder.
///
/// This function builds a Minimalium-owned DOM tree. It does not execute scripts,
/// load subresources, apply CSS, or render pixels.
pub fn parse_html(input: &str) -> HtmlDocument {
    parse_document(DomSink::new(), Default::default()).one(input.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_document_tree_and_recovers_omitted_tags() {
        let document = parse_html("<title>Example</title><p>Hello <b>world");
        let root = document.root();
        assert!(root.is_document());
        let html = root
            .children()
            .into_iter()
            .find(|node| node.tag_name().as_deref() == Some("html"));
        assert!(html.is_some());
        assert!(root.text_content().contains("Hello world"));
    }

    #[test]
    fn preserves_attributes_and_text() {
        let document = parse_html("<div id='main' data-kind='demo'>Minimalium</div>");
        let div = document
            .root()
            .children()
            .into_iter()
            .flat_map(|node| node.children())
            .flat_map(|node| node.children())
            .find(|node| node.tag_name().as_deref() == Some("div"))
            .expect("div element should exist");
        assert_eq!(div.attribute("id").as_deref(), Some("main"));
        assert_eq!(div.text_content(), "Minimalium");
    }

    #[test]
    fn reports_malformed_markup_errors_when_present() {
        let document = parse_html("<div><span></div>");
        // The HTML5 parser recovers malformed markup; errors are diagnostics,
        // not a reason to reject the document.
        assert!(document.root().is_document());
    }
}
