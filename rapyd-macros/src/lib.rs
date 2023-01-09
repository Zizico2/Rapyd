use std::{
    collections::VecDeque,
    iter::{self, Peekable},
    mem,
};

use proc_macro2::TokenStream;
use proc_macro_error::{
    abort_if_dirty, emit_call_site_error, emit_call_site_warning, emit_error, emit_warning,
    proc_macro_error,
};
use quote::quote;
use rapyd::component::{Walk, WalkIterator};
use syn::{
    parse_macro_input, parse_quote, punctuated::Punctuated, token::Comma, Block, ExprClosure,
    LitInt, Stmt,
};
use syn_rsx::{Node, ParserConfig};

#[proc_macro_error]
#[proc_macro]
pub fn arr(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut closure = parse_macro_input!(item as ExprClosure);

    let mut ts: Punctuated<Block, Comma> = Punctuated::new();

    let size = closure.inputs.pop().unwrap().into_value();
    let size: LitInt = parse_quote!(#size);

    let input_var = closure.inputs.pop().unwrap().into_value();

    for i in 0_usize..size.base10_parse().unwrap() {
        let mut block: Block = parse_quote!({});
        block
            .stmts
            .push(parse_quote!(const #input_var: usize = #i;));

        let iife = Stmt::Expr(parse_quote!((#closure)()));
        block.stmts.push(iife);

        ts.push(block);
    }

    quote!([#ts]).into()
}

#[proc_macro_error]
#[proc_macro]
pub fn html(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let html = syn_rsx::parse_with_config(item, ParserConfig::default());
    let mut template: Option<TokenStream> = None;
    match html {
        Ok(html) => {
            let walks = process_html(html);
            let walks_len = walks.len();
            let mut walks_ts: Punctuated<TokenStream, Comma> = Punctuated::new();
            for walk in walks {
                match walk {
                    Walk::In(n) => walks_ts.push(parse_quote!(::rapyd::component::Walk::In(#n))),
                    Walk::Out(n) => walks_ts.push(parse_quote!(::rapyd::component::Walk::Out(#n))),
                    Walk::Over(n) => {
                        walks_ts.push(parse_quote!(::rapyd::component::Walk::Over(#n)))
                    }

                    Walk::Replace => walks_ts.push(parse_quote!(::rapyd::component::Walk::Replace)),
                    Walk::Event(event_type) => {
                        walks_ts.push(parse_quote!(::rapyd::component::Walk::Event(#event_type)))
                    }
                }
            }
            template = Some(
                parse_quote!(const WALKS: [::rapyd::component::Walk; #walks_len] = [#walks_ts];),
            );
        }
        Err(err) => emit_error!(err),
    }

    abort_if_dirty();
    template.expect("WTF happened?").into()
}

#[proc_macro_error]
#[proc_macro]
pub fn html_iter(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let html = syn_rsx::parse_with_config(item, ParserConfig::default());
    let mut template: Option<TokenStream> = None;
    match html {
        Ok(html) => {
            let walks = WalkIteratorTest::new(html.into_iter());
            let mut walks_len: usize = 0;
            let mut walks_ts: Punctuated<TokenStream, Comma> = Punctuated::new();
            for walk in walks {
                walks_len += 1;
                match walk {
                    Walk::In(n) => walks_ts.push(parse_quote!(::rapyd::component::Walk::In(#n))),
                    Walk::Out(n) => walks_ts.push(parse_quote!(::rapyd::component::Walk::Out(#n))),
                    Walk::Over(n) => {
                        walks_ts.push(parse_quote!(::rapyd::component::Walk::Over(#n)))
                    }

                    Walk::Replace => walks_ts.push(parse_quote!(::rapyd::component::Walk::Replace)),
                    Walk::Event(event_type) => {
                        walks_ts.push(parse_quote!(::rapyd::component::Walk::Event(#event_type)))
                    }
                }
            }
            template = Some(
                parse_quote!(const WALKS: [::rapyd::component::Walk; #walks_len] = [#walks_ts];),
            );
        }
        Err(err) => emit_error!(err),
    }

    abort_if_dirty();
    template.expect("WTF happened?").into()
}

fn process_html_inner(mut node_iterator: Peekable<impl Iterator<Item = Node>>) -> WalkIterator {
    let mut walks = WalkIterator::new();
    while let Some(node) = node_iterator.next() {
        match node {
            Node::Element(e) => {
                walks.push_in(1);
                for walk in Vec::<Walk>::from(process_html_inner(e.children.into_iter().peekable()))
                {
                    match walk {
                        Walk::In(n) => walks.push_in(n),
                        Walk::Out(n) => walks.push_out(n),
                        Walk::Over(n) => walks.push_over(n),
                        Walk::Replace => walks.push_replace(),
                        Walk::Event(_) => todo!(),
                    }
                }
                walks.push_out(1);
                if node_iterator.peek().is_some() {
                    walks.push_over(1);
                }
            }
            Node::Attribute(_) => unreachable!(),
            Node::Text(_) => walks.push_over(1),
            Node::Comment(_) => walks.push_over(1),

            Node::Doctype(_) => todo!(),
            Node::Block(_) => {
                walks.push_replace();
                walks.push_over(1);
            }
            Node::Fragment(_) => todo!(),
        }
    }

    walks.into()
}
fn process_html(html: Vec<Node>) -> Vec<Walk> {
    process_html_inner(html.into_iter().peekable()).into()
}

struct WalkIteratorTest<NodeIter: Iterator<Item = Node>> {
    nodes: Peekable<NodeIter>,
    queue: VecDeque<Walk>,
    // TODO: try to remove this recurssion. This should be possible without heap allocations
    recursive_iterator: Option<Box<WalkIteratorTest<Box<dyn Iterator<Item = Node>>>>>,
}

impl<NodeIter: Iterator<Item = Node>> WalkIteratorTest<NodeIter> {
    pub fn new(iter: NodeIter) -> Self {
        Self {
            nodes: iter.peekable(),
            queue: VecDeque::new(),
            recursive_iterator: None,
        }
    }
    fn new_with_queue(iter: NodeIter, queue: VecDeque<Walk>) -> Self {
        Self {
            nodes: iter.peekable(),
            queue,
            recursive_iterator: None,
        }
    }
    fn block_invariant(&mut self) {
        self.queue.push_front(Walk::Over(1));
    }

    fn queue_push_back(&mut self, walk: Walk) {
        match &mut self.recursive_iterator {
            Some(recursive_iterator) => recursive_iterator.queue_push_back(walk),
            None => {
                self.queue.push_back(walk);
            }
        }
    }
}

impl<NodeIter: Iterator<Item = Node>> Iterator for WalkIteratorTest<NodeIter> {
    type Item = Walk;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = None;
        loop {
            if let Some(recursive_iterator) = &mut self.recursive_iterator {
                let mut should_break = false;
                while let Some(walk) = recursive_iterator.next() {
                    if let Err(_) = Walk::combine(&mut next, walk) {
                        // Thi cant append to queue drectly
                        recursive_iterator.queue_push_back(walk);
                        should_break = true;
                        break;
                        // return next;
                    };
                }
                if should_break {
                    break;
                }
            }
            {
                let mut should_break = false;
                while let Some(walk) = self.queue.pop_back() {
                    if let Err(_) = Walk::combine(&mut next, walk) {
                        self.queue.push_back(walk);
                        should_break = true;
                        break;
                        // return next;
                    };
                }
                if should_break {
                    break;
                }
            }
            let node = match self.nodes.next() {
                Some(node) => node,
                None => return next,
            };
            match node {
                Node::Element(e) => {
                    if let Err(_) = Walk::combine_in(&mut next, 1) {
                        self.queue.push_front(Walk::In(1));
                        self.recursive_iterator = Some(Box::new(WalkIteratorTest::new_with_queue(
                            Box::new(e.children.into_iter()),
                            mem::take(&mut self.queue),
                        )));

                        self.queue.push_front(Walk::Out(1));
                        // TODO: THINK ABOUT THIS OVER 1
                        self.queue.push_front(Walk::Over(1));
                        break;
                    };
                    self.recursive_iterator = Some(Box::new(WalkIteratorTest::new_with_queue(
                        Box::new(e.children.into_iter()),
                        mem::take(&mut self.queue),
                    )));
                    self.queue.push_front(Walk::Out(1));
                    // TODO: THINK ABOUT THIS OVER 1
                    self.queue.push_front(Walk::Over(1));
                }
                Node::Attribute(_) => unreachable!(),
                Node::Text(_) => {
                    if let Err(_) = Walk::combine_over(&mut next, 1) {
                        self.queue.push_front(Walk::Over(1));
                        break;
                    };
                }
                Node::Comment(_) => {
                    if let Err(_) = Walk::combine_over(&mut next, 1) {
                        self.queue.push_front(Walk::Over(1));
                        break;
                    };
                }

                Node::Doctype(_) => todo!(),
                Node::Block(_) => {
                    if let Err(_) = Walk::combine_replace(&mut next) {
                        self.queue.push_front(Walk::Replace);
                        self.block_invariant();
                        break;
                    };
                    self.block_invariant();
                }
                Node::Fragment(_) => todo!(),
            }
        }
        next
    }
}
