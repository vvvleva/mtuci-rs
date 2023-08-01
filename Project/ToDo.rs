use directories::BaseDirs;
use druid::MenuItem;
use druid::UnitPoint;
use druid::Widget;
use druid::widget::Label;
use druid::WindowDesc;
use druid::AppLauncher;
use im::Vector;
use druid::{Data, Lens};
use druid::widget::Flex;
use druid::widget::Button;
use druid::widget::TextBox;
use druid::WidgetExt;
use druid::Env;
use druid::widget::Checkbox;
use druid::widget::List;
use druid::EventCtx;
use druid::menu;
use crate::menu::Menu;
use druid::Point;
use druid::widget::ZStack;
use druid::widget::Padding;
// use std::default;
use std::{path::Path, fs};
use serde::{Serialize, Deserialize};

pub mod ui {
    pub use super::*;
}

pub mod data {
    pub use super::*;
}

pub mod server {
    pub use super::*;
}

pub fn ui_builder() -> impl Widget <TodoState>{      // Функция возвращает всё, что реализует виджет Druid
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
        }))
        .with_child(Saver {});

    let todos = List::new(|| {
        Flex::row()
            .with_child(Checkbox::new("").lens(TodoItem::checked))
            .with_default_spacer()
            .with_child(Label::new(|data: &TodoItem, _: &Env| data.text.clone()))
            .with_flex_spacer(0.1)
            .with_child(Button::new("...").on_click(|ctx: &mut EventCtx, data: &mut TodoItem, _env|{
                let data_clone = data.clone();
                let menu: Menu<TodoState> = Menu::empty()
                    .entry(MenuItem::new("Remove").on_activate(move|_, main_data: &mut TodoState, _| {
                        let location = main_data.todos.index_of(&data_clone).unwrap();
                        main_data.todos.remove(location);
            }));
               
                ctx.show_context_menu(menu, Point::new(0., 0.))
            }))
    
    }).lens(TodoState::todos).scroll().vertical();   // Возможность пролистывать вниз/вверх

    let clear_complete = Button::new("Clear Completed")
        .on_click(|_, data: &mut TodoState, _| {
            data.todos.retain(|item| !item.checked)
        });

    ZStack::new(Flex::column().with_child(header).with_flex_child(todos, 1.)).with_aligned_child(Padding::new(5., clear_complete), UnitPoint::BOTTOM_RIGHT)  // Создание столбца из задач (оформление задач в столбец)
}

fn main(){
    pub mod ui{}
    pub mod data{}
    pub mod saver{}
    let main_window = WindowDesc::new(ui_builder())  // Создадим главное окно
        .title ("ToDo App")
        .window_size((500., 600.));
let stored = read_stored();
let default_state = TodoState {
    todos: Vector::from(stored.tasks),
    ..Default::default()
};

    AppLauncher::with_window(main_window)  // Происходит запуск приложения
        .launch(default_state)
        .expect("Faild to start")  // Обработка ошибки
}


#[derive(Clone, Data, Lens, Default)]
pub struct TodoState{
    pub todos: Vector<TodoItem>,
    pub new_text: String,
}

pub struct Saver;
impl Widget<TodoState> for Saver {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &druid::Event, _data: &mut TodoState, _env: &Env) {
    }
    fn lifecycle(&mut self, _ctx: &mut druid::LifeCycleCtx, _event: &druid::LifeCycle, _data: &TodoState, _env: &Env) {
    }
    fn update(&mut self, _ctx: &mut druid::UpdateCtx, old_data: &TodoState, data: &TodoState, _env: &Env) {
        if data.todos!= old_data.todos {
            if let Some(base_dirs) = BaseDirs::new() {
                let config = format!("{}/{}", base_dirs.config_dir().to_str().unwrap(), "Todo.json");
                let config_path = Path::new(&config);
                let tasks = TaskData {tasks: data.todos.clone().into_iter().collect()};
                fs::write(config_path, serde_json::to_string(&tasks).unwrap()).expect("Config path does not fully exist");
            }
        }
    }
    fn layout(&mut self, _ctx: &mut druid::LayoutCtx, _bc: &druid::BoxConstraints, _data: &TodoState, _env: &Env) -> druid::Size {
        druid::Size {width: 0., height: 0.}
    }
    fn paint(&mut self, _ctx: &mut druid::PaintCtx, _data: &TodoState, _env: &Env) {
    }
}

#[derive(Serialize, Deserialize)]
pub struct TaskData{
    pub tasks: Vec<TodoItem>,
}

pub fn read_stored() -> TaskData {
    if let Some(base_dirs) = BaseDirs::new() {
        let config = format!("{}/{}", base_dirs.config_dir().to_str().unwrap(), "Todo.json");
        let config_path = Path::new(&config);
        let data = match fs::read_to_string(config_path) {
            Ok(a) => a,
            Err(_) => return  TaskData {tasks: Vec::new()},
        };
        match serde_json::from_str(&data) {
            Ok(a) => a,
            Err(e) => {
                eprint!("The save data is corrupted or no longer in the format it should be in \nError {}", e);
                return TaskData{tasks: Vec::new()};
            },
        }
    } else {
        return TaskData {tasks: Vec::new()};
    }
}

#[derive(Clone, Data, Lens, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodoItem{
    pub checked: bool,
    pub text: String,
}
