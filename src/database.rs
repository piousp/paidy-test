use super::item;
use std::collections::HashMap;
use uuid::Uuid;

pub struct Restaurant {
    tables: HashMap<u8, Vec<item::Item>>,
}

#[derive(Debug)]
pub enum Action{
    Inserted,
    Deleted,
    Updated,
    Data(Vec<item::Item>)
}

impl Restaurant {

    pub fn new() -> Restaurant {
        Restaurant {
            tables: HashMap::new()
        }
    }

    pub fn items_from_table(&self, table: u8) -> Result<Action, String> {
        self.map(table, |items| Action::Data(items.to_vec()))
    }

    pub fn update_item(&mut self, table: u8, item_id: Uuid, item: item::Item ) -> Result<Action, String> {
        let result = self.map(table, |old_items| -> Vec<item::Item> {
            old_items
                .iter()
                .map(|x| (if x._id == item_id { &item } else { x }).clone() )
                .collect()
        });
        result.and_then(|items| if items.len() == 0 {
            Err("Nothing to update".to_string())
        } else {
            self.replace_whole(table, items, Action::Updated)
        } )
    }

    pub fn add_items(&mut self, table: u8, items: Vec<item::Item>) -> Result<Action, String> {
        let result = self.map(table, |old_items| [old_items, &items[..]].concat());
        result.or(Ok(items)).and_then(|items_to_add| self.replace_whole(table, items_to_add, Action::Inserted))
    }

    pub fn remove_item(&mut self, table: u8, item_id: Uuid) -> Result<Action, String> {
        let result = self.map(table, |items| -> Vec<item::Item> {
            items.iter()
                .filter(|x| x._id != item_id)
                .map(|x| x.clone())
                .collect()
        });
        result.and_then(|items| self.replace_whole(table, items, Action::Deleted))
    }

    fn map<T>(&self, table: u8, op: impl Fn(&Vec<item::Item>) -> T) -> Result<T, String> {
        match self.tables.get(&table) {
            Some(items) => Ok(op(items)),
            None => Err("Table is empty".to_string()),
        }
    }

    fn replace_whole(&mut self, table: u8, items: Vec<item::Item>, action: Action) -> Result<Action, String>{
        self.tables.insert(table, items);
        Ok(action)
    }
}

#[cfg(test)]
mod tests {
    use super::item::Item;
    use super::*;

    #[test]
    fn items_from_table_empty() {
        let res = Restaurant::new();
        let expected = Some("Table is empty".to_string());
        assert_eq!(expected, res.items_from_table(1).err());
    }

    #[test]
    fn items_from_table_non_empty() {
        let mut res = Restaurant::new();
        let item = Item::new("Test 1");
        res.tables.insert(1, vec![item.clone()]);
        let expected = vec![item];
        match res.items_from_table(1) {
            Ok(Action::Data(items)) => assert_eq!(items, expected),
            Err(_) => assert!(false),
            _a => panic!("This isn't right")
        }
    }

    #[test]
    fn add_items_to_new_table() {
        let mut res = Restaurant::new();
        let item = Item::new("Test 1");
        match res.add_items(1, vec![item.clone()]) {
            Ok(Action::Inserted) => assert!(true),
            Err(_) => assert!(false),
            _a => panic!("This isn't right")
        };
        let expected = vec![item];
        match res.items_from_table(1) {
            Ok(Action::Data(items)) => assert_eq!(items, expected),
            Err(_) => assert!(false),
            _a => panic!("This isn't right")
        }
    }

    #[test]
    fn add_items_to_empty_table() {
        let mut res = Restaurant::new();
        let item = Item::new("Test 1");
        res.tables.insert(1, Vec::new());
        match res.add_items(1, vec![item.clone()]) {
            Ok(Action::Inserted) => assert!(true),
            Err(err) => assert!(false, err),
            _a => panic!("This isn't right")
        };
        let expected = vec![item];
        let t: u8 = 1;
        println!("Current contents {:?}", res.tables.get(&t));
        match res.items_from_table(1) {
            Ok(Action::Data(items)) => assert_eq!(items, expected),
            Err(_) => assert!(false),
            _a => panic!("This isn't right")
        }
    }

    #[test]
    fn add_items_to_non_empty_table() {
        let mut res = Restaurant::new();
        let item = Item::new("Test 1");
        let item2 = Item::new("Test 2");
        res.tables.insert(1,vec![item.clone()]);
        match res.add_items(1, vec![item2.clone()]) {
            Ok(Action::Inserted) => assert!(true),
            Err(_) => assert!(false),
            _a => panic!("This isn't right")
        };
        println!("Everything's good so far");
        let expected = vec![item, item2];
        match res.items_from_table(1) {
            Ok(Action::Data(items)) => assert_eq!(items, expected),
            Err(_) => assert!(false),
            _a => panic!("This isn't right")
        }
    }

    #[test]
    fn remove_items_from_new_table() {
        let mut res = Restaurant::new();
        match res.remove_item(1, Item::new("Test 1")._id){
            Ok(Action::Deleted) => assert!(false),
            Err(_) => assert!(true),
            _a => panic!("This isn't right")
        };
        match res.items_from_table(1) {
            Err(_) => assert!(true),
            Ok(_) => assert!(false)
        }

    }

    #[test]
    fn remove_items_from_empty_table() {
        let mut res = Restaurant::new();
        res.tables.insert(1, Vec::new());
        match res.remove_item(1, Item::new("Test 1")._id){
            Ok(Action::Deleted) => assert!(true),
            Err(_) => assert!(false),
            _a => panic!("This isn't right")
        };
        println!("Everthing's fine so far");
        println!("Items from table 1 {:?}", res.items_from_table(1));
        match res.items_from_table(1) {
            Err(_) => assert!(false),
            Ok(_) => assert!(true)
        }
    }

    #[test]
    fn remove_items_from_non_empty_table_to_empty() {
        let mut res = Restaurant::new();
        let item = Item::new("Test 1");
        res.tables.insert(1,vec![item.clone()]);
        match res.remove_item(1, item._id) {
            Ok(Action::Deleted) => assert!(true),
            Err(_) => assert!(false),
            _a => panic!("This isn't right")
        };
        match res.items_from_table(1) {
            Err(_) => assert!(false),
            Ok(_) => assert!(true)
        }
    }

    #[test]
    fn remove_items_from_non_empty_table_begining() {
        let mut res = Restaurant::new();
        let item = Item::new("Test 1");
        let item2 = Item::new("Test 2");
        let item3 = Item::new("Test 3");
        res.tables.insert(1,vec![item.clone(), item2.clone(), item3.clone()]);
        println!("Current items: {:?}", res.items_from_table(1));
        let expected = vec![item2, item3];
        match res.remove_item(1, item._id) {
            Ok(Action::Deleted) => assert!(true),
            Err(_) => assert!(false),
            _a => panic!("This isn't right")
        };
        match res.items_from_table(1) {
            Err(_) => assert!(false),
            Ok(Action::Data(items)) => assert_eq!(expected, items),
            _a => panic!("This isn't right")
        }
    }

    #[test]
    fn remove_items_from_non_empty_table_middle() {
        let mut res = Restaurant::new();
        let item = Item::new("Test 1");
        let item2 = Item::new("Test 2");
        let item3 = Item::new("Test 3");
        res.tables.insert(1,vec![item.clone(), item2.clone(), item3.clone()]);
        let expected = vec![item, item3];
        match res.remove_item(1, item2._id) {
            Ok(Action::Deleted) => assert!(true),
            Err(_) => assert!(false),
            _a => panic!("This isn't right")
        };
        match res.items_from_table(1) {
            Err(_) => assert!(false),
            Ok(Action::Data(items)) => assert_eq!(expected, items),
            _a => panic!("This isn't right")
        }
    }

    #[test]
    fn remove_items_from_non_empty_table_ending() {
        let mut res = Restaurant::new();
        let item = Item::new("Test 1");
        let item2 = Item::new("Test 2");
        let item3 = Item::new("Test 3");
        res.tables.insert(1,vec![item.clone(), item2.clone(), item3.clone()]);
        let expected = vec![item, item2];
        match res.remove_item(1, item3._id) {
            Ok(Action::Deleted) => assert!(true),
            Err(_) => assert!(false),
            _a => panic!("This isn't right")
        };
        match res.items_from_table(1) {
            Err(_) => assert!(false),
            Ok(Action::Data(items)) => assert_eq!(expected, items),
            _a => panic!("This isn't right")
        }
    }

    #[test]
    fn update_item_from_new_table() {
        let mut res = Restaurant::new();
        match res.update_item(1, Item::new("Test 1")._id, Item::new("Test A")){
            Ok(Action::Updated) => assert!(false),
            Err(_) => assert!(true),
            _a => panic!("This isn't right")
        };
        match res.items_from_table(1) {
            Err(_) => assert!(true),
            Ok(_) => assert!(false)
        }
    }

    #[test]
    fn update_item_from_empty_table() {
        let mut res = Restaurant::new();
        res.tables.insert(1, Vec::new());
        match res.update_item(1, Item::new("Test 1")._id, Item::new("Test A")){
            Ok(Action::Updated) => assert!(false),
            Err(_) => assert!(true),
            _a => panic!("This isn't right")
        };
        println!("Everthing's fine so far");
        println!("Items from table 1 {:?}", res.items_from_table(1));
        match res.items_from_table(1) {
            Err(_) => assert!(false),
            Ok(_) => assert!(true)
        }
    }

    #[test]
    fn update_item_from_non_empty_table_to_empty() {
        let mut res = Restaurant::new();
        let item = Item::new("Test 1");
        let updated = Item{
            name: "Test A".to_string(),
            cook_time: item.cook_time,
            _id: item._id
        };
        let expected = vec![updated.clone()];
        res.tables.insert(1,vec![item.clone()]);
        match res.update_item(1, item._id, updated) {
            Ok(Action::Updated) => assert!(true),
            Err(_) => assert!(false),
            _a => panic!("This isn't right")
        };
        match res.items_from_table(1) {
            Err(_) => assert!(false),
            Ok(Action::Data(items)) => assert_eq!(expected, items),
            _a => panic!("This isn't right")
        }
    }

    #[test]
    fn update_item_from_non_empty_table_begining() {
        let mut res = Restaurant::new();
        let item = Item::new("Test 1");
        let item2 = Item::new("Test 2");
        let item3 = Item::new("Test 3");
        let updated = Item{
            name: "Test A".to_string(),
            cook_time: item.cook_time,
            _id: item._id
        };
        res.tables.insert(1,vec![item.clone(), item2.clone(), item3.clone()]);
        println!("Current items: {:?}", res.items_from_table(1));
        let expected = vec![updated.clone(), item2, item3];
        match res.update_item(1, item._id, updated) {
            Ok(Action::Updated) => assert!(true),
            Err(_) => assert!(false),
            _a => panic!("This isn't right")
        };
        match res.items_from_table(1) {
            Err(_) => assert!(false),
            Ok(Action::Data(items)) => assert_eq!(expected, items),
            _a => panic!("This isn't right")
        }
    }

    #[test]
    fn update_item_from_non_empty_table_middle() {
        let mut res = Restaurant::new();
        let item = Item::new("Test 1");
        let item2 = Item::new("Test 2");
        let item3 = Item::new("Test 3");
        let updated = Item{
            name: "Test B".to_string(),
            cook_time: item2.cook_time,
            _id: item2._id
        };
        res.tables.insert(1,vec![item.clone(), item2.clone(), item3.clone()]);
        let expected = vec![item, updated.clone(), item3];
        match res.update_item(1, item2._id, updated) {
            Ok(Action::Updated) => assert!(true),
            Err(_) => assert!(false),
            _a => panic!("This isn't right")
        };
        match res.items_from_table(1) {
            Err(_) => assert!(false),
            Ok(Action::Data(items)) => assert_eq!(expected, items),
            _a => panic!("This isn't right")
        }
    }

    #[test]
    fn update_items_from_non_empty_table_ending() {
        let mut res = Restaurant::new();
        let item = Item::new("Test 1");
        let item2 = Item::new("Test 2");
        let item3 = Item::new("Test 3");
        let updated = Item{
            name: "Test C".to_string(),
            cook_time: item3.cook_time,
            _id: item3._id
        };
        res.tables.insert(1,vec![item.clone(), item2.clone(), item3.clone()]);
        let expected = vec![item, item2, updated.clone()];
        match res.update_item(1, item3._id, updated) {
            Ok(Action::Updated) => assert!(true),
            Err(_) => assert!(false),
            _a => panic!("This isn't right")
        };
        match res.items_from_table(1) {
            Err(_) => assert!(false),
            Ok(Action::Data(items)) => assert_eq!(expected, items),
            _a => panic!("This isn't right")
        }
    }
}
