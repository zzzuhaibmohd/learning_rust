use std::io;

#[derive(Debug, Clone, PartialEq)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub is_completed: bool,
}

pub struct TodoManager {
    todos: Vec<Todo>,
    next_id: u64,
}

impl TodoManager {
    pub fn new() -> Self {
        TodoManager {
            todos: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_todo(&mut self, title: String) -> Result<u64, String> {
        if title.trim().is_empty() {
            return Err("Todo cannot be empty!".to_string());
        }

        let id = self.next_id;
        self.todos.push(Todo {
            id,
            title: title.trim().to_string(),
            is_completed: false,
        });
        self.next_id += 1;
        Ok(id)
    }

    pub fn get_todos(&self) -> &Vec<Todo> {
        &self.todos
    }

    pub fn toggle_todo(&mut self, id: u64) -> Result<bool, String> {
        for todo in &mut self.todos {
            if todo.id == id {
                todo.is_completed = !todo.is_completed;
                return Ok(todo.is_completed);
            }
        }
        Err(format!("Todo with ID {} not found!", id))
    }

    pub fn get_todo_by_id(&self, id: u64) -> Option<&Todo> {
        self.todos.iter().find(|todo| todo.id == id)
    }

    pub fn get_completed_count(&self) -> usize {
        self.todos.iter().filter(|todo| todo.is_completed).count()
    }

    pub fn get_pending_count(&self) -> usize {
        self.todos.iter().filter(|todo| !todo.is_completed).count()
    }

    pub fn is_empty(&self) -> bool {
        self.todos.is_empty()
    }
}

fn main() {
    let mut todo_manager = TodoManager::new();

    loop {
        main_menu();

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read an input");

        let user_option: u32 = user_input.trim().parse().expect("Please type a number!");

        match user_option {
            1 => {
                view_todos(&todo_manager);
            }
            2 => {
                add_todo(&mut todo_manager);
            }
            3 => {
                edit_todo(&mut todo_manager);
            }
            4 => {
                break;
            }
            _ => {
                println!("Invalid option");
            }
        }
    }
}

fn main_menu() {
    println!("\n╔══════════════════════════════════════════╗");
    println!("║           🦀 RUST TODO APP 🦀            ║");
    println!("╚══════════════════════════════════════════╝");
    println!();
    println!("📋 1. List all todos");
    println!("➕ 2. Add a new todo");
    println!("✏️  3. Edit a todo");
    println!("🚪 4. Exit");
    println!();
    print!("Enter your choice (1-4): ");
}

fn view_todos(todo_manager: &TodoManager) {
    if todo_manager.is_empty() {
        println!("\n📭 No todos found. Add a new todo to get started!");
        return;
    }

    println!("\n╔══════════════════════════════════════════╗");
    println!("║                📋 TODOS 📋                ║");
    println!("╚══════════════════════════════════════════╝");
    println!();
    
    for todo in todo_manager.get_todos() {
        let status_icon = if todo.is_completed { "✅" } else { "⏳" };
        let status_text = if todo.is_completed { "COMPLETED" } else { "PENDING" };
        
        println!("┌─ Task #{} ──────────────────────────────────┐", todo.id);
        println!("│ Title: {:<35} │", todo.title);
        println!("│ Status: {} {:<30} │", status_icon, status_text);
        println!("└─────────────────────────────────────────────┘");
        println!();
    }
    
    println!("Total tasks: {} | Completed: {} | Pending: {}", 
        todo_manager.get_todos().len(),
        todo_manager.get_completed_count(),
        todo_manager.get_pending_count()
    );
}

fn add_todo(todo_manager: &mut TodoManager) {
    println!("\n➕ ADD NEW TODO");
    println!("══════════════════════════════════════════");
    print!("Enter a new todo: ");
    
    let mut new_todo = String::new();
    io::stdin()
        .read_line(&mut new_todo)
        .expect("Failed to read input");
    
    match todo_manager.add_todo(new_todo) {
        Ok(id) => println!("✅ Todo added successfully! (ID: {})", id),
        Err(e) => println!("❌ {}", e),
    }
}

fn edit_todo(todo_manager: &mut TodoManager) {
    if todo_manager.is_empty() {
        println!("\n❌ No todos to edit. Add some todos first!");
        return;
    }
    
    println!("\n✏️  EDIT TODO");
    println!("══════════════════════════════════════════");
    print!("Enter the ID of the todo to edit: ");
    
    let mut id = String::new();
    io::stdin()
        .read_line(&mut id)
        .expect("Failed to read input");
    
    match id.trim().parse::<u64>() {
        Ok(id) => {
            match todo_manager.toggle_todo(id) {
                Ok(is_completed) => {
                    let status = if is_completed { "completed" } else { "pending" };
                    println!("✅ Todo #{} status changed to {}!", id, status);
                }
                Err(e) => println!("❌ {}", e),
            }
        }
        Err(_) => {
            println!("❌ Invalid ID format. Please enter a number.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_manager_new() {
        let manager = TodoManager::new();
        assert!(manager.is_empty());
        assert_eq!(manager.get_completed_count(), 0);
        assert_eq!(manager.get_pending_count(), 0);
    }

    #[test]
    fn test_add_todo_success() {
        let mut manager = TodoManager::new();
        
        let result = manager.add_todo("Buy groceries".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
        
        assert!(!manager.is_empty());
        assert_eq!(manager.get_todos().len(), 1);
        assert_eq!(manager.get_pending_count(), 1);
        assert_eq!(manager.get_completed_count(), 0);
        
        let todo = manager.get_todo_by_id(1).unwrap();
        assert_eq!(todo.title, "Buy groceries");
        assert!(!todo.is_completed);
    }

    #[test]
    fn test_add_todo_empty() {
        let mut manager = TodoManager::new();
        
        let result = manager.add_todo("".to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Todo cannot be empty!");
        
        let result = manager.add_todo("   ".to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Todo cannot be empty!");
    }

    #[test]
    fn test_add_todo_with_whitespace() {
        let mut manager = TodoManager::new();
        
        let result = manager.add_todo("  Buy groceries  ".to_string());
        assert!(result.is_ok());
        
        let todo = manager.get_todo_by_id(1).unwrap();
        assert_eq!(todo.title, "Buy groceries"); // Should be trimmed
    }

    #[test]
    fn test_add_multiple_todos() {
        let mut manager = TodoManager::new();
        
        assert_eq!(manager.add_todo("First todo".to_string()).unwrap(), 1);
        assert_eq!(manager.add_todo("Second todo".to_string()).unwrap(), 2);
        assert_eq!(manager.add_todo("Third todo".to_string()).unwrap(), 3);
        
        assert_eq!(manager.get_todos().len(), 3);
        assert_eq!(manager.get_pending_count(), 3);
        assert_eq!(manager.get_completed_count(), 0);
    }

    #[test]
    fn test_toggle_todo_success() {
        let mut manager = TodoManager::new();
        manager.add_todo("Test todo".to_string()).unwrap();
        
        // Initially pending
        let todo = manager.get_todo_by_id(1).unwrap();
        assert!(!todo.is_completed);
        
        // Toggle to completed
        let result = manager.toggle_todo(1);
        assert!(result.is_ok());
        assert!(result.unwrap()); // Should be completed now
        
        let todo = manager.get_todo_by_id(1).unwrap();
        assert!(todo.is_completed);
        assert_eq!(manager.get_completed_count(), 1);
        assert_eq!(manager.get_pending_count(), 0);
        
        // Toggle back to pending
        let result = manager.toggle_todo(1);
        assert!(result.is_ok());
        assert!(!result.unwrap()); // Should be pending now
        
        let todo = manager.get_todo_by_id(1).unwrap();
        assert!(!todo.is_completed);
        assert_eq!(manager.get_completed_count(), 0);
        assert_eq!(manager.get_pending_count(), 1);
    }

    #[test]
    fn test_toggle_todo_not_found() {
        let mut manager = TodoManager::new();
        
        let result = manager.toggle_todo(999);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Todo with ID 999 not found!");
    }

    #[test]
    fn test_get_todo_by_id() {
        let mut manager = TodoManager::new();
        manager.add_todo("Test todo".to_string()).unwrap();
        
        let todo = manager.get_todo_by_id(1);
        assert!(todo.is_some());
        assert_eq!(todo.unwrap().title, "Test todo");
        
        let todo = manager.get_todo_by_id(999);
        assert!(todo.is_none());
    }

    #[test]
    fn test_todo_struct() {
        let todo = Todo {
            id: 1,
            title: "Test".to_string(),
            is_completed: false,
        };
        
        assert_eq!(todo.id, 1);
        assert_eq!(todo.title, "Test");
        assert!(!todo.is_completed);
    }

    #[test]
    fn test_todo_equality() {
        let todo1 = Todo {
            id: 1,
            title: "Test".to_string(),
            is_completed: false,
        };
        
        let todo2 = Todo {
            id: 1,
            title: "Test".to_string(),
            is_completed: false,
        };
        
        let todo3 = Todo {
            id: 2,
            title: "Test".to_string(),
            is_completed: false,
        };
        
        assert_eq!(todo1, todo2);
        assert_ne!(todo1, todo3);
    }

    #[test]
    fn test_complex_scenario() {
        let mut manager = TodoManager::new();
        
        // Add multiple todos
        manager.add_todo("Task 1".to_string()).unwrap();
        manager.add_todo("Task 2".to_string()).unwrap();
        manager.add_todo("Task 3".to_string()).unwrap();
        
        // Verify initial state
        assert_eq!(manager.get_todos().len(), 3);
        assert_eq!(manager.get_pending_count(), 3);
        assert_eq!(manager.get_completed_count(), 0);
        
        // Complete first and third tasks
        manager.toggle_todo(1).unwrap();
        manager.toggle_todo(3).unwrap();
        
        // Verify final state
        assert_eq!(manager.get_todos().len(), 3);
        assert_eq!(manager.get_pending_count(), 1);
        assert_eq!(manager.get_completed_count(), 2);
        
        // Verify individual todo states
        assert!(manager.get_todo_by_id(1).unwrap().is_completed);
        assert!(!manager.get_todo_by_id(2).unwrap().is_completed);
        assert!(manager.get_todo_by_id(3).unwrap().is_completed);
    }
}
