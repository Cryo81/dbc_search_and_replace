#![allow(warnings)]
use wow_dbc::DbcTable;
use wow_dbc::vanilla_tables;
use std::fs::File;
use std::env;

const WOW_ERROR_STRINGS: &[u8] = include_bytes!("dbc/WowError_Strings.dbc");
const WORLD_STATE_UI: &[u8] = include_bytes!("dbc/WorldStateUI.dbc");
const WORLD_SAFE_LOCS: &[u8] = include_bytes!("dbc/WorldSafeLocs.dbc");
const WMO_AREA_TABLE: &[u8] = include_bytes!("dbc/WMOAreaTable.dbc");
const TAXI_NODES: &[u8] = include_bytes!("dbc/TaxiNodes.dbc");
const TALENT_TAB: &[u8] = include_bytes!("dbc/TalentTab.dbc");
const SPELL: &[u8] = include_bytes!("dbc/Spell.dbc");
const SPELL_SHAPESHIFT_FORM: &[u8] = include_bytes!("dbc/SpellShapeshiftForm.dbc");
const SPELL_RANGE: &[u8] = include_bytes!("dbc/SpellRange.dbc");
const SPELL_MECHANIC: &[u8] = include_bytes!("dbc/SpellMechanic.dbc");
const SPELL_ITEM_ENCHANTMENT: &[u8] = include_bytes!("dbc/SpellItemEnchantment.dbc");
const SPELL_FOCUS_OBJECT: &[u8] = include_bytes!("dbc/SpellFocusObject.dbc");
const SPELL_EFFECT_NAMES: &[u8] = include_bytes!("dbc/SpellEffectNames.dbc");
const SPELL_DISPEL_TYPE: &[u8] = include_bytes!("dbc/SpellDispelType.dbc");
const SPELL_AURA_NAMES: &[u8] = include_bytes!("dbc/SpellAuraNames.dbc");
const SKILL_LINE_CATEGORY: &[u8] = include_bytes!("dbc/SkillLineCategory.dbc");
const SKILL_LINE: &[u8] = include_bytes!("dbc/SkillLine.dbc");
const SERVER_MESSAGES: &[u8] = include_bytes!("dbc/ServerMessages.dbc");
const RESISTANCES: &[u8] = include_bytes!("dbc/Resistances.dbc");
const QUEST_SORT: &[u8] = include_bytes!("dbc/QuestSort.dbc");
const QUEST_INFO: &[u8] = include_bytes!("dbc/QuestInfo.dbc");
const PET_PERSONALITY: &[u8] = include_bytes!("dbc/PetPersonality.dbc");
const PET_LOYALTY: &[u8] = include_bytes!("dbc/PetLoyalty.dbc");
const PACKAGE: &[u8] = include_bytes!("dbc/Package.dbc");
const MAP: &[u8] = include_bytes!("dbc/Map.dbc");
const MAIL_TEMPLATE: &[u8] = include_bytes!("dbc/MailTemplate.dbc");
const LOCK_TYPE: &[u8] = include_bytes!("dbc/LockType.dbc");
const LFG_DUNGEONS: &[u8] = include_bytes!("dbc/LFGDungeons.dbc");
const LANGUAGES: &[u8] = include_bytes!("dbc/Languages.dbc");
const ITEM_SUB_CLASS_MASK: &[u8] = include_bytes!("dbc/ItemSubClassMask.dbc");
const ITEM_SET: &[u8] = include_bytes!("dbc/ItemSet.dbc");
const ITEM_RANDOM_PROPERTIES: &[u8] = include_bytes!("dbc/ItemRandomProperties.dbc");
const ITEM_PET_FOOD: &[u8] = include_bytes!("dbc/ItemPetFood.dbc");
const ITEM_CLASS: &[u8] = include_bytes!("dbc/ItemClass.dbc");
const ITEM_BAG_FAMILY: &[u8] = include_bytes!("dbc/ItemBagFamily.dbc");
const GM_TICKET_CATEGORY: &[u8] = include_bytes!("dbc/GMTicketCategory.dbc");
const GM_SURVEY_QUESTIONS: &[u8] = include_bytes!("dbc/GMSurveyQuestions.dbc");
const GAME_TIPS: &[u8] = include_bytes!("dbc/GameTips.dbc");
const FACTION_GROUP: &[u8] = include_bytes!("dbc/FactionGroup.dbc");
const FACTION: &[u8] = include_bytes!("dbc/Faction.dbc");
const EXHAUSTION: &[u8] = include_bytes!("dbc/Exhaustion.dbc");
const EMOTES_TEXT_DATA: &[u8] = include_bytes!("dbc/EmotesTextData.dbc");
const CREATURE_TYPE: &[u8] = include_bytes!("dbc/CreatureType.dbc");
const CREATURE_FAMILY: &[u8] = include_bytes!("dbc/CreatureFamily.dbc");
const CHR_RACES: &[u8] = include_bytes!("dbc/ChrRaces.dbc");
const CHR_CLASSES: &[u8] = include_bytes!("dbc/ChrClasses.dbc");
const CHAT_CHANNELS: &[u8] = include_bytes!("dbc/ChatChannels.dbc");
const CFG_CATEGORIES: &[u8] = include_bytes!("dbc/Cfg_Categories.dbc");
const AUCTION_HOUSE: &[u8] = include_bytes!("dbc/AuctionHouse.dbc");
const AREA_TABLE: &[u8] = include_bytes!("dbc/AreaTable.dbc");
const AREA_POI: &[u8] = include_bytes!("dbc/AreaPOI.dbc");

fn help(){
    let args: Vec<String> = env::args().collect();
    println!("Usage: {} <command>\nCommand :\n    search <string> [lang]\n    replace <old> <new> <lang>",&args[0]);
}

fn check_args(args: &[String],languages: &[&str]) -> Result<Vec<String>, ()>{
    let mut sel_language = String::new();
    let mut commands : Vec<String> = vec![];
    if true { //command passé automatiquement
        let path : String = "out-".to_string(); // fonctionnalité temporaire
        let verbose : String = "verbose".to_string(); // for test purposes
        commands.push(path);
        commands.push(verbose);
    }
    if args.len()<2{
        return Err(())
    } else if args[1].starts_with("search") && args.len()==4{            
            for &language in languages {
                if args[3].starts_with(language) {
                    sel_language=language.to_string();
                }
            }
            if sel_language=="" {
                println!("Unsupported language, use {} or {}", &languages[0], &languages[1]);
                return Err(())
            }
            commands.push(sel_language);
            commands.push("search".to_string());
            commands.push(args[2].clone());
            return Ok(commands)
    }
    else if args[1].starts_with("replace") && args.len()==5{
        for &language in languages {
            if args[4].starts_with(language) {
                sel_language=language.to_string();
            }
        }
        if sel_language=="" {
            println!("Unsupported language, use {} or {}", &languages[0], &languages[1] );
            return Err(())
        }
        commands.push(sel_language);
        commands.push("replace".to_string());
        commands.push(args[2].clone());
        commands.push(args[3].clone());
        return Ok(commands)
    }
    Err(())
}

fn search_or_replace<T, FuncOne, FuncTwo>(
    db: &mut T, 
    db_name: &str,
    select_fields: FuncOne,
    write_enabled: bool,
    args: &[String], 
    get_id: FuncTwo,
) -> u32
where
    T: DbcTable, 
    FuncOne: for<'a> Fn(&'a mut T::Row, &str) -> Vec<&'a mut String>, 
    FuncTwo: Fn(&T::Row) -> u32,
    {
    let mut found_elements = 0;
    let lang : &str = &args[2];
    for elem in db.rows_mut() { 
        let id = get_id(elem).clone();
        let fields = select_fields(elem, lang); 
        for field in fields {
            
            if field.contains(&args[4]) {
                let old = field.clone();
                if write_enabled{
                    *field = field.replace(&args[4], &args[5]);
                    if args[1]=="verbose"{ //currently args[1] contains the string "verbose"
                        println!("Field:\t\t{old}\nReplaced by : \t{field}\n==== At id: {id} in {db_name} ====");
                    }
                } else{
                    if args[1]=="verbose"{ //currently args[1] contains the string "verbose"
                        println!("Field:\t\t{old}\n==== At id: {id} in {db_name} ====");
                    }
                }
                found_elements += 1;
            }
        }
    }
    let mut path = args[0].clone();
    path.push_str(db_name.clone());
    if write_enabled{
        T::write(db, &mut File::create(path).expect("Error creating file"))
        .expect("Write error");
        if found_elements > 0 {
            println!("{found_elements} element replaced in {db_name}");
        }
    } else {
        if found_elements > 0 {
            println!("{found_elements} element found in {db_name}");
        }
    }
    found_elements
}

fn load_and_process(args: &[String]){   
    let mut total_count : u32 = 0;
    let mut write_enabled :bool;
    if args[3]=="replace"{
        println!("== Text to replace : '{}' replacement text : '{}', Selected language : '{}' == ", &args[4],  &args[5],  &args[2]);
        write_enabled = true;
    } else if args[3]=="search"{
        println!("== Text to search : '{}', Selected language : '{}' ==", &args[4], &args[2]);
        write_enabled = false;
    } else {println!("internal error");std::process::exit(1);}
    if args[1]=="verbose"{ //currently args[1] contains the string "verbose"
        println!("loading dbc files ..");
    }
    //load the dbc file into variables
    let mut wow_error_strings_db = vanilla_tables::wow_error_strings::WowError_Strings::read(&mut WOW_ERROR_STRINGS).unwrap(); 
    let mut world_state_ui_db = vanilla_tables::world_state_ui::WorldStateUI::read(&mut WORLD_STATE_UI).unwrap(); 
    let mut world_safe_locs_db = vanilla_tables::world_safe_locs::WorldSafeLocs::read(&mut WORLD_SAFE_LOCS).unwrap(); 
    let mut wmo_area_table_db = vanilla_tables::wmo_area_table::WMOAreaTable::read(&mut WMO_AREA_TABLE).unwrap(); 
    let mut taxi_nodes_db = vanilla_tables::taxi_nodes::TaxiNodes::read(&mut TAXI_NODES).unwrap(); 
    let mut talent_tab_db = vanilla_tables::talent_tab::TalentTab::read(&mut TALENT_TAB).unwrap(); 
    let mut spell_db = vanilla_tables::spell::Spell::read(&mut SPELL).unwrap(); 
    let mut spell_shapeshift_form_db = vanilla_tables::spell_shapeshift_form::SpellShapeshiftForm::read(&mut SPELL_SHAPESHIFT_FORM).unwrap(); 
    let mut spell_range_db = vanilla_tables::spell_range::SpellRange::read(&mut SPELL_RANGE).unwrap(); 
    let mut spell_mechanic_db = vanilla_tables::spell_mechanic::SpellMechanic::read(&mut SPELL_MECHANIC).unwrap(); 
    let mut spell_item_enchantment_db = vanilla_tables::spell_item_enchantment::SpellItemEnchantment::read(&mut SPELL_ITEM_ENCHANTMENT).unwrap(); 
    let mut spell_focus_object_db = vanilla_tables::spell_focus_object::SpellFocusObject::read(&mut SPELL_FOCUS_OBJECT).unwrap(); 
    // let mut spell_effect_names_db = vanilla_tables::spell_effect_names::SpellEffectNames::read(&mut SPELL_EFFECT_NAMES).unwrap();  //doesn't work
    let mut spell_dispel_type_db = vanilla_tables::spell_dispel_type::SpellDispelType::read(&mut SPELL_DISPEL_TYPE).unwrap(); 
    // let mut spell_aura_names_db = vanilla_tables::spell_aura_names::SpellAuraNames::read(&mut SPELL_AURA_NAMES).unwrap).unwrap();  //doesn't work
    let mut skill_line_category_db = vanilla_tables::skill_line_category::SkillLineCategory::read(&mut SKILL_LINE_CATEGORY).unwrap();
    let mut skill_line_db = vanilla_tables::skill_line::SkillLine::read(&mut SKILL_LINE).unwrap(); 
    let mut server_messages_db = vanilla_tables::server_messages::ServerMessages::read(&mut SERVER_MESSAGES).unwrap(); 
    let mut resistances_db = vanilla_tables::resistances::Resistances::read(&mut RESISTANCES).unwrap(); 
    let mut quest_sort_db = vanilla_tables::quest_sort::QuestSort::read(&mut QUEST_SORT).unwrap(); 
    let mut quest_info_db = vanilla_tables::quest_info::QuestInfo::read(&mut QUEST_INFO).unwrap(); 
    let mut pet_personality_db = vanilla_tables::pet_personality::PetPersonality::read(&mut PET_PERSONALITY).unwrap(); 
    let mut pet_loyalty_db = vanilla_tables::pet_loyalty::PetLoyalty::read(&mut PET_LOYALTY).unwrap(); 
    let mut package_db = vanilla_tables::package::Package::read(&mut PACKAGE).unwrap(); 
    let mut map_db = vanilla_tables::map::Map::read(&mut MAP).unwrap(); 
    let mut mail_template_db = vanilla_tables::mail_template::MailTemplate::read(&mut MAIL_TEMPLATE).unwrap(); 
    let mut lock_type_db = vanilla_tables::lock_type::LockType::read(&mut LOCK_TYPE).unwrap(); 
    let mut lfg_dungeons_db = vanilla_tables::lfg_dungeons::LFGDungeons::read(&mut LFG_DUNGEONS).unwrap(); 
    let mut languages_db = vanilla_tables::languages::Languages::read(&mut LANGUAGES).unwrap(); 
    let mut item_sub_class_mask_db = vanilla_tables::item_sub_class_mask::ItemSubClassMask::read(&mut ITEM_SUB_CLASS_MASK).unwrap(); 
    let mut item_set_db = vanilla_tables::item_set::ItemSet::read(&mut ITEM_SET).unwrap(); 
    let mut item_random_properties_db = vanilla_tables::item_random_properties::ItemRandomProperties::read(&mut ITEM_RANDOM_PROPERTIES).unwrap(); 
    let mut item_pet_food_db = vanilla_tables::item_pet_food::ItemPetFood::read(&mut ITEM_PET_FOOD).unwrap(); 
    let mut item_class_db = vanilla_tables::item_class::ItemClass::read(&mut ITEM_CLASS).unwrap(); 
    let mut item_bag_family_db = vanilla_tables::item_bag_family::ItemBagFamily::read(&mut ITEM_BAG_FAMILY).unwrap(); 
    let mut gm_ticket_category_db = vanilla_tables::gm_ticket_category::GMTicketCategory::read(&mut GM_TICKET_CATEGORY).unwrap(); 
    let mut gm_survey_questions_db = vanilla_tables::gm_survey_questions::GMSurveyQuestions::read(&mut GM_SURVEY_QUESTIONS).unwrap(); 
    let mut game_tips_db = vanilla_tables::game_tips::GameTips::read(&mut GAME_TIPS).unwrap(); 
    let mut faction_group_db = vanilla_tables::faction_group::FactionGroup::read(&mut FACTION_GROUP).unwrap(); 
    let mut faction_db = vanilla_tables::faction::Faction::read(&mut FACTION).unwrap(); 
    let mut exhaustion_db = vanilla_tables::exhaustion::Exhaustion::read(&mut EXHAUSTION).unwrap(); 
    let mut emotes_text_data_db = vanilla_tables::emotes_text_data::EmotesTextData::read(&mut EMOTES_TEXT_DATA).unwrap(); 
    let mut creature_type_db = vanilla_tables::creature_type::CreatureType::read(&mut CREATURE_TYPE).unwrap(); 
    let mut creature_family_db = vanilla_tables::creature_family::CreatureFamily::read(&mut CREATURE_FAMILY).unwrap(); 
    let mut chr_races_db = vanilla_tables::chr_races::ChrRaces::read(&mut CHR_RACES).unwrap(); 
    let mut chr_classes_db = vanilla_tables::chr_classes::ChrClasses::read(&mut CHR_CLASSES).unwrap(); 
    let mut chat_channels_db = vanilla_tables::chat_channels::ChatChannels::read(&mut CHAT_CHANNELS).unwrap(); 
    let mut cfg_categories_db = vanilla_tables::cfg_categories::Cfg_Categories::read(&mut CFG_CATEGORIES).unwrap(); 
    let mut auction_house_db = vanilla_tables::auction_house::AuctionHouse::read(&mut AUCTION_HOUSE).unwrap(); 
    let mut area_table_db = vanilla_tables::area_table::AreaTable::read(&mut AREA_TABLE).unwrap(); 
    let mut area_poi_db = vanilla_tables::area_poi::AreaPOI::read(&mut AREA_POI).unwrap();
    if args[1]=="verbose"{ //currently args[1] contains the string "verbose"
        println!("dbc files loaded successfully!");
    }

    // Send dbc and associated field to be search or replaced.
    total_count += search_or_replace(&mut wow_error_strings_db, "WowError_Strings.dbc",
        |row, lang| match lang {
            "es_es" => vec![ &mut row.text.es_es, ],
            "fr_fr" => vec![ &mut row.text.fr_fr, ],
            _ => { help(); std::process::exit(1); }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );
    
    total_count += search_or_replace(&mut world_state_ui_db, "WorldStateUI.dbc",
        |row, lang| match lang {
            "es_es" => vec![ &mut row.state_variable.es_es, &mut row.tooltip.es_es,&mut row.dynamic_tooltip.es_es, ],
            "fr_fr" => vec![ &mut row.state_variable.fr_fr, &mut row.tooltip.fr_fr,&mut row.dynamic_tooltip.fr_fr, ],
                _   => { help();std::process::exit(1);}
        },write_enabled, &args, |row| -> u32 {row.id.id}, );

    total_count += search_or_replace(&mut world_safe_locs_db, "WorldSafeLocs.dbc",
        |row, lang| match lang {
            "es_es" => vec![ &mut row.area_name.es_es, ],
            "fr_fr" => vec![ &mut row.area_name.fr_fr, ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );

    total_count += search_or_replace(&mut wmo_area_table_db, "WMOAreaTable.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.name.es_es,  ],  
            "fr_fr" => vec![&mut row.name.fr_fr,  ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );

    total_count += search_or_replace(&mut taxi_nodes_db, "TaxiNodes.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.name.es_es,  ],  
            "fr_fr" => vec![&mut row.name.fr_fr,  ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );

    total_count += search_or_replace(&mut talent_tab_db, "TalentTab.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.name.es_es,  ],  
            "fr_fr" => vec![&mut row.name.fr_fr,  ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );

        total_count += search_or_replace(    &mut spell_db, "Spell.dbc", 
            |row, lang| match lang {
                "es_es" => vec![ &mut row.name.es_es, &mut row.name_subtext.es_es, &mut row.description.es_es, &mut row.aura_description.es_es, ],
                "fr_fr" => vec![ &mut row.name.fr_fr, &mut row.name_subtext.fr_fr, &mut row.description.fr_fr, &mut row.aura_description.fr_fr, ],
                _ => {help(); std::process::exit(1);}
            },write_enabled, &args, |row| ->  u32  {row.id.id},
        );

    total_count += search_or_replace(&mut spell_shapeshift_form_db, "SpellShapeshiftForm.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.name.es_es,  ],  
            "fr_fr" => vec![&mut row.name.fr_fr,  ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );
        
    total_count += search_or_replace(&mut spell_range_db, "SpellRange.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.display_name.es_es,&mut row.display_name_short.es_es, ],  
            "fr_fr" => vec![&mut row.display_name.fr_fr,&mut row.display_name_short.fr_fr, ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );

    total_count += search_or_replace(&mut spell_mechanic_db, "SpellMechanic.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.state_name.es_es,  ],  
            "fr_fr" => vec![&mut row.state_name.fr_fr,  ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );

    total_count += search_or_replace(&mut spell_item_enchantment_db, "SpellItemEnchantment.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.name.es_es,  ],  
            "fr_fr" => vec![&mut row.name.fr_fr,  ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );

    total_count += search_or_replace(&mut spell_focus_object_db, "SpellFocusObject.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.name.es_es,  ],  
            "fr_fr" => vec![&mut row.name.fr_fr,  ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );

    total_count += search_or_replace(&mut spell_dispel_type_db, "SpellDispelType.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.name.es_es,  ],  
            "fr_fr" => vec![&mut row.name.fr_fr,  ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );
    total_count += search_or_replace(&mut skill_line_category_db, "SkillLineCategory.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.name.es_es,  ],  
            "fr_fr" => vec![&mut row.name.fr_fr,  ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );
    total_count += search_or_replace(&mut skill_line_db, "SkillLine.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.display_name.es_es, &mut row.description.es_es, ],  
            "fr_fr" => vec![&mut row.display_name.fr_fr, &mut row.description.fr_fr, ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );
    total_count += search_or_replace(&mut server_messages_db, "ServerMessages.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.text.es_es,  ],  
            "fr_fr" => vec![&mut row.text.fr_fr,  ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );
    total_count += search_or_replace(&mut resistances_db, "Resistances.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.name.es_es,  ],  
            "fr_fr" => vec![&mut row.name.fr_fr,  ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );
    total_count += search_or_replace(&mut quest_sort_db, "QuestSort.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.name.es_es,  ],  
            "fr_fr" => vec![&mut row.name.fr_fr,  ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );
    total_count += search_or_replace(&mut quest_info_db, "QuestInfo.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.name.es_es,  ],  
            "fr_fr" => vec![&mut row.name.fr_fr,  ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );
    total_count += search_or_replace(&mut pet_personality_db, "PetPersonality.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.name.es_es,  ],  
            "fr_fr" => vec![&mut row.name.fr_fr,  ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );
    total_count += search_or_replace(&mut pet_loyalty_db, "PetLoyalty.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.name.es_es,  ],  
            "fr_fr" => vec![&mut row.name.fr_fr,  ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );
    total_count += search_or_replace(&mut package_db, "Package.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.name.es_es,  ],  
            "fr_fr" => vec![&mut row.name.fr_fr,  ], 
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );
    total_count += search_or_replace(&mut map_db, "Map.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.map_name.es_es, &mut row.map_description_horde.es_es, &mut row.map_description_alliance.es_es, ],  
            "fr_fr" => vec![&mut row.map_name.fr_fr, &mut row.map_description_horde.fr_fr, &mut row.map_description_alliance.fr_fr, ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );
    total_count += search_or_replace(&mut mail_template_db, "MailTemplate.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.body.es_es,  ],  
            "fr_fr" => vec![&mut row.body.fr_fr,  ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );
    total_count += search_or_replace(&mut lock_type_db, "LockType.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.name.es_es, &mut row.resource_name.es_es, &mut row.verb.es_es, ],  
            "fr_fr" => vec![&mut row.name.fr_fr, &mut row.resource_name.fr_fr, &mut row.verb.fr_fr, ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );
    total_count += search_or_replace(&mut lfg_dungeons_db, "LFGDungeons.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.name.es_es,  ],  
            "fr_fr" => vec![&mut row.name.fr_fr,  ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );
    total_count += search_or_replace(&mut languages_db, "Languages.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.name.es_es,  ],  
            "fr_fr" => vec![&mut row.name.fr_fr,  ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );
    total_count += search_or_replace(&mut item_sub_class_mask_db, "ItemSubClassMask.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.name.es_es,  ],  
            "fr_fr" => vec![&mut row.name.fr_fr,  ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {0}, ); //no id column in this dbc
    total_count += search_or_replace(&mut item_set_db, "ItemSet.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.name.es_es,  ],  
            "fr_fr" => vec![&mut row.name.fr_fr,  ], 
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id as u32}, ); //no id is i32 in this dbc
    total_count += search_or_replace(&mut item_random_properties_db, "ItemRandomProperties.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.suffix.es_es,  ],  
            "fr_fr" => vec![&mut row.suffix.fr_fr,  ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );
    total_count += search_or_replace(&mut item_pet_food_db, "ItemPetFood.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.name.es_es,  ],  
            "fr_fr" => vec![&mut row.name.fr_fr,  ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );
    total_count += search_or_replace(&mut item_class_db, "ItemClass.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.class_name.es_es,  ],  
            "fr_fr" => vec![&mut row.class_name.fr_fr,  ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );
    total_count += search_or_replace(&mut item_bag_family_db, "ItemBagFamily.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.name.es_es,  ],  
            "fr_fr" => vec![&mut row.name.fr_fr,  ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );
    total_count += search_or_replace(&mut gm_ticket_category_db, "GMTicketCategory.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.name.es_es,  ],  
            "fr_fr" => vec![&mut row.name.fr_fr,  ], 
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );
    total_count += search_or_replace(&mut gm_survey_questions_db, "GMSurveyQuestions.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.question.es_es,  ],  
            "fr_fr" => vec![&mut row.question.fr_fr,  ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );
    total_count += search_or_replace(&mut game_tips_db, "GameTips.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.text.es_es,  ],  
            "fr_fr" => vec![&mut row.text.fr_fr,  ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );
    total_count += search_or_replace(&mut faction_group_db, "FactionGroup.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.name.es_es,  ],  
            "fr_fr" => vec![&mut row.name.fr_fr,  ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );
    total_count += search_or_replace(&mut faction_db, "Faction.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.name.es_es, &mut row.description.es_es ],  
            "fr_fr" => vec![&mut row.name.fr_fr, &mut row.description.fr_fr ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );
    total_count += search_or_replace(&mut exhaustion_db , "Exhaustion.dbc",
        |row, lang| match lang {  
            "es_es" => vec![&mut row.state_name.es_es,  ],  
            "fr_fr" => vec![&mut row.state_name.fr_fr,  ],  
            _ => {help();std::process::exit(1);  }
        },write_enabled, &args, |row| -> u32 {row.id.id}, );
    total_count += search_or_replace(&mut area_poi_db,"AreaPOI.dbc",
        |row, lang| match lang {
            "es_es" => vec![&mut row.name.es_es,],
            "fr_fr" => vec![&mut row.name.fr_fr,],
            _ => {help();std::process::exit(1);}
        },write_enabled, &args, |row| ->  u32  {row.id.id}, );

    total_count += search_or_replace(&mut emotes_text_data_db, "EmotesTextData.dbc",
        |row, lang| match lang {
            "es_es" => vec![ &mut row.text.es_es, ],
            "fr_fr" => vec![ &mut row.text.fr_fr, ],
            _ => { help(); std::process::exit(1); }
        },write_enabled, &args, |row| ->  u32  {row.id.id}, ); 
        
    total_count += search_or_replace(&mut emotes_text_data_db, "EmotesTextData.dbc",
        |row, lang| match lang {
            "es_es" => vec![ &mut row.text.es_es, ],
            "fr_fr" => vec![ &mut row.text.fr_fr, ],
            _ => { help(); std::process::exit(1); }
        },write_enabled, &args, |row| ->  u32  {row.id.id}, );

    total_count += search_or_replace(&mut creature_type_db, "CreatureType.dbc",
        |row, lang| match lang {
            "es_es" => vec![ &mut row.name.es_es, ],
            "fr_fr" => vec![ &mut row.name.fr_fr, ],
            _ => { help(); std::process::exit(1); }
        },write_enabled, &args, |row| ->  u32  {row.id.id}, );

    total_count += search_or_replace(&mut creature_family_db, "CreatureFamily.dbc",
            |row, lang| match lang {
                "es_es" => vec![ &mut row.name.es_es, ],
                "fr_fr" => vec![ &mut row.name.fr_fr, ],
                _ => { help(); std::process::exit(1); }
            },write_enabled, &args, |row| ->  u32  {row.id.id}, );

    total_count += search_or_replace(&mut chr_classes_db, "ChrClasses.dbc",
        |row, lang| match lang {
            "es_es" => vec![ &mut row.name.es_es, ],
            "fr_fr" => vec![ &mut row.name.fr_fr, ],
            _ => { help(); std::process::exit(1); }
        },write_enabled, &args, |row| ->  u32  {row.id.id}, );

    total_count += search_or_replace(&mut chat_channels_db, "ChatChannels.dbc",
        |row, lang| match lang {
            "es_es" => vec![ &mut row.name.es_es, &mut row.shortcut.es_es, ],
            "fr_fr" => vec![ &mut row.name.fr_fr, &mut row.shortcut.fr_fr, ],
            _ => { help(); std::process::exit(1); }
        },write_enabled, &args, |row| ->  u32  {row.id.id}, );

    total_count += search_or_replace(&mut cfg_categories_db, "Cfg_Categories.dbc",
        |row, lang| match lang {
            "es_es" => vec![ &mut row.name.es_es, ],
            "fr_fr" => vec![ &mut row.name.fr_fr, ],
            _ => { help(); std::process::exit(1); }
        },write_enabled, &args, |row| ->  u32  {0}, );

    total_count += search_or_replace(&mut auction_house_db, "AuctionHouse.dbc",
        |row, lang| match lang {
            "es_es" => vec![ &mut row.name.es_es, ],
            "fr_fr" => vec![ &mut row.name.fr_fr, ],
            _ => { help(); std::process::exit(1); }
        },write_enabled, &args, |row| ->  u32  {row.id.id}, );

    total_count += search_or_replace(&mut area_table_db, "AreaTable.dbc",
        |row, lang| match lang {
            "es_es" => vec![ &mut row.area_name.es_es, ],
            "fr_fr" => vec![ &mut row.area_name.fr_fr, ],
            _ => { help(); std::process::exit(1); }
        },write_enabled, &args, |row| ->  u32  {row.id.id}, );

    if write_enabled {
        println!("Total elements modified: {}", total_count);
    } else {
        println!("Total elements found: {}", total_count);
    }    
}

fn main() {
    let languages = vec!["es_es", "fr_fr"];
    let args: Vec<String> = env::args().collect();
    match check_args(&args,&languages) {
        Ok(read_command)=>{
            load_and_process(&read_command[..]);
        }
        Err(_)=>{
            help();
            std::process::exit(1);
        }
    }
}