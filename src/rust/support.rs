use std::vec::Vec;
use tokens::*;

pub fn slice_of(string: &str, range: (usize, usize)) -> String
{
    let native_slice = &string[range.0..range.1];
    let mut slice = String::new();
    slice.push_str(native_slice);
    slice
}

/// Identifies the end index of a nestable body given the open and close symbols for the body
/// For instance, an arithmetic expression can have nested parenthesis groups (e.g. (1 + (2*(2+1))) * (2 + 5) )
/// In the example, this method would identify the start and stop as these (> and < respectively):
///     >(1 + (2*(2+1)))< * (2 + 5)
/// If the start_index was specified as the parenthesis surrounding 2 + 5, then this method would select:
///     (1 + (2*(2+1))) * >(2 + 5)<
///
/// The start index passed to this method should be the index AFTER the first open symbol
/// @return the index of the token that closes the specified body or None if the block is not closed or nested properly
pub fn identify_body_bounds(tokens: &Vec<Token>, start: usize, symbols: (&str, &str)) -> Option<usize>
{
    let (open, close) = symbols;
    let mut open_braces = 1;

    for(index, token) in tokens[start..].iter().enumerate()
    {
        if token.value == open
        {
            open_braces += 1;
        }
        else if token.value == close
        {
            open_braces -= 1;
            if open_braces == 0
            {
                return Some(index + start);
            }
        }
    }

    return None;
}

/// Identifies the index of the next token matching the specified symbol
/// @return the index of the specified token (after the specified start index) or None if no such symbol exists
pub fn find_next(tokens: &Vec<Token>, start: usize, symbol: &str) -> Option<usize>
{
    for (index, token) in tokens[start..].iter().enumerate()
    {
        if token.value == symbol { return Some(index + start); }
    }

    return None;
}

pub fn alter_string(original_string: &str, delimiter: &str, join_symbol: &str, string_added: &str, remove: bool) -> String
{
    let mut string: Vec<&str> = original_string.split_terminator(delimiter).collect();
    let mut length = string.len();
    let mut result: String;

    if remove
    {
        string.remove(length - 1);
    }
    else
    {
        string.push(string_added);
    }
    
    result = string.connect(join_symbol);
    return result;
}
