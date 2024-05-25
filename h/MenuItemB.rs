pub struct MenuItemBuilder<'a> {
  text    	: &'a str,
  disabled	: bool,
  check   	: bool,
  parent  	: Option<ControlHandle>}
impl<'a> MenuItemBuilder<'a> {
  pub fn text                          	(mut self, text    	:&'a  str     	) -> MenuItemBuilder<'a> {self.text = text;self}
  pub fn disabled                      	(mut self, disabled	:     bool    	) -> MenuItemBuilder<'a> {self.disabled = disabled;self}
  pub fn check                         	(mut self, check   	:     bool    	) -> MenuItemBuilder<'a> {self.check = check;self}
  pub fn parent<C: Into<ControlHandle>>	(mut self, p       	:     C       	) -> MenuItemBuilder<'a> {self.parent = Some(p.into());self}
  pub fn build                         	(    self, item    	:&mut MenuItem	) -> Result<(), NwgError> {
    if self.parent.is_none() {return Err(NwgError::no_parent_menu());}
    item.handle = ControlBase::build_hmenu()
      .text(self.text)
      .item(true)

      .parent(self.parent.unwrap())
      .build()?;
    if self.disabled {item.set_enabled(false);}
    if self.check    {item.set_checked(true);}
    Ok(())}
}
