use dcs_bios_const::json_type::Function;
use std::{collections::HashSet, u16, vec};

#[derive(Debug, PartialEq, Eq,Hash)]
pub enum Filter {
    IdFilter { id: String, addresses: Vec<u16> },
    AddressFilter { address: u16 },
}

impl Filter {
    fn contains(&self, addr: u16) -> bool {
        match &self {
            Filter::IdFilter { id, addresses } => addresses.contains(&addr),
            Filter::AddressFilter { address } => *address == addr,
        }
    }
}

pub fn build_filter(
    ids: Vec<String>,
    addresses: Vec<u16>,
    functions: Vec<Function>,
) -> HashSet<Filter> {
    functions
        .into_iter()
        .filter_map(|ele| -> Option<Vec<Filter>> {
            if ids.contains(&ele.identifier) {
                Some(vec![Filter::IdFilter {
                    id: ele.identifier,
                    addresses: ele.outputs.iter().map(|v| v.address).collect(),
                }])
            } else {
                let filters = ele
                    .outputs
                    .iter()
                    .filter_map(|f| -> Option<Filter> {
                        if addresses.contains(&f.address) {
                            Some(Filter::AddressFilter { address: f.address })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Filter>>();

                if filters.is_empty() {
                    None
                } else {
                    Some(filters)
                }
            }
        })
        .flatten()
        .collect::<HashSet<Filter>>()
}
