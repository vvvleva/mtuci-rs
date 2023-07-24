use druid::Widget;
use druid::widget::Label;
use druid::WindowDesc;
use druid::AppLauncher;
use im::Vector;
use druid::{Data, Lens};
use crate::data::TodoState;
use ui::ui_builder;
use druid::widget::Flex;
use druid::widget::Button;
use druid::widget::TextBox;
use druid::WidgetExt;

pub fn ui_builder() -> impl Widget <TodoState>{
    let header = Flex::row()
    .with_child(TextBox::new().lens(TodoState::new_text))
    .with_child(Button::new("->"));
    Flex::column().with_child(header)
}

fn main(){
    mod ui{}
    mod data{}
    let main_window = WindowDesc::new(ui_builder())
        .title ("ToDo App")
        .window_size((500., 500.));
    AppLauncher::with_window(main_window)
        .launch(TodoState::default())
        .expect("Faild to start")
}


#[derive(Clone, Data, Lens, Default)]
pub struct TodoState{
    pub todos: Vector<TodoItem>,
    pub new_text: String,
}

#[derive(Clone, Data, Lens, Default)]
pub struct TodoItem{
    pub checked: bool,
    pub text: String,
}
