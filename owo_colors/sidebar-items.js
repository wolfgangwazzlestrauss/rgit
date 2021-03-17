initSidebarItems({"enum":[["AnsiColors","Available standard ANSI colors for use with `OwoColorize::color` or `OwoColorize::on_color`"],["DynColors","An enum describing runtime-configurable colors which can be displayed using `FgDynColorDisplay` or `BgDynColorDisplay`, allowing for multiple types of colors to be used at runtime."],["Effect",""],["XtermColors","Available Xterm colors for use with `OwoColorize::color` or `OwoColorize::on_color`"]],"fn":[["style","Helper to create [`Style`]s more ergonomically"]],"mod":[["colored","Module for drop-in `colored` support to aid in porting code from `colored` to owo-colors."],["colors","Color types for used for being generic over the color"],["styles","Different display styles (strikethrough, bold, etc.)"]],"struct":[["BgColorDisplay","Transparent wrapper around a type which implements all the formatters the wrapped type does, with the addition of changing the background color. Recommended to be constructed using `OwoColorize`."],["BgDynColorDisplay","Wrapper around a type which implements all the formatters the wrapped type does, with the addition of changing the background color. Is not recommended unless compile-time coloring is not an option."],["FgColorDisplay","Transparent wrapper around a type which implements all the formatters the wrapped type does, with the addition of changing the foreground color. Recommended to be constructed using `OwoColorize`."],["FgDynColorDisplay","Wrapper around a type which implements all the formatters the wrapped type does, with the addition of changing the foreground color. Is not recommended unless compile-time coloring is not an option."],["ParseColorError",""],["Rgb","Available RGB colors for use with `OwoColorize::color` or `OwoColorize::on_color`"],["Style",""],["Styled",""]],"trait":[["Color","A trait for describing a type which can be used with `FgColorDisplay` or `BgCBgColorDisplay`"],["DynColor","A trait describing a runtime-configurable color which can displayed using `FgDynColorDisplay` or `BgDynColorDisplay`. If your color will be known at compile time it is recommended you avoid this."],["OwoColorize","Extension trait for colorizing a type which implements any std formatter (`Display`, `Debug`, `UpperHex`, etc.)"]]});