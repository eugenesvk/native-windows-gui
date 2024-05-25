pub struct MenuBuilder<'a> {
  text    	: &'a str,
  disabled	: bool,
  popup   	: bool,
  parent  	: Option<ControlHandle>}
impl<'a> MenuBuilder<'a> {
  pub fn text                          	(mut self, text    	:&'a  str 	) -> MenuBuilder<'a> {self.text = text;self}
  pub fn disabled                      	(mut self, disabled	:     bool	) -> MenuBuilder<'a> {self.disabled = disabled;self}
  pub fn popup                         	(mut self, popup   	:     bool	) -> MenuBuilder<'a> {self.popup = popup;self}
  pub fn parent<C: Into<ControlHandle>>	(mut self, p       	:     C   	) -> MenuBuilder<'a> {self.parent = Some(p.into());self}
  pub fn build                         	(    self, menu    	:&mut Menu	) -> Result<(), NwgError> {
    if self.parent.is_none() {return Err(NwgError::no_parent_menu());}
    menu.handle = ControlBase::build_hmenu()
      .text(self.text)
      .item(false)
      .popup(self.popup)
      .parent(self.parent.unwrap())
      .build()?;
    if self.disabled {menu.set_enabled(false)}

    Ok(())}
}
