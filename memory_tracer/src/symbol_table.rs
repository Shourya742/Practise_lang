use std::{collections::HashMap, sync::atomic::AtomicUsize};

use crate::demangle;


pub struct Symbol {
    allocated: AtomicUsize,
    count: AtomicUsize
}

impl Symbol {
    pub fn allocated(&self) -> usize {
        self.allocated.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn count(&self) -> usize {
        self.count.load(std::sync::atomic::Ordering::Relaxed)
    }
}

pub struct SymbolTable {
    modules: &'static[&'static str],
    symbols: HashMap<&'static str, Symbol>
}

impl SymbolTable {
    pub(crate) fn new(size: usize, modules: &'static[&'static str]) -> Self {
        Self {
            modules,
            symbols: HashMap::with_capacity(size)
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&&'static str, &Symbol)> {
        self.symbols.iter()
    }

    pub fn get(&self, name: &'static str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    pub(crate) fn alloc(&mut self, bytes: usize) {
        let name = demangle::get_demangled_symbol(self.modules);

        if !self.symbols.contains_key(&name) {
            self.insert(name);
        }

        let symbol = self.symbols.get_mut(name).expect("Symbol should be there");

        symbol.allocated.fetch_add(bytes, std::sync::atomic::Ordering::Relaxed);
        symbol.count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    pub(crate) fn dealloc(&mut self, bytes: usize) {
        let name = demangle::get_demangled_symbol(self.modules);
        if let Some(symbol) = self.symbols.get_mut(name) {
            symbol.allocated.fetch_sub(bytes, std::sync::atomic::Ordering::Relaxed);
            symbol.count.fetch_sub(bytes, std::sync::atomic::Ordering::Relaxed);
        }
    }

    fn insert(&mut self, name: &'static str) {
        self.symbols.insert(name, Symbol { allocated: AtomicUsize::new(0), count: AtomicUsize::new(0) });
    }

}