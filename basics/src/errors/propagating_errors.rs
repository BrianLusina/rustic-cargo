use std::fs::File;
use std::io::Read;

/**
Demonstration of propagating errors to calling code. This returns the error of failing to read a
username from a given file and let the calling code decide what to do with the returned result
**/
fn read_username_from_file(file_name: &str) -> Result<String, std::io::Error> {
    let username_file_result = File::open(file_name);

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => {Ok(username)},
        Err(e) => {Err(e)}
    }
}

/**
This is the same as the read_username_from_file function, but instead uses the ? operator to handle
the errors. When the File::open(file_name)?; method is called, the error is returned if it fails,
And if there is no error, the username_file variable will contain the file handle. The same case for
when reading the contents of the file handle to the username variable. If that fails the ? operator
ensures that the error is returned. If there is no error Ok(username) is returned. The ? operator
returns the result as if a return is used as in the read_username_from_file function.
**/
fn read_username_from_file_v2(file_name: &str) -> Result<String, std::io::Error> {
    let mut username_file = File::open(file_name)?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
