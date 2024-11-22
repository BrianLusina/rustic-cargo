use std::fs::File;
use std::io::ErrorKind;

/**
This is a demonstration of error handling using the Result<T, E> enum, which allows branching between
Ok and Err variants to handle recoverable errors such as when opening a file. In this case, the file
handle is returned and passed on to the file variable if it exists, otherwise, the error is handled
in the Err(error) branch where the error.kind() method of the Error allows us to further branch to
handle the different kinds of errors that come with io operations, in this case, if the file is not
found, then we attempt to create the file, if that can not happen, we call the panic! macro and exit
the program, if we can create the file, we return the file handle to the file variable.

Note the use of many match statements though!
**/
fn opening_file(file_name: &str) {
    let file_result = File::open(file_name);

    let file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_name) {
                Ok(file) => file,
                Err(error) => panic!("Could not create file: {:?}", error),
            }
            other_error => panic!("Opening file failed: {:?}", error),
        }
    };
}

/**
This is the same as the opening_file function, with the only difference in how the error handling
is done. In this case, the unwrap_or_else method is used to handle the error. This makes the code
more readable and easier to maintain.
**/
fn open_file(file_name: &str) {
    let file_result = File::open(file_name).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(&file_name).unwrap_or_else(|error| {
                panic!("Could not create file: {:?}", error);
            })
        } else {
            panic!("Opening file failed: {:?}", error);
        }
    });
}

/**
This uses the unwrap() method which will call panic for us
**/
fn open_file_v2(file_name: &str) {
    // This uses the unwrap() method which will call panic for us
    let file_result = File::open(file_name).unwrap();
    // expect here allows passing in an error message to display in case an error is encountered
    let file_result_2 = File::open(file_name).expect("{file_name} should be included");
}