use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Write;
use std::path::PathBuf;

use crate::models::xml_struct::blocks::title::TitleStyle;
use crate::models::xml_struct::blocks::{
    ActionDescription, CharacterDialog, NameCharacter, Parenthetical, ScenarioBlock, SceneHeading,
    Title,
};
use crate::scenario::page::PageDto;

pub fn generate_block_xml<W: Write>(
    writer: &mut Writer<W>,
    block: &ScenarioBlock,
) -> Result<(), String> {
    match block {
        ScenarioBlock::SceneHeading(sh) => {
            // <scene_heading>
            writer
                .write_event(Event::Start((BytesStart::new("scene_heading"))))
                .map_err(|e| e.to_string())?;

            // <place>sh.place</place>
            writer
                .write_event(Event::Start(BytesStart::new("place")))
                .map_err(|e| e.to_string())?;
            writer
                .write_event(Event::Text(BytesText::new(&sh.place)))
                .map_err(|e| e.to_string())?;
            writer
                .write_event(Event::End(BytesEnd::new("place")))
                .map_err(|e| e.to_string())?;

            // <title>sh.title</title>
            writer
                .write_event(Event::Start(BytesStart::new("title")))
                .map_err(|e| e.to_string())?;
            writer
                .write_event(Event::Text(BytesText::new(&sh.title)))
                .map_err(|e| e.to_string())?;
            writer
                .write_event(Event::End(BytesEnd::new("title")))
                .map_err(|e| e.to_string())?;

            // <time>sh.time</time>
            writer
                .write_event(Event::Start(BytesStart::new("time")))
                .map_err(|e| e.to_string())?;
            writer
                .write_event(Event::Text(BytesText::new(&sh.time)))
                .map_err(|e| e.to_string())?;
            writer
                .write_event(Event::End(BytesEnd::new("time")))
                .map_err(|e| e.to_string())?;

            // </scene_heading>
            writer
                .write_event(Event::End(BytesEnd::new("scene_heading")))
                .map_err(|e| e.to_string())?;
        }

        ScenarioBlock::ActionDescription(ad) => {
            // <action_description> ad.text </action_description>
            writer
                .write_event(Event::Start(BytesStart::new("action_description")))
                .map_err(|e| e.to_string())?;
            writer
                .write_event(Event::Text(BytesText::new(&ad.text)))
                .map_err(|e| e.to_string())?;
            writer
                .write_event(Event::End(BytesEnd::new("action_description")))
                .map_err(|e| e.to_string())?;
        }

        ScenarioBlock::NameCharacter(nc) => {
            // <name_character> nc.text </name_character>
            writer
                .write_event(Event::Start(BytesStart::new("name_character")))
                .map_err(|e| e.to_string())?;
            writer
                .write_event(Event::Text(BytesText::new(&nc.text)))
                .map_err(|e| e.to_string())?;
            writer
                .write_event(Event::End(BytesEnd::new("name_character")))
                .map_err(|e| e.to_string())?;
        }

        ScenarioBlock::Dialog(d) => {
            // <dialog> d.text </dialog>
            writer
                .write_event(Event::Start(BytesStart::new("dialog")))
                .map_err(|e| e.to_string())?;
            writer
                .write_event(Event::Text(BytesText::new(&d.text)))
                .map_err(|e| e.to_string())?;
            writer
                .write_event(Event::End(BytesEnd::new("dialog")))
                .map_err(|e| e.to_string())?;
        }
        ScenarioBlock::Parenthetical(p) => {
            // <parenthetical> p.text </parenthetical>
            writer
                .write_event(Event::Start(BytesStart::new("parenthetical")))
                .map_err(|e| e.to_string())?;
            writer
                .write_event(Event::Text(BytesText::new(&p.text)))
                .map_err(|e| e.to_string())?;
            writer
                .write_event(Event::End(BytesEnd::new("parenthetical")))
                .map_err(|e| e.to_string())?;
        }
        ScenarioBlock::Title(t) => {
            // <title>
            let mut title_tag = BytesStart::new("title");

            if let Some(ref actual_style) = t.style {
                let style_str = match actual_style {
                    TitleStyle::Center => "center",
                    TitleStyle::Left => "left",
                    TitleStyle::Bottom => "bottom",
                };

                // <title style="center or left or bottom">
                title_tag.push_attribute(("style", style_str));
            }

            writer
                .write_event(Event::Start(title_tag))
                .map_err(|e| e.to_string())?;

            // <title style="style_str"> t.text
            writer
                .write_event(Event::Text(BytesText::new(&t.text)))
                .map_err(|e| e.to_string())?;

            // <title style="style_str"> t.text </title>
            writer
                .write_event(Event::End(BytesEnd::new("title")))
                .map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

pub fn parse_block_xml(path: &PathBuf) -> Result<Vec<PageDto>, String> {
    let mut reader = Reader::from_file(path).map_err(|e| e.to_string())?;
    reader.config_mut().trim_text(true);

    let mut pages: Vec<PageDto> = Vec::new();
    let mut current_blocks: Vec<ScenarioBlock> = Vec::new();
    let mut current_page_number: u32 = 1;

    let mut buf = Vec::new();
    let mut current_tag = String::new();
    let mut current_style: Option<crate::models::xml_struct::blocks::title::TitleStyle> = None;

    let mut place_text = String::new();
    let mut title_text = String::new();
    let mut time_text = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let tag_name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                current_tag = tag_name.clone();

                if tag_name == "page" {
                    if let Some(attr) = e
                        .attributes()
                        .flatten()
                        .find(|a| a.key.as_ref() == b"number")
                    {
                        let num_str = String::from_utf8_lossy(&attr.value).into_owned();
                        current_page_number = num_str.parse().unwrap_or(1);
                    }
                    current_blocks.clear();
                } else if tag_name == "title" {
                    current_style = None;
                    if let Some(attr) = e
                        .attributes()
                        .flatten()
                        .find(|a| a.key.as_ref() == b"style")
                    {
                        let style_str = String::from_utf8_lossy(&attr.value);
                        current_style = match style_str.as_ref() {
                            "center" => {
                                Some(crate::models::xml_struct::blocks::title::TitleStyle::Center)
                            }
                            "left" => {
                                Some(crate::models::xml_struct::blocks::title::TitleStyle::Left)
                            }
                            "bottom" => {
                                Some(crate::models::xml_struct::blocks::title::TitleStyle::Bottom)
                            }
                            _ => None,
                        };
                    }
                }
            }

            Ok(Event::Text(e)) => {
                let text = e
                    .decode()
                    .map_err(|e| format!("Ошибка декодирования текста: {}", e))?
                    .into_owned();

                if text.is_empty() {
                    buf.clear();
                    continue;
                }

                match current_tag.as_str() {
                    "action_description" => {
                        current_blocks.push(ScenarioBlock::ActionDescription(ActionDescription {
                            block_type:
                                crate::models::xml_struct::block_type::BlockType::ActionDescription,
                            text,
                        }));
                    }
                    "name_character" => {
                        current_blocks.push(ScenarioBlock::NameCharacter(NameCharacter {
                            block_type:
                                crate::models::xml_struct::block_type::BlockType::NameCharacter,
                            text,
                        }));
                    }
                    "parenthetical" => {
                        current_blocks.push(ScenarioBlock::Parenthetical(Parenthetical {
                            block_type:
                                crate::models::xml_struct::block_type::BlockType::Parenthetical,
                            text,
                        }));
                    }
                    "dialog" => {
                        current_blocks.push(ScenarioBlock::Dialog(CharacterDialog {
                            block_type: crate::models::xml_struct::block_type::BlockType::Dialog,
                            text,
                        }));
                    }
                    "title" => {
                        current_blocks.push(ScenarioBlock::Title(Title {
                            block_type: crate::models::xml_struct::block_type::BlockType::Title,
                            text,
                            style: current_style.clone(),
                            is_spoken: None,
                        }));
                    }
                    "place" => place_text = text,
                    "title_scene" => title_text = text,
                    "time" => time_text = text,
                    _ => {}
                }
            }

            Ok(Event::End(e)) => {
                let tag_name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                if tag_name == "page" {
                    pages.push(PageDto {
                        number: current_page_number,
                        blocks: current_blocks.clone(),
                    });
                } else if tag_name == "scene_heading" {
                    current_blocks.push(ScenarioBlock::SceneHeading(SceneHeading {
                        block_type: crate::models::xml_struct::block_type::BlockType::SceneHeading,
                        place: place_text.clone(),
                        title: title_text.clone(),
                        time: time_text.clone(),
                    }));

                    place_text.clear();
                    title_text.clear();
                    time_text.clear();
                }
                current_tag.clear();
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("Ошибка парсинга XML: {}", e)),
            _ => {}
        }
        buf.clear();
    }
    Ok(pages)
}
