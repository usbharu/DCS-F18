use dcs_bios_const::json_type::Function;
use std::{collections::HashSet, ops::RangeInclusive};

#[derive(Debug, PartialEq, Eq,Hash)]
pub enum Filter {
    IdFilter { id: String, addresses: Vec<u16> },
    AddressFilter { address: u16 },
}

pub struct Metric{
    module:String,
    
}

impl Filter {
    pub fn contains(&self, addr: &RangeInclusive<u16>) -> bool {
        match &self {
            Filter::IdFilter { id, addresses } => {
                addresses.iter().any(|f| addr.contains(f))
            },
            Filter::AddressFilter { address } => addr.contains(address),
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
