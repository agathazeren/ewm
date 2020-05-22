# Ewm
_(Editing with Wasm Macros)_

Ewm is a work in progress text editor with heavy inspiration from Emacs. 

The goal is to have a very small core editor, with not even the ability to render text, and then to build everything up from small, highly composable modules, encoded in wasm. While these wasm modules are inherently less hackable than elisp, extensibility is achived (or attempt to) through composablilty, and the small size of each module. 
