use std::{alloc::{GlobalAlloc, Layout, System}, cell::Cell, sync::{atomic::AtomicUsize, Mutex, MutexGuard, OnceLock, PoisonError}};

use crate::symbol_table::SymbolTable;

mod symbol_table;
mod demangle;

thread_local! {
    static IN_ALLOC: Cell<bool> = const { Cell::new(false) }
}

const DEFAULT_SYMBOL_TABLE_SIZE: usize = 1024;

static SYMBOL_TABLE: OnceLock<Mutex<SymbolTable>> = OnceLock::new();

pub struct MemoryTrackerAllocator {
    allocated: AtomicUsize
}

pub fn init_symbol_table(modules: &'static[&'static str]) {
    SYMBOL_TABLE.get_or_init(|| Mutex::new(SymbolTable::new(DEFAULT_SYMBOL_TABLE_SIZE, modules)));
}

pub fn with_symbol_table<F, R>(f: F) -> Result<R,  PoisonError<std::sync::MutexGuard<'static, SymbolTable>>> where F: FnOnce(&SymbolTable) -> R {
    IN_ALLOC.with(|cell| cell.set(true));


    let lock = match SYMBOL_TABLE.get().expect("Symbol table not initialized").lock() {
        Ok(lock) => lock,
        Err(poisoned) => {
            IN_ALLOC.with(|cell| cell.set(false));
            return Err(poisoned);
        }
    };

    let res = Ok(f(&lock));

    IN_ALLOC.with(|cell| cell.set(false));

    res    
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AllocOp {
    Alloc,
    Dealloc
}


impl MemoryTrackerAllocator {
    pub const fn init() -> Self {
        MemoryTrackerAllocator { allocated: AtomicUsize::new(0) }
    }

    pub fn allocated(&self) -> usize {
        self.allocated.load(std::sync::atomic::Ordering::Relaxed)
    }

    fn is_external_allocation(&self) -> bool {
        !IN_ALLOC.get()
    }

    fn enter_alloc(&self) {
        IN_ALLOC.with(|cell| cell.set(true));
    }

    fn exit_alloc(&self) {
        IN_ALLOC.with(|cell| cell.set(false));
    }

    fn trace_allocation(&self, layout: Layout, table: Option<&mut MutexGuard<SymbolTable>>) {
        self.allocated.fetch_add(layout.size(), std::sync::atomic::Ordering::Relaxed);
        if let Some(table) = table {
            table.alloc(layout.size());
        }
    }

    fn trace_deallocation(&self, layout: Layout, table: Option<&mut MutexGuard<SymbolTable>>) {
        self.allocated.fetch_sub(layout.size(), std::sync::atomic::Ordering::Relaxed);
        if let Some(table) = table {
            table.dealloc(layout.size());
        }
    }

    fn trace(&self, layout: Layout, op: AllocOp) {
        let mut lock = SYMBOL_TABLE.get().and_then(|table| table.lock().ok());

        self.enter_alloc();
        match op {
            AllocOp::Alloc => self.trace_allocation(layout, lock.as_mut()),
            AllocOp::Dealloc => self.trace_deallocation(layout, lock.as_mut()),
        }
        self.exit_alloc();
        drop(lock)
    }
}


unsafe impl GlobalAlloc for MemoryTrackerAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe {System.alloc(layout)};
        if !ptr.is_null() && self.is_external_allocation() {
            self.trace(layout, AllocOp::Alloc);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if !ptr.is_null() && self.is_external_allocation() {
            self.trace(layout, AllocOp::Dealloc);
        }

        unsafe  { System.dealloc(ptr, layout) };
    }
}