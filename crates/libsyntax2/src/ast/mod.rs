mod generated;

use std::sync::Arc;

use smol_str::SmolStr;

use {
    SyntaxNode, SyntaxRoot, TreeRoot, SyntaxError,
    SyntaxKind::*,
};
pub use self::generated::*;

pub trait AstNode<R: TreeRoot> {
    fn cast(syntax: SyntaxNode<R>) -> Option<Self>
        where Self: Sized;
    fn syntax(&self) -> &SyntaxNode<R>;
}

pub trait NameOwner<R: TreeRoot>: AstNode<R> {
    fn name(&self) -> Option<Name<R>> {
        self.syntax()
            .children()
            .filter_map(Name::cast)
            .next()
    }
}

impl File<Arc<SyntaxRoot>> {
    pub fn parse(text: &str) -> Self {
        File::cast(::parse(text)).unwrap()
    }
}

impl<R: TreeRoot> File<R> {
    pub fn errors(&self) -> Vec<SyntaxError> {
        self.syntax().root.errors.clone()
    }
}

impl<R: TreeRoot> FnDef<R> {
    pub fn has_atom_attr(&self, atom: &str) -> bool {
        self.syntax()
            .children()
            .filter(|node| node.kind() == ATTR)
            .any(|attr| {
                let mut metas = attr.children().filter(|node| node.kind() == META_ITEM);
                let meta = match metas.next() {
                    None => return false,
                    Some(meta) => {
                        if metas.next().is_some() {
                            return false;
                        }
                        meta
                    }
                };
                let mut children = meta.children();
                match children.next() {
                    None => false,
                    Some(child) => {
                        if children.next().is_some() {
                            return false;
                        }
                        child.kind() == IDENT && child.text() == atom
                    }
                }
            })
    }
}

impl<R: TreeRoot> Name<R> {
    pub fn text(&self) -> SmolStr {
        let ident = self.syntax().first_child()
            .unwrap();
        ident.leaf_text().unwrap()
    }
}

impl<R: TreeRoot> NameRef<R> {
    pub fn text(&self) -> SmolStr {
        let ident = self.syntax().first_child()
            .unwrap();
        ident.leaf_text().unwrap()
    }
}

impl <R: TreeRoot> ImplItem<R> {
    pub fn target_type(&self) -> Option<TypeRef<R>> {
        match self.target() {
            (Some(t), None) | (_, Some(t)) => Some(t),
            _ => None,
        }
    }

    pub fn target_trait(&self) -> Option<TypeRef<R>> {
        match self.target() {
            (Some(t), Some(_)) => Some(t),
            _ => None,
        }
    }

    fn target(&self) -> (Option<TypeRef<R>>, Option<TypeRef<R>>) {
        let mut types = self.syntax().children().filter_map(TypeRef::cast);
        let first = types.next();
        let second = types.next();
        (first, second)
    }
}