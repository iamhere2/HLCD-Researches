use super::components::Component;

pub trait ConsoleApp: Component {

}


pub fn run<T>()
    where T: ConsoleApp {

        let app = T::new();


}