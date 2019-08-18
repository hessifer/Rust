use std::collections::LinkedList;

fn main() {
   let mut ll = LinkedList::new();

   // add data to our LinkedList
   ll.push_back(1);
   ll.push_back(2);
   ll.push_back(3);
   ll.push_back(4);
   ll.push_back(3);

   // loop over LinkedList and keep a count for all items that exist
   // and odd number of times in the LinkedList
   for item in ll {
       println!("{}", item);
   }
}
