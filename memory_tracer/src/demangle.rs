use backtrace::BacktraceSymbol;



const UNKNOWN: &str = "<unknown>";


const IGNORE_LIST: &[&str] = &[
    "memory_tracer::demangle::get_demangled_symbol",
    "memory_tracer::SymbolTable::alloc",
    "memory_tracer::SymbolTable::dealloc",
];


pub fn get_demangled_symbol(modules: &[&str]) -> &'static str {
    let backtrace = backtrace::Backtrace::new();
    let Some(caller) = get_symbol_from_backtrace(&backtrace, modules) else {
        return UNKNOWN
    };

    symbol_name(caller).unwrap_or(UNKNOWN)
}


fn get_symbol_from_backtrace<'a>(backtrace: &'a backtrace::Backtrace, modules: &[&str]) -> Option<&'a BacktraceSymbol> {
    let frame = backtrace.frames().iter().enumerate().find_map(|(index, frame)| {
        let symbol = frame.symbols().first()?;
        let name = symbol.name().map(|name| format!("{name}"))?;

        if IGNORE_LIST.iter().any(|ignore| name.starts_with(*ignore)) {
            return None;
        }
        if modules.iter().any(|module| name.starts_with(*module)) {
            return Some(index);
        }

        None
    })?;

    backtrace.frames().get(frame).and_then(|frame| frame.symbols().first())
}


fn symbol_name(symbol: &BacktraceSymbol) -> Option<&'static str> {
    let name_str = symbol.name().map(|name| format!("{name}"))?;

    let name_string = if let Some(pos) = name_str.rfind("::") {
        &name_str[..pos]
    } else {
        &name_str
    };

    Some(Box::leak(name_string.to_string().into_boxed_str()))
}