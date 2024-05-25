impl Menu {
  pub fn builder<'a>() -> MenuBuilder<'a> {
    MenuBuilder {
      text    	: "Menu",
      disabled	: false,
      popup   	: false,
      parent  	: None
    }
  }
  /// Return true if the control user can interact with the control, return false otherwise
  pub fn enabled(&self) -> bool {
    if self.handle.blank() { panic!("{}", NOT_BOUND); }
    let (parent_handle, handle) = match self.handle {
      ControlHandle::Menu(parent, menu)	=> (parent, menu),
      ControlHandle::PopMenu(_, _)     	=> { return true; },
      _                                	=> panic!("{}", BAD_HANDLE)};
    unsafe { mh::is_menu_enabled(parent_handle, handle) }
  }
  /// Enable or disable the control A popup menu cannot be disabled
  pub fn set_enabled(&self, v: bool) {
    if self.handle.blank() { panic!("{}", NOT_BOUND); }
    let (parent_handle, handle) = match self.handle {
      ControlHandle::Menu(parent, menu)	=> (parent, menu),
      ControlHandle::PopMenu(_, _)     	=> { return; },
      _                                	=> panic!("{}", BAD_HANDLE)};
    unsafe { mh::enable_menu(parent_handle, handle, v); }
  }
  /// Show a popup menu as the selected position. Do nothing for menubar menu.
  pub fn popup_with_flags(&self, x: i32, y: i32, flags: PopupMenuFlags) {
    use winapi::um::winuser::{TrackPopupMenu, SetForegroundWindow};
    use winapi::ctypes::c_int;
    if self.handle.blank() { panic!("Menu is not bound"); }
    let (parent_handle, handle) = match self.handle.pop_hmenu() {
      Some(v) => v,
      None => { return; }
    };
    unsafe { 
      SetForegroundWindow(parent_handle);
      TrackPopupMenu(
        handle,
        flags.bits(),
        x as c_int,
        y as c_int,
        0,
        parent_handle,
        ptr::null()
      );
    }
  }
  /// Show a popup menu as the selected position. Do nothing for menubar menu.
  pub fn popup(&self, x: i32, y: i32) {
    self.popup_with_flags(x, y, PopupMenuFlags::empty())
  }
}
