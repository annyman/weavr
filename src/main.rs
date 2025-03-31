use note_compiler::parse_note;

fn main() -> Result<(), anyhow::Error> {
    let text = "note1 { contents go here and i can link to @note2 and add tag of #some_tag }";
    let note = parse_note(text)?;
    println!("{:?}", note);
    Ok(())
}