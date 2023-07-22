use druid::Widget;
use druid::widget::Label;
use druid::WindowDesc;
use druid::AppLauncher;
use im::Vector;
use druid::{Data, Lens};
use crate::data::TodoState;
use ui::ui_builder;

pub fn ui_builder() -> impl Widget <TodoState>{
    Label::new("Hello world")
}

fn main(){
    mod ui{}
    mod data{}
    let main_window = WindowDesc::new(ui_builder())
        .title ("ToDo App")
        .window_size((500., 500.));
    AppLauncher::with_window(main_window)
        .launch(data::TodoState::default())
        .expect("Faild to start")
}

#[derive(Clone, Data, Lens, Default)]
pub struct TodoState{
    pub todos: Vector<TodoItem>,
}

#[derive(Clone, Data, Lens, Default)]
pub struct TodoItem{
    pub checked: bool,
    pub text: String,
}
