pub fn pop_newline(line:&mut String){
    if line.ends_with('\n') {
        line.pop();
    }
}