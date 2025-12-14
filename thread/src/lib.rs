pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub struct AverageCollection {
    list: Vec<i32>,
    average: f64,
}
impl AverageCollection {
    pub fn new() -> Self {
        Self {
            list: Vec::new(),
            average: 0.0,
        }
    }
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }
    pub fn average(&self) -> f64 {
        self.average
    }
    pub fn update_average(&mut self) {
        let total = self.list.iter().sum::<i32>();
        self.average = total as f64 / self.list.len() as f64;
    }
}
impl Default for AverageCollection {
    fn default() -> Self {
        Self::new()
    }
}

pub trait DrawComponent {
    fn draw(&self);
}

pub struct Page {
    pub components: Vec<Box<dyn DrawComponent>>,
}
impl Page {
    pub fn render(&self) {
        for component in self.components.iter() {
            component.as_ref().draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl DrawComponent for Button {
    fn draw(&self) {
        println!(
            "Drawing button: {} {} {}",
            self.width, self.height, self.label
        );
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}
impl DrawComponent for SelectBox {
    fn draw(&self) {
        println!(
            "Drawing select box: {} {} {}",
            self.width,
            self.height,
            self.options.join(", ")
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
