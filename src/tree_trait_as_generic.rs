trait Weapon { 
    fn reload(&self);
}

trait CharacterInput { 
    fn reload(&self);
}

trait ReloadOrder<W: Weapon> : CharacterInput { 
    fn reload(&self, weapon: &W){
        CharacterInput::reload(self);
        weapon.reload();
    }
}

























pub fn run (){ 

}