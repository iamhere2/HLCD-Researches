use std::collections::HashMap;
use super::interfaces;

pub struct Metadata {
    name: &'static str,
    children: &'static HashMap<&'static str, &'static Metadata>,
    consumes: &'static HashMap<&'static str, &'static interfaces::Metadata>,
    provides: &'static HashMap<&'static str, &'static interfaces::Metadata>,
}

pub trait Component {

    fn new() -> Self;
    fn metadata() -> &'static Metadata;

}