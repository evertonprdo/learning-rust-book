/*  https://rust-book.cs.brown.edu/ch08-03-hash-maps.html#summary
    Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company;
    for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a
    department or all people in the company by department, sorted alphabetically.
*/

use interface::Interface;

// Practicing a bit of chapter 7 too: https://rust-book.cs.brown.edu/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
mod interface;
mod stdin_helper;

fn main() {
    Interface::new().run();
}
