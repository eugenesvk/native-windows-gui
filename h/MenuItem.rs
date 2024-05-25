impl MenuItem {
  pub fn builder<'a>() -> MenuItemBuilder<'a> {
    MenuItemBuilder {
      text    	: "Menu Item",
      disabled	: false,
      check   	: false,
      parent  	: None
    }
  }
  /// Return true if the control user can interact with the control, return false otherwise
  pub fn enabled(&self) -> bool {
    if self.handle.blank() { panic!("{}", NOT_BOUND); }
    let (parent_handle, id) = self.handle.hmenu_item().expect(BAD_HANDLE);



    unsafe { mh::is_menuitem_enabled(parent_handle, None, Some(id)) }
  }
  /// Enable or disable the control
  pub fn set_enabled(&self, v: bool) {
    if self.handle.blank() { panic!("{}", NOT_BOUND); }
    let (parent_handle, id) = self.handle.hmenu_item().expect(BAD_HANDLE);



    unsafe { mh::enable_menuitem(parent_handle, None, Some(id), v); }
  }
  /// Sets the check state of a menu item
  pub fn set_checked(&self, check: bool) {
    if self.handle.blank() { panic!("{}", NOT_BOUND); }
    let (parent_handle, id) = self.handle.hmenu_item().expect(BAD_HANDLE);
    unsafe { mh::check_menu_item(parent_handle, id, check); }
  }
  /// Returns the check state of a menu item
  pub fn checked(&self) -> bool {
    if self.handle.blank() { panic!("{}", NOT_BOUND); }
    let (parent_handle, id) = self.handle.hmenu_item().expect(BAD_HANDLE);
    unsafe { mh::menu_item_checked(parent_handle, id) }
  }
}
