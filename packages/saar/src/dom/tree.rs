use crate::html::{Html, Attribute, ComponentRef};

use crate::dom::state::{self, Identity};
use crate::dom::component::Component;

use std::sync::Arc;
use std::any::Any;


pub enum Inner {
    Component(Identity),
    Block(fn(Context) -> String),
}

impl Inner {
    pub fn new(component: ComponentRef) -> Inner {
        match component {
            ComponentRef::Component(component) => {
                let view = component.view();

                Inner::Component(state::push(component, view))
            },
            ComponentRef::Block(f) => Inner::Block(f),
        }
    }

    pub fn render(&self, props: Props, attributes: Attributes, context: Context) -> String {
        match self {
            Inner::Component(component) => state::get(*component).render(props, attributes),
            Inner::Block(f) => f(context),
        }
    }
}

#[derive(Clone)]
pub struct Attributes {
    inner: Arc<Vec<Attribute>>,
}

impl Attributes {
    pub fn new(inner: Vec<Attribute>) -> Attributes {
        Attributes {
            inner: Arc::new(inner),
        }
    }

    pub fn render(&self) -> String {
        self.inner.iter()
            .map(|attribute| attribute.render())
            .collect::<String>()
    }
}

#[derive(Clone)]
pub struct Props {
    inner: Arc<Vec<String>>,
}

// TODO: figure out how to do the props thing
impl Props {
    pub fn new(nodes: Vec<Node>, context: Context) -> Props {
        let inner = nodes.iter()
            .map(|node| node.render(context.clone()))
            .collect::<Vec<String>>();

        Props {
            inner: Arc::new(inner),
        }
    }

    pub fn render(&self) -> String {
        self.inner.iter()
            .cloned()
            .collect::<String>()
    }
}

pub struct Node {
    inner: Inner,
    attributes: Attributes,
    props: Props,
}

impl Node {
    pub fn new(view: Html, identity: Identity) -> Node {
        let props = view.props.into_iter()
            .map(|html| Node::new(html, identity))
            .collect::<Vec<Node>>();

        // TODO: here we should hook up the callbacks

        Node {
            inner: Inner::new(view.component),
            attributes: Attributes::new(view.attributes),
            props: Props::new(props, ),
        }
    }

    pub fn render(&self, context: Context) -> String {
        self.inner.render(self.props.clone(), self.attributes.clone(), context)
    }
}

pub struct Tree {
    node: Node,
    identity: Identity,
}

impl Tree {
    pub fn new(view: Html, identity: Identity) -> Tree {
        Tree {
            node: Node::new(view, identity),
            identity,
        }
    }

    pub fn render(&self, props: Props, attributes: Attributes) -> String {
        let state = state::get(self.identity);

        self.node.render(Context::new(state.component, props, attributes))
    }
}

#[derive(Clone)]
pub struct Context {
    component: Arc<dyn Component>,
    pub props: Props,
    pub attributes: Attributes,
}

impl Context {
    pub fn new(component: Arc<dyn Component>, props: Props, attributes: Attributes) -> Context {
        Context {
            component,
            props,
            attributes,
        }
    }

    pub fn extract<T: Any>(&self, extract: T) -> String {
        self.component.extract(Box::new(extract))
    }
}


