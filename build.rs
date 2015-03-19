fn main() {
	// Static linking (doesn't really work for some reason)
	// println!("cargo:rustc-link-search=native={}", "C:/Program Files (x86)/Windows Kits/8.1/Lib/winv6.3/um/x64/");
	// println!("cargo:rustc-link-search=native={}", "./");

	// Dynamic linking. Mostly a temporary solution, would prefer static linking
	println!("cargo:rustc-link-search=native={}", "C:/Windows/System32/");
	// println!("cargo:rustc-link-search=native={}", "./");
}