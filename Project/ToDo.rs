use druid::Widget;
use druid::widget::Label;
use druid::WindowDesc;
use druid::AppLauncher;
use im::Vector;
use druid::{Data, Lens};
use crate::data::TodoState as OtherTodoState;
use ui::ui_builder as other_ui_builder;
use druid::widget::Flex;
use druid::widget::Button;
use druid::widget::TextBox;
use druid::WidgetExt;
use druid::Env;
use druid::widget::Checkbox;
use druid::widget::List;

pub mod ui {
    pub use super::*;
}

pub mod data {
    pub use super::*;
}

pub fn ui_builder() -> impl Widget <TodoState>{
    let header = Flex::row()
    .with_flex_child(TextBox::new()
        .lens(TodoState::new_text)
        .expand_width(), 1.)
    .with_child(
        Button::new("->")
        .on_click(|_ctx, data: &mut TodoState, _env|{
            if data.new_text.trim() !=""{
                let text = data.new_text.clone();
                data.new_text = "". to_string();
                data.todos.push_front(TodoItem{ checked: false, text })
            }
        }));

    let todos = List::new(|| {
        Flex::row()
        .with_child(Checkbox::new("").lens(TodoItem::checked))
        .with_child(Label::new(|data: &TodoItem, _: &Env| data.text.clone()))
    }).lens(TodoState::todos);

    Flex::column().with_child(header).with_child(todos)
}

fn main(){
    pub mod ui{}
    pub mod data{}
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