# Starting Out With Java Chapter 10 Programming Challenge 8 Implementation in Rust

I am attempting to teach myself about elements of software construction, Rust, and package development by attempting to answer programming challenge 9 in chapter 10 of the 3rd edition of "Starting Out With Java: From Control Structures through Data Structures". I will be using information and concepts from the [MIT Course 6.031 website](https://web.mit.edu/6.031/www/sp22/), [The Cargo Book](https://doc.rust-lang.org/cargo/index.html), and the more general [The Rust Programming Language](https://doc.rust-lang.org/stable/book/) book.

## Original Problem Statement (With Small Edits)
Design an abstract class named **BankAccount** to hold the following data for a bank account:

- Balance
- Number of deposits during the current calendar month
- Number of withdrawald during the current calendar month
- Annual interest rate
- Monthly service charges

The class should have the following methods:

- ```Constructor```
- ```deposit```: A method that accepts an argument for the amount to deposit. The method should add the argument to the account balance. It should also increment the variable holding the number of deposits.
- ```withdraw```: A method that accepts an argument for the amount of the withdrawal. The method should subtract the argument from the balance. It should also increment the variable holding the number of withdrawals.
- ```calcInterest```: A method that updates the balance by calculating the monthly interest earned by the account, and adding this interest to the balance. This is performed by the following formulas:
    - $Monthly\space Interest\space Rate = Annual\space Interest\space Rate \div 12$
    - $Monthly\space Interest = Balance \times Monthly\space Interest\space Rate$
    - $Balance = Balance\space + Monthly\space Interest$
- ```monthlyProcess```: A method that subtracts the monthly service charges from the balance, calls the *calcInterest*, and then sets the number of withdrawals and deposits as well as service charges all to zero

Next, design a ```SavingsAccount``` class that extends the ```BankAccount``` class. The ```SavingsAccount``` class should have a status field to represent an active or inactive account. If the balance of a ```SavingsAccount``` falls below $25, it becomes inactive. No more withdrawals may be made until the balance is raised above $25, at which time the account becomes active again. The savings account class should have the following methods:

- ```withdraw```: A method that determines whether the account is inactive before a withdrawal is made, and no withdrawal should be allowed if the account is inactive. A withdrawal is then made by calling the superclass version of the method.
- ```deposit```: A method that determines whether the account is inactive before a deposit is made. If the account is inactive and the deposit brings the balance above $25, the account becomes active again. A deposit is then made by calling the superclass version of the method.
- ```monthlyProcess```: Before the superclass method is called, this method checks the number of withdrawals. If the number of withdrawals for the month is more than 4, a service charge of $1 for each withdrawal above 4 is added to the superclass field that holds the monthly service charges. If, after the service charge is taken, the balance falls below $25, the account becomes inactive

## Modifications
According to [this](https://doc.rust-lang.org/stable/book/ch17-01-what-is-oo.html) section of the online [Rust Programming Language Book](https://doc.rust-lang.org/stable/book/title-page.html), Rust does not implement inheritance *in sensu stricto*. Instead, Rust uses structs and traits along with methods defined in impl blocks. This is an issue given that the whole point of Chapter 10 is learning about inheritance and polymorphism. Instead of a ```SavingsAccount``` class that inherits from a ```BankAccount``` class, with the main state information being whether or not the ```SavingsAccount``` instance is in good standing, I will attempt to implement one or both of two design patterns outlined [here](https://doc.rust-lang.org/stable/book/ch17-03-oo-design-patterns.html). One implements an object and shared behavior by storing a state and another only using type checking.