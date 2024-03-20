use printpdf::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::fs;

#[derive(Serialize, Deserialize)]
struct Element {
    r#type: String,
    element_id: String,
    text: String,
    metadata: Metadata,
}

#[derive(Serialize, Deserialize)]
struct Metadata {
    languages: Vec<String>,
    page_number: i32,
    filename: String,
    filetype: String,
    #[serde(default)]
    parent_id: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the JSON file
    let file = File::open("federal_rules.json")?;
    let reader = BufReader::new(file);

    // Parse the JSON data into a vector of Element structs
    let elements: Vec<Element> = serde_json::from_reader(reader)?;

    // Create a new PDF document
    let (doc, page1, layer1) = PdfDocument::new("PDF Document", Mm(210.0), Mm(297.0), "Layer 1");

    // Load the font
    let font = doc.add_external_font(File::open("assets/fonts/Roboto-Medium.ttf").unwrap()).unwrap();
    //let font2 = doc.add_external_font(File::open("assets/fonts/Roboto-Bold.ttf").unwrap()).unwrap();
    // Set the initial position and font size
    let mut current_layer = doc.get_page(page1).get_layer(layer1);
    let mut y_position = Mm(297.0 - 20.0);
    let font_size = 12.0;
    // Read the JSON file
    let json = fs::read_to_string("federal_rules.json").expect("Failed to read file");

    // Convert JSON to Markdown
    let markdown = json_to_markdown(&json);

    // Print the Markdown output and write file
   // println!("{}", markdown);
    fs::write("federal_rules.md", markdown).expect("Failed to write file");
    // Iterate over the elements and add them to the PDF
    for element in elements {
        // Create a text section
        let text = format!("{}: {}", element.r#type, element.text);

        // Set the font and font size
        current_layer.use_text(text.as_str(), font_size, Mm(20.0), y_position, &font);

        // Move to the next line
        y_position -= Mm(font_size + 5.0);

        // If reached the bottom of the page, create a new page and layer
        if y_position < Mm(20.0) {
            let (page2, layer2) = doc.add_page(Mm(210.0), Mm(297.0), "Page 2");
            current_layer = doc.get_page(page2).get_layer(layer2);
            y_position = Mm(297.0 - 20.0);
        }
    }

    // Save the PDF
    doc.save(&mut BufWriter::new(File::create("federal_rules.pdf")?))?;

    Ok(())
}

fn json_to_markdown(json: &str) -> String {
    let elements: Vec<Element> = serde_json::from_str(json).unwrap();

    let mut markdown = String::new();
    for element in elements {
        match element.r#type.as_str() {
            "Title" => {
                markdown.push_str(&format!("# {}\n\n", element.text));
            }
            "NarrativeText" => {
                markdown.push_str(&format!("{}\n\n", element.text));
            }
            _ => {}
        }
    }

    markdown
}