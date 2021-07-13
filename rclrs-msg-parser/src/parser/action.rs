use std::fs;
use std::path::Path;

use anyhow::Result;
use regex::Regex;

use super::msg::parse_message_string;
use crate::error::RclMsgError;
use crate::spec::Action;

const ACTION_GOAL_SUFFIX: &str = "_Goal";
const ACTION_RESULT_SUFFIX: &str = "_Result";
const ACTION_FEEDBACK_SUFFIX: &str = "_Feedback";

pub fn parse_action_file<P: AsRef<Path>>(pkg_name: &str, interface_file: P) -> Result<Action> {
    parse_action_string(
        pkg_name,
        interface_file
            .as_ref()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap(),
        fs::read_to_string(interface_file.as_ref())?.as_str(),
    )
}

pub fn parse_action_string(
    pkg_name: &str,
    action_name: &str,
    action_string: &str,
) -> Result<Action> {
    let re = Regex::new(r"(?m)^---$").unwrap();
    let action_blocks: Vec<_> = re.split(action_string).collect();
    if action_blocks.len() != 3 {
        return Err(RclMsgError::InvalidActionSpecification(
            "Number of '---' separators nonconformant with action definition".into(),
        )
        .into());
    }

    Ok(Action {
        package: pkg_name.into(),
        name: action_name.into(),
        goal: parse_message_string(
            pkg_name,
            &format!("{}{}", action_name, ACTION_GOAL_SUFFIX),
            action_blocks[0],
        )?,
        result: parse_message_string(
            pkg_name,
            &format!("{}{}", action_name, ACTION_RESULT_SUFFIX),
            action_blocks[1],
        )?,
        feedback: parse_message_string(
            pkg_name,
            &format!("{}{}", action_name, ACTION_FEEDBACK_SUFFIX),
            action_blocks[2],
        )?,
    })
}
