use std::collections::BTreeMap;

use serde::Serializer;

use crate::role::LuroRolePositions;

pub fn serialize_role_positions<S,>(input: &LuroRolePositions, s: S,) -> Result<S::Ok, S::Error,>
where
    S: Serializer,
{
    let data = input
        .clone()
        .into_iter()
        .map(|(str_key, value,)| (str_key.to_string(), value,),)
        .collect::<BTreeMap<String, _,>>();

    s.collect_map(data,)
}
