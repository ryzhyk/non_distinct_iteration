#![allow(unused_imports, non_snake_case, non_camel_case_types, non_upper_case_globals, unused_parens, non_shorthand_field_patterns, dead_code, overflowing_literals, clippy::ptr_arg)]

extern crate fnv;
extern crate differential_dataflow;
extern crate timely;
extern crate num_traits;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate libc;
extern crate twox_hash;

#[macro_use]
extern crate differential_datalog;

#[macro_use]
extern crate abomonation;
extern crate ddlog_ovsdb_adapter;

use differential_dataflow::collection;
use timely::dataflow::scopes;
use timely::worker;
use timely::communication;

use differential_datalog::program::*;
use differential_datalog::uint::*;
use differential_datalog::int::*;
use differential_datalog::arcval;
use differential_datalog::record;
use differential_datalog::record::{FromRecord, IntoRecord, Mutator};
use abomonation::Abomonation;

use fnv::{FnvHashSet, FnvHashMap};
use std::fmt::Display;
use std::fmt;
use std::sync;
use std::hash::Hash;
use std::hash::Hasher;
use std::os::raw;
use std::borrow;
use std::ptr;
use std::ffi;
use std::boxed;
use std::fs;
use std::ops::Deref;
use std::io::Write;
use std::os::unix;
use std::os::unix::io::{IntoRawFd, FromRawFd};
use std::mem;
use num_traits::identities::One;
use libc::size_t;

pub mod valmap;
pub mod update_handler;
pub mod ovsdb;
pub mod api;

pub fn string_append_str(mut s1: String, s2: &str) -> String
{
    s1.push_str(s2);
    s1
}

pub fn string_append(mut s1: String, s2: &String) -> String
{
    s1.push_str(s2.as_str());
    s1
}


pub use __std::*;
mod __std {
    use super::*;
    /// Rust implementation of DDlog standard library functions and types.
    
    extern crate num;
    
    use differential_datalog::arcval;
    use differential_datalog::record::*;
    
    use std::fmt::Display;
    use std::fmt;
    use std::hash::Hash;
    use std::hash::Hasher;
    use twox_hash::XxHash;
    use std::vec;
    use std::collections::btree_set;
    use std::collections::btree_map;
    use std::vec::{Vec};
    use std::collections::{BTreeMap, BTreeSet};
    use std::iter::FromIterator;
    use std::ops;
    use std::cmp;
    use std::marker;
    use std::slice;
    
    const XX_SEED1: u64 = 0x23b691a751d0e108;
    const XX_SEED2: u64 = 0x20b09801dce5ff84;
    
    // Ref
    pub type std_Ref<A> = arcval::ArcVal<A>;
    
    pub fn std_ref_new<A: Clone>(x: &A) -> std_Ref<A> {
        arcval::ArcVal::from(x.clone())
    }
    
    pub fn std_deref<A: Clone>(x: &std_Ref<A>) -> &A {
        x.deref()
    }
    
    // min/max
    pub fn std_max<A: Ord + Clone>(x: &A, y: &A) -> A {
        if *x >= *y {
            x.clone()
        } else {
            y.clone()
        }
    }
    
    pub fn std_min<A: Ord + Clone>(x: &A, y: &A) -> A {
        if *x <= *y {
            x.clone()
        } else {
            y.clone()
        }
    }
    
    // Arithmetic functions
    pub fn std_pow32<T: num::One + ops::Mul + Clone>(base: &T, exp: &u32) -> T {
        num::pow::pow(base.clone(), *exp as usize)
    }
    
    // Option
    pub fn option2std<T: Clone>(x: Option<T>) -> std_Option<T> {
        match x {
            None => std_Option::std_None,
            Some(v) => std_Option::std_Some{x: v}
        }
    }
    
    // Range
    pub fn std_range<A: Clone + Ord + ops::Add<Output = A> + PartialOrd>(from: &A, to: &A, step: &A) -> std_Vec<A> {
        let mut vec = std_Vec::new();
        let mut x = from.clone();
        while x <= *to {
            vec.push(x.clone());
            x = x + step.clone();
        };
        vec
    }
    
    // Vector
    
    #[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize, Debug, Default)]
    pub struct std_Vec<T> {
        pub x: Vec<T>
    }
    
    /* This is needed so we can support for-loops over `Vec`'s
     */
    pub struct VecIter<'a, X> {
        iter: slice::Iter<'a, X>
    }
    
    impl<'a, X> VecIter<'a, X> {
        pub fn new(vec: &'a std_Vec<X>) -> VecIter<'a, X> {
            VecIter{iter: vec.x.iter()}
        }
    }
    
    impl<'a, X> Iterator for VecIter<'a, X> {
        type Item = &'a X;
    
        fn next(&mut self) -> Option<Self::Item> {
            self.iter.next()
        }
    
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.iter.size_hint()
        }
    }
    
    impl <'a, T> std_Vec<T> {
        pub fn iter(&'a self) -> VecIter<'a, T> {
            VecIter::new(self)
        }
    }
    
    impl <T: Ord> std_Vec<T> {
        pub fn new() -> Self {
            std_Vec{x: Vec::new()}
        }
        pub fn with_capacity(capacity: usize) -> Self {
            std_Vec{x: Vec::with_capacity(capacity)}
        }
        pub fn push(&mut self, v: T) {
            self.x.push(v);
        }
    }
    
    impl<T: FromRecord> FromRecord for std_Vec<T> {
        fn from_record(val: &Record) -> Result<Self, String> {
            Vec::from_record(val).map(|x|std_Vec{x})
        }
    }
    
    impl<T: IntoRecord> IntoRecord for std_Vec<T> {
        fn into_record(self) -> Record {
            self.x.into_record()
        }
    }
    
    impl<T: FromRecord> Mutator<std_Vec<T>> for Record
    {
        fn mutate(&self, vec: &mut std_Vec<T>) -> Result<(), String> {
            self.mutate(&mut vec.x)
        }
    }
    
    impl<T: Display> Display for std_Vec<T> {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            let len = self.x.len();
            formatter.write_str("[")?;
            for (i, v) in self.x.iter().enumerate() {
                formatter.write_fmt(format_args!("{}", *v))?;
                if i < len-1 {
                    formatter.write_str(",")?;
                }
            }
            formatter.write_str("]")?;
            Ok(())
        }
    }
    
    impl<T> IntoIterator for std_Vec<T> {
        type Item = T;
        type IntoIter = vec::IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter {
            self.x.into_iter()
        }
    }
    
    pub fn std_vec_len<X: Ord + Clone>(v: &std_Vec<X>) -> u64 {
        v.x.len() as u64
    }
    
    pub fn std_vec_empty<X: Ord + Clone>() -> std_Vec<X> {
        std_Vec::new()
    }
    
    pub fn std_vec_singleton<X: Ord + Clone>(x: &X) -> std_Vec<X> {
        std_Vec{x: vec![x.clone()]}
    }
    
    pub fn std_vec_push<X: Ord+Clone>(v: &mut std_Vec<X>, x: &X) {
        v.push((*x).clone());
    }
    
    pub fn std_vec_push_imm<X: Ord+Clone>(v: &std_Vec<X>, x: &X) -> std_Vec<X> {
        let mut v2 = v.clone();
        v2.push((*x).clone());
        v2
    }
    
    pub fn std_vec_contains<X: Ord>(v: &std_Vec<X>, x: &X) -> bool {
        v.x.contains(x)
    }
    
    pub fn std_vec_is_empty<X: Ord>(v: &std_Vec<X>) -> bool {
        v.x.is_empty()
    }
    
    pub fn std_vec_nth<X: Ord + Clone>(v: &std_Vec<X>, n: &u64) -> std_Option<X> {
        option2std(v.x.get(*n as usize).cloned())
    }
    
    pub fn std_vec2set<X: Ord + Clone>(s: &std_Vec<X>) -> std_Set<X> {
        std_Set{x: s.x.iter().cloned().collect()}
    }
    
    // Set
    
    #[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize, Debug, Default)]
    pub struct std_Set<T: Ord> {
        pub x: BTreeSet<T>
    }
    
    /* This is needed so we can support for-loops over `Set`'s
     */
    pub struct SetIter<'a, X> {
        iter: btree_set::Iter<'a, X>
    }
    
    impl<'a, X: Ord> SetIter<'a, X> {
        pub fn new(set: &'a std_Set<X>) -> SetIter<'a, X> {
            SetIter{iter: set.x.iter()}
        }
    }
    
    impl<'a, X> Iterator for SetIter<'a, X> {
        type Item = &'a X;
    
        fn next(&mut self) -> Option<Self::Item> {
            self.iter.next()
        }
    
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.iter.size_hint()
        }
    }
    
    impl <'a, T: Ord> std_Set<T> {
        pub fn iter(&'a self) -> SetIter<'a, T> {
            SetIter::new(self)
        }
    }
    
    impl <T: Ord> std_Set<T> {
        pub fn new() -> Self {
            std_Set{x: BTreeSet::new()}
        }
        pub fn insert(&mut self, v: T) {
            self.x.insert(v);
        }
    }
    
    impl<T: FromRecord + Ord> FromRecord for std_Set<T> {
        fn from_record(val: &Record) -> Result<Self, String> {
            BTreeSet::from_record(val).map(|x|std_Set{x})
        }
    }
    
    impl<T: IntoRecord + Ord> IntoRecord for std_Set<T> {
        fn into_record(self) -> Record {
            self.x.into_record()
        }
    }
    
    impl<T: FromRecord + Ord> Mutator<std_Set<T>> for Record
    {
        fn mutate(&self, set: &mut std_Set<T>) -> Result<(), String> {
            self.mutate(&mut set.x)
        }
    }
    
    impl<T: Ord> IntoIterator for std_Set<T> {
        type Item = T;
        type IntoIter = btree_set::IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter {
            self.x.into_iter()
        }
    }
    
    impl<T: Ord> FromIterator<T> for std_Set<T> {
        fn from_iter<I>(iter: I) -> Self
        where I: IntoIterator<Item = T>
        {
            std_Set{x: BTreeSet::from_iter(iter)}
        }
    }
    
    
    impl<T: Display + Ord> Display for std_Set<T> {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            let len = self.x.len();
            formatter.write_str("[")?;
            for (i, v) in self.x.iter().enumerate() {
                formatter.write_fmt(format_args!("{}", *v))?;
                if i < len-1 {
                    formatter.write_str(",")?;
                }
            }
            formatter.write_str("]")?;
            Ok(())
        }
    }
    
    pub fn std_set_size<X: Ord + Clone>(s: &std_Set<X>) -> u64 {
        s.x.len() as u64
    }
    
    pub fn std_set_empty<X: Ord + Clone>() -> std_Set<X> {
        std_Set::new()
    }
    
    pub fn std_set_singleton<X: Ord + Clone>(v: &X) -> std_Set<X> {
        let mut s = std_Set::new();
        s.insert(v.clone());
        s
    }
    
    pub fn std_set_insert<X: Ord+Clone>(s: &mut std_Set<X>, v: &X) {
        s.x.insert((*v).clone());
    }
    
    pub fn std_set_insert_imm<X: Ord+Clone>(s: &std_Set<X>, v: &X) -> std_Set<X> {
        let mut s2 = s.clone();
        s2.insert((*v).clone());
        s2
    }
    
    pub fn std_set_contains<X: Ord>(s: &std_Set<X>, v: &X) -> bool {
        s.x.contains(v)
    }
    
    pub fn std_set_is_empty<X: Ord>(s: &std_Set<X>) -> bool {
        s.x.is_empty()
    }
    
    pub fn std_set_nth<X: Ord + Clone>(s: &std_Set<X>, n: &u64) -> std_Option<X> {
        option2std(s.x.iter().nth(*n as usize).cloned())
    }
    
    pub fn std_set2vec<X: Ord + Clone>(s: &std_Set<X>) -> std_Vec<X> {
        std_Vec{x: s.x.iter().cloned().collect()}
    }
    
    pub fn std_set_union<X: Ord + Clone>(s1: &std_Set<X>, s2: &std_Set<X>) -> std_Set<X> {
        let mut s = s1.clone();
        s.x.append(&mut s2.x.clone());
        s
    }
    
    pub fn std_set_unions<X: Ord + Clone>(sets: &std_Vec<std_Set<X>>) -> std_Set<X> {
        let mut s = BTreeSet::new();
        for si in sets.x.iter() {
            s.append(&mut si.x.clone());
        };
        std_Set{x: s}
    }
    
    
    // Map
    
    #[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize, Debug, Default)]
    pub struct std_Map<K: Ord,V> {
        pub x: BTreeMap<K,V>
    }
    
    /* This is needed so we can support for-loops over `Map`'s
     */
    pub struct MapIter<'a, K, V> {
        iter: btree_map::Iter<'a, K, V>
    }
    
    impl<'a, K: Ord, V> MapIter<'a, K, V> {
        pub fn new(map: &'a std_Map<K, V>) -> MapIter<'a, K, V> {
            MapIter{iter: map.x.iter()}
        }
    }
    
    impl<'a, K, V> Iterator for MapIter<'a, K, V> {
        type Item = (&'a K, &'a V);
    
        fn next(&mut self) -> Option<Self::Item> {
            self.iter.next()
        }
    
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.iter.size_hint()
        }
    }
    
    impl <'a, K: Ord, V> std_Map<K, V> {
        pub fn iter(&'a self) -> MapIter<'a, K, V> {
            MapIter::new(self)
        }
    }
    
    impl <K: Ord, V> std_Map<K,V> {
        pub fn new() -> Self {
            std_Map{x: BTreeMap::new()}
        }
        pub fn insert(&mut self, k: K, v: V) {
            self.x.insert(k,v);
        }
    }
    
    impl<K: FromRecord+Ord, V: FromRecord> FromRecord for std_Map<K,V> {
        fn from_record(val: &Record) -> Result<Self, String> {
            BTreeMap::from_record(val).map(|x|std_Map{x})
        }
    }
    
    impl<K: IntoRecord + Ord, V: IntoRecord> IntoRecord for std_Map<K,V> {
        fn into_record(self) -> Record {
            self.x.into_record()
        }
    }
    
    impl<K: FromRecord + Ord, V: FromRecord + PartialEq> Mutator<std_Map<K,V>> for Record
    {
        fn mutate(&self, map: &mut std_Map<K,V>) -> Result<(), String> {
            self.mutate(&mut map.x)
        }
    }
    
    impl<K: Ord,V> IntoIterator for std_Map<K,V> {
        type Item = (K,V);
        type IntoIter = btree_map::IntoIter<K,V>;
        fn into_iter(self) -> Self::IntoIter {
            self.x.into_iter()
        }
    }
    
    impl<K: Ord, V> FromIterator<(K,V)> for std_Map<K,V> {
        fn from_iter<I>(iter: I) -> Self
        where I: IntoIterator<Item = (K,V)>
        {
            std_Map{x: BTreeMap::from_iter(iter)}
        }
    }
    
    impl<K: Display+Ord, V: Display> Display for std_Map<K,V> {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            let len = self.x.len();
            formatter.write_str("[")?;
            for (i, (k,v)) in self.x.iter().enumerate() {
                formatter.write_fmt(format_args!("({},{})", *k, *v))?;
                if i < len-1 {
                    formatter.write_str(",")?;
                }
            }
            formatter.write_str("]")?;
            Ok(())
        }
    }
    
    pub fn std_map_empty<K: Ord + Clone,V: Clone>() -> std_Map<K, V> {
        std_Map::new()
    }
    
    pub fn std_map_singleton<K: Ord + Clone,V: Clone>(k: &K, v: &V) -> std_Map<K, V> {
        let mut m = std_Map::new();
        m.insert(k.clone(), v.clone());
        m
    }
    
    pub fn std_map_insert<K: Ord+Clone, V: Clone>(m: &mut std_Map<K,V>, k: &K, v: &V) {
        m.x.insert((*k).clone(), (*v).clone());
    }
    
    pub fn std_map_remove<K: Ord+Clone, V: Clone>(m: &mut std_Map<K,V>, k: &K) {
        m.x.remove(k);
    }
    
    pub fn std_map_insert_imm<K: Ord+Clone, V: Clone>(m: &std_Map<K,V>, k: &K, v: &V) -> std_Map<K,V> {
        let mut m2 = m.clone();
        m2.insert((*k).clone(), (*v).clone());
        m2
    }
    
    pub fn std_map_get<K: Ord, V: Clone>(m: &std_Map<K,V>, k: &K) -> std_Option<V> {
        option2std(m.x.get(k).cloned())
    }
    
    pub fn std_map_contains_key<K: Ord, V: Clone>(s: &std_Map<K,V>, k: &K) -> bool {
        s.x.contains_key(k)
    }
    
    pub fn std_map_is_empty<K: Ord, V: Clone>(m: &std_Map<K,V>) -> bool {
        m.x.is_empty()
    }
    
    pub fn std_map_union<K: Ord + Clone,V: Clone>(m1: &std_Map<K,V>, m2: &std_Map<K,V>) -> std_Map<K, V> {
        let mut m = m1.clone();
        m.x.append(&mut m2.x.clone());
        m
    }
    
    
    // strings
    
    pub fn std___builtin_2string<T: Display>(x: &T) -> String {
        format!("{}", *x).to_string()
    }
    
    pub fn std_hex<T: fmt::LowerHex>(x: &T) -> String {
        format!("{:x}", *x).to_string()
    }
    
    pub fn std_parse_dec_u64(s: &String) -> std_Option<u64> {
        option2std(s.parse::<u64>().ok())
    }
    
    pub fn std_parse_dec_i64(s: &String) -> std_Option<i64> {
        option2std(s.parse::<i64>().ok())
    }
    
    pub fn std_string_join(strings: &std_Vec<String>, sep: &String) -> String {
        strings.x.join(sep.as_str())
    }
    
    pub fn std_string_split(s: &String, sep: &String) -> std_Vec<String> {
        std_Vec{x: s.split(sep).map(|x| x.to_owned()).collect()}
    }
    
    pub fn std_string_contains(s1: &String, s2: &String) -> bool {
        s1.contains(s2.as_str())
    }
    
    pub fn std_string_substr(s: &String, start: &u64, end: &u64) -> String {
        let len = s.len();
        let from = cmp::min(*start as usize, len);
        let to = cmp::max(from, cmp::min(*end as usize, len));
        s[from..to].to_string()
    }
    
    pub fn std_string_len(s: &String) -> u64 {
        s.len() as u64
    }
    
    pub fn std_str_to_lower(s: &String) -> String {
        s.to_lowercase()
    }
    
    // Hashing
    
    pub fn std_hash64<T: Hash>(x: &T) -> u64 {
        let mut hasher = XxHash::with_seed(XX_SEED1);
        x.hash(&mut hasher);
        hasher.finish()
    }
    
    pub fn std_hash128<T: Hash>(x: &T) -> u128 {
        let mut hasher = XxHash::with_seed(XX_SEED1);
        x.hash(&mut hasher);
        let w1 = hasher.finish();
        let mut hasher = XxHash::with_seed(XX_SEED2);
        x.hash(&mut hasher);
        let w2 = hasher.finish();
        ((w1 as u128) << 64) | (w2 as u128)
    }
    
    pub type ProjectFunc<X> = fn(&Value) -> X;
    
    /*
     * Group type (used in aggregation operators)
     */
    pub struct std_Group<'a, X> {
        /* TODO: remove "pub" */
        pub group: &'a [(&'a Value, Weight)],
        pub project: &'a ProjectFunc<X>
    }
    
    /* This is needed so we can support for-loops over `Group`'s
     */
    pub struct GroupIter<'a, X> {
        iter: slice::Iter<'a, (&'a Value, Weight)>,
        project: &'a ProjectFunc<X>
    }
    
    impl<'a, X> GroupIter<'a, X> {
        pub fn new(grp: &std_Group<'a, X>) -> GroupIter<'a, X> {
            GroupIter{iter: grp.group.iter(), project: grp.project}
        }
    }
    
    impl<'a, X> Iterator for GroupIter<'a, X> {
        type Item = X;
    
        fn next(&mut self) -> Option<Self::Item> {
            match self.iter.next() {
                None => None,
                Some((x,_)) => Some((self.project)(x))
            }
        }
    
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.iter.size_hint()
        }
    }
    
    impl<'a, X> std_Group<'a, X> {
        pub fn new(group: &'a[(&'a Value, Weight)], project: &'static ProjectFunc<X>) -> std_Group<'a, X> {
            std_Group{group: group, project: project}
        }
    
        fn size(&self) -> u64 {
            self.group.len() as u64
        }
    
        fn first(&'a self) -> X {
            (self.project)(self.group[0].0)
        }
    
        fn nth_unchecked(&'a self, n: u64) -> X {
            (self.project)(self.group[n as usize].0)
        }
    
        pub fn iter(&'a self) -> GroupIter<'a, X> {
            GroupIter::new(self)
        }
    }
    
    impl<'a, X> std_Group<'a, X> {
        fn nth(&'a self, n: u64) -> std_Option<X> {
            if self.size() > n {
                std_Option::std_Some{x: (self.project)(self.group[n as usize].0)}
            } else {
                std_Option::std_None
            }
        }
    }
    
    /*
     * Standard aggregation function
     */
    pub fn std_group_count<A>(g: &std_Group<A>) -> u64 {
        g.size()
    }
    
    pub fn std_group_first<A>(g: &std_Group<A>) -> A {
        g.first()
    }
    
    pub fn std_group_nth<A>(g: &std_Group<A>, n: &u64) -> std_Option<A> {
        g.nth(*n)
    }
    
    pub fn std_group2set<A: Ord + Clone>(g: &std_Group<A>) -> std_Set<A> {
        let mut res = std_Set::new();
        for v in g.iter() {
            std_set_insert(&mut res, &v);
        };
        res
    }
    
    pub fn std_group_set_unions<A: Ord + Clone>(g: &std_Group<std_Set<A>>) -> std_Set<A>
    {
        let mut res = std_Set::new();
        for gr in g.iter() {
            for v in gr.iter() {
               std_set_insert(&mut res, v);
            }
        };
        res
    }
    
    pub fn std_group_setref_unions<A: Ord + Clone>(g: &std_Group<std_Ref<std_Set<A>>>)
        -> std_Ref<std_Set<A>>
    {
        if g.size() == 1 {
            g.first()
        } else {
            let mut res: std_Ref<std_Set<A>> = std_ref_new(&std_Set::new());
            {
                let mut rres = std_Ref::get_mut(&mut res).unwrap();
                for gr in g.iter() {
                    for v in gr.iter() {
                        std_set_insert(&mut rres, &v);
                    }
                };
            }
            res
        }
    }
    
    pub fn std_group2vec<A: Ord + Clone>(g: &std_Group<A>) -> std_Vec<A>
    {
        let mut res = std_Vec::with_capacity(g.size() as usize);
        for v in g.iter() {
            std_vec_push(&mut res, &v);
        };
        res
    }
    
    pub fn std_group2map<K: Ord + Clone, V: Clone>(g: &std_Group<(K,V)>) -> std_Map<K,V>
    {
        let mut res = std_Map::new();
        for (k, v) in g.iter() {
            std_map_insert(&mut res, &k, &v);
        };
        res
    }
    
    pub fn std_group_min<A: Ord>(g: &std_Group<A>) -> A {
        g.iter().min().unwrap()
    }
    
    pub fn std_group_max<A: Ord>(g: &std_Group<A>) -> A {
        g.iter().max().unwrap()
    }
    
    pub fn std_group_sum<A: ops::Add + ops::AddAssign>(g: &std_Group<A>) -> A {
        let mut res = std_group_first(g);
        for v in g.iter().skip(1) {
            res += v;
        };
        res
    }
    
    /* Tuples */
    #[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
    pub struct tuple0;
    
    impl FromRecord for tuple0 {
        fn from_record(val: &Record) -> Result<Self, String> {
            <()>::from_record(val).map(|_|tuple0)
        }
    }
    
    impl IntoRecord for tuple0 {
        fn into_record(self) -> Record {
            ().into_record()
        }
    }
    
    macro_rules! decl_tuple {
        ( $name:ident, $( $t:tt ),+ ) => {
            #[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
            pub struct $name< $($t),* >($(pub $t),*);
            impl <$($t: FromRecord),*> FromRecord for $name<$($t),*> {
                fn from_record(val: &Record) -> Result<Self, String> {
                    <($($t),*)>::from_record(val).map(|($($t),*)|$name($($t),*))
                }
            }
    
            impl <$($t: IntoRecord),*> IntoRecord for $name<$($t),*> {
                fn into_record(self) -> Record {
                    let $name($($t),*) = self;
                    Record::Tuple(vec![$($t.into_record()),*])
                }
            }
    
            impl <$($t: FromRecord),*> Mutator<$name<$($t),*>> for Record {
                fn mutate(&self, x: &mut $name<$($t),*>) -> Result<(), String> {
                    *x = <$name<$($t),*>>::from_record(self)?;
                    Ok(())
                }
            }
        };
    }
    
    decl_tuple!(tuple2,  T1, T2);
    decl_tuple!(tuple3,  T1, T2, T3);
    decl_tuple!(tuple4,  T1, T2, T3, T4);
    decl_tuple!(tuple5,  T1, T2, T3, T4, T5);
    decl_tuple!(tuple6,  T1, T2, T3, T4, T5, T6);
    decl_tuple!(tuple7,  T1, T2, T3, T4, T5, T6, T7);
    decl_tuple!(tuple8,  T1, T2, T3, T4, T5, T6, T7, T8);
    decl_tuple!(tuple9,  T1, T2, T3, T4, T5, T6, T7, T8, T9);
    decl_tuple!(tuple10, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    decl_tuple!(tuple11, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    decl_tuple!(tuple12, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    decl_tuple!(tuple13, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    decl_tuple!(tuple14, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    decl_tuple!(tuple15, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    decl_tuple!(tuple16, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    decl_tuple!(tuple17, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    decl_tuple!(tuple18, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    decl_tuple!(tuple19, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    decl_tuple!(tuple20, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    
    // Endianness
    pub fn std_ntohl(x: &u32) -> u32 {
        u32::from_be(*x)
    }
    
    pub fn std_ntohs(x: &u16) -> u16 {
        u16::from_be(*x)
    }
    
    pub fn std_htonl(x: &u32) -> u32 {
        u32::to_be(*x)
    }
    
    pub fn std_htons(x: &u16) -> u16 {
        u16::to_be(*x)
    }
}
pub use __log::*;
mod __log {
    use super::*;
    use std::sync;
    use std::collections;
    use std::ffi;
    
    type log_callback_t = Box<dyn Fn(log_log_level_t, &str) + Send + Sync>;
    
    lazy_static! {
        /* Logger configuration for each module consists of the maximal enabled
         * log level (messages above this level are ignored) and callback.
         */
        static ref LOG_CONFIG: sync::RwLock<collections::HashMap<log_module_t, (log_callback_t, log_log_level_t)>> = {
            sync::RwLock::new(collections::HashMap::new())
        };
    }
    
    /*
     * Logging API exposed to the DDlog program.
     * (see detailed documentation in `log.dl`)
     */
    pub fn log_log(module: &log_module_t, level: &log_log_level_t, msg: &String) -> bool
    {
        if let Some((cb, current_level)) = LOG_CONFIG.read().unwrap().get(&module) {
            if *level <= *current_level {
                cb(*level, msg.as_str());
            }
        };
        true
    }
    
    /*
     * Configuration API
     * (detailed documentation in `ddlog_log.h`)
     *
     * `cb = None` - disables logging for the given module.
     *
     * NOTE: we set callback and log level simultaneously.  A more flexible API
     * would allow changing log level without changing the callback.
     */
    pub fn log_set_callback(module: log_module_t, cb: Option<log_callback_t>, max_level: log_log_level_t)
    {
        match cb {
            Some(cb) => {
                LOG_CONFIG.write().unwrap().insert(module, (cb, max_level));
            },
            None => {
                LOG_CONFIG.write().unwrap().remove(&module);
            }
        }
    }
    
    /*
     * C bindings for the config API
     */
    #[no_mangle]
    pub unsafe extern "C" fn ddlog_log_set_callback(module: raw::c_int,
                                                    cb: Option<extern "C" fn(arg: libc::uintptr_t,
                                                                             level: raw::c_int,
                                                                             msg: *const raw::c_char)>,
                                                    cb_arg: libc::uintptr_t,
                                                    max_level: raw::c_int)
    {
        match cb {
            Some(cb) => {
                log_set_callback(module as log_module_t,
                                 Some(Box::new(move |level, msg| {
                                     cb(cb_arg,
                                        level as raw::c_int,
                                        ffi::CString::new(msg).unwrap_or_default().as_ptr())
                                 })),
                                 max_level as log_log_level_t)
            },
            None => {
                log_set_callback(module as log_module_t,
                                 None,
                                 max_level as log_log_level_t)
            }
        }
    }
}
pub use __graph::*;
mod __graph {
    use super::*;
    /* Functions and transformers for use in graph processing */
    
    use differential_dataflow::algorithms::graphs::propagate;
    use differential_dataflow::algorithms::graphs::scc;
    use differential_dataflow::collection::Collection;
    use differential_dataflow::operators::consolidate::Consolidate;
    use differential_dataflow::lattice::Lattice;
    use timely::dataflow::scopes::Scope;
    use std::mem;
    
    pub fn graph_SCC<S,V,E,N,EF,LF>(edges: &Collection<S,V,Weight>, _edges: EF,
                                    from: fn(&E) -> N,
                                    to:   fn(&E) -> N,
                                    _scclabels: LF) -> (Collection<S,V,Weight>)
    where
         S: Scope,
         S::Timestamp: Lattice + Ord,
         V: Val,
         N: Val,
         E: Val,
         EF: Fn(V) -> E + 'static,
         LF: Fn((N,N)) -> V + 'static
    {
        let pairs = edges.map(move |v| {
            let e = _edges(v);
            (from(&e), to(&e))
        });
    
        /* Recursively trim nodes without incoming and outgoing edges */
        let trimmed = scc::trim(&scc::trim(&pairs).map_in_place(|x| mem::swap(&mut x.0, &mut x.1)))
                      .map_in_place(|x| mem::swap(&mut x.0, &mut x.1));
        /* Edges that form cycles */
        let cycles = scc::strongly_connected(&trimmed);
        /* Initially each node is labeled by its own id */
        let nodes  = cycles.map_in_place(|x| x.0 = x.1.clone()).consolidate();
        /* Propagate smallest ID within SCC */
        let scclabels = propagate::propagate(&cycles, &nodes);
        scclabels.map(_scclabels)
    }
    
    pub fn graph_ConnectedComponents<S,V,E,N,EF,LF>(edges: &Collection<S,V,Weight>, _edges: EF,
                                                    from: fn(&E) -> N,
                                                    to:   fn(&E) -> N,
                                                    _cclabels: LF) -> (Collection<S,V,Weight>)
    where
         S: Scope,
         S::Timestamp: Lattice + Ord,
         V: Val,
         N: Val,
         E: Val,
         EF: Fn(V) -> E + 'static,
         LF: Fn((N,N)) -> V + 'static
    {
        let pairs = edges.map(move |v| {
            let e = _edges(v);
            (from(&e), to(&e))
        });
    
        /* Initially each node is labeled by its own id */
        let nodes  = pairs.map_in_place(|x| x.0 = x.1.clone()).consolidate();
        let labels = propagate::propagate(&pairs, &nodes);
        labels.map(_cclabels)
    }
}
type log_log_level_t = i32;
type log_module_t = i32;
#[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct redistributionspan_DdlogBinding {
    tn: redistributionspan_tnid_t,
    entity: redistributionspan_entid_t
}
impl Abomonation for redistributionspan_DdlogBinding{}
impl <> record::FromRecord for redistributionspan_DdlogBinding<> {
    fn from_record(val: &record::Record) -> Result<Self, String> {
        match val {
            record::Record::PosStruct(constr, args) => {
                match constr.as_ref() {
                    "redistributionspan.DdlogBinding" if args.len() == 2 => {
                        Ok(redistributionspan_DdlogBinding{tn: <redistributionspan_tnid_t>::from_record(&args[0])?, entity: <redistributionspan_entid_t>::from_record(&args[1])?})
                    },
                    c => Result::Err(format!("unknown constructor {} of type redistributionspan_DdlogBinding in {:?}", c, *val))
                }
            },
            record::Record::NamedStruct(constr, args) => {
                match constr.as_ref() {
                    "redistributionspan.DdlogBinding" => {
                        Ok(redistributionspan_DdlogBinding{tn: record::arg_extract::<redistributionspan_tnid_t>(args, "tn")?, entity: record::arg_extract::<redistributionspan_entid_t>(args, "entity")?})
                    },
                    c => Result::Err(format!("unknown constructor {} of type redistributionspan_DdlogBinding in {:?}", c, *val))
                }
            },
            v => {
                Result::Err(format!("not a struct {:?}", *v))
            }
        }
    }
}
decl_struct_into_record!(redistributionspan_DdlogBinding, <>, tn, entity);
decl_record_mutator_struct!(redistributionspan_DdlogBinding, <>, tn: redistributionspan_tnid_t, entity: redistributionspan_entid_t);
impl fmt::Display for redistributionspan_DdlogBinding {
    fn fmt(&self, __formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            redistributionspan_DdlogBinding{tn,entity} => {
                __formatter.write_str("redistributionspan.DdlogBinding{")?;
                fmt::Debug::fmt(tn, __formatter)?;
                __formatter.write_str(",")?;
                fmt::Debug::fmt(entity, __formatter)?;
                __formatter.write_str("}")
            }
        }
    }
}
impl fmt::Debug for redistributionspan_DdlogBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}
#[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct redistributionspan_DdlogDependency {
    parent: redistributionspan_entid_t,
    child: redistributionspan_entid_t
}
impl Abomonation for redistributionspan_DdlogDependency{}
impl <> record::FromRecord for redistributionspan_DdlogDependency<> {
    fn from_record(val: &record::Record) -> Result<Self, String> {
        match val {
            record::Record::PosStruct(constr, args) => {
                match constr.as_ref() {
                    "redistributionspan.DdlogDependency" if args.len() == 2 => {
                        Ok(redistributionspan_DdlogDependency{parent: <redistributionspan_entid_t>::from_record(&args[0])?, child: <redistributionspan_entid_t>::from_record(&args[1])?})
                    },
                    c => Result::Err(format!("unknown constructor {} of type redistributionspan_DdlogDependency in {:?}", c, *val))
                }
            },
            record::Record::NamedStruct(constr, args) => {
                match constr.as_ref() {
                    "redistributionspan.DdlogDependency" => {
                        Ok(redistributionspan_DdlogDependency{parent: record::arg_extract::<redistributionspan_entid_t>(args, "parent")?, child: record::arg_extract::<redistributionspan_entid_t>(args, "child")?})
                    },
                    c => Result::Err(format!("unknown constructor {} of type redistributionspan_DdlogDependency in {:?}", c, *val))
                }
            },
            v => {
                Result::Err(format!("not a struct {:?}", *v))
            }
        }
    }
}
decl_struct_into_record!(redistributionspan_DdlogDependency, <>, parent, child);
decl_record_mutator_struct!(redistributionspan_DdlogDependency, <>, parent: redistributionspan_entid_t, child: redistributionspan_entid_t);
impl fmt::Display for redistributionspan_DdlogDependency {
    fn fmt(&self, __formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            redistributionspan_DdlogDependency{parent,child} => {
                __formatter.write_str("redistributionspan.DdlogDependency{")?;
                fmt::Debug::fmt(parent, __formatter)?;
                __formatter.write_str(",")?;
                fmt::Debug::fmt(child, __formatter)?;
                __formatter.write_str("}")
            }
        }
    }
}
impl fmt::Debug for redistributionspan_DdlogDependency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}
#[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct redistributionspan_DdlogNode {
    id: redistributionspan_entid_t
}
impl Abomonation for redistributionspan_DdlogNode{}
impl <> record::FromRecord for redistributionspan_DdlogNode<> {
    fn from_record(val: &record::Record) -> Result<Self, String> {
        match val {
            record::Record::PosStruct(constr, args) => {
                match constr.as_ref() {
                    "redistributionspan.DdlogNode" if args.len() == 1 => {
                        Ok(redistributionspan_DdlogNode{id: <redistributionspan_entid_t>::from_record(&args[0])?})
                    },
                    c => Result::Err(format!("unknown constructor {} of type redistributionspan_DdlogNode in {:?}", c, *val))
                }
            },
            record::Record::NamedStruct(constr, args) => {
                match constr.as_ref() {
                    "redistributionspan.DdlogNode" => {
                        Ok(redistributionspan_DdlogNode{id: record::arg_extract::<redistributionspan_entid_t>(args, "id")?})
                    },
                    c => Result::Err(format!("unknown constructor {} of type redistributionspan_DdlogNode in {:?}", c, *val))
                }
            },
            v => {
                Result::Err(format!("not a struct {:?}", *v))
            }
        }
    }
}
decl_struct_into_record!(redistributionspan_DdlogNode, <>, id);
decl_record_mutator_struct!(redistributionspan_DdlogNode, <>, id: redistributionspan_entid_t);
impl fmt::Display for redistributionspan_DdlogNode {
    fn fmt(&self, __formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            redistributionspan_DdlogNode{id} => {
                __formatter.write_str("redistributionspan.DdlogNode{")?;
                fmt::Debug::fmt(id, __formatter)?;
                __formatter.write_str("}")
            }
        }
    }
}
impl fmt::Debug for redistributionspan_DdlogNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}
#[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct redistributionspan_Span {
    entity: redistributionspan_entid_t,
    tns: std_Set<redistributionspan_tnid_t>
}
impl Abomonation for redistributionspan_Span{}
impl <> record::FromRecord for redistributionspan_Span<> {
    fn from_record(val: &record::Record) -> Result<Self, String> {
        match val {
            record::Record::PosStruct(constr, args) => {
                match constr.as_ref() {
                    "redistributionspan.Span" if args.len() == 2 => {
                        Ok(redistributionspan_Span{entity: <redistributionspan_entid_t>::from_record(&args[0])?, tns: <std_Set<redistributionspan_tnid_t>>::from_record(&args[1])?})
                    },
                    c => Result::Err(format!("unknown constructor {} of type redistributionspan_Span in {:?}", c, *val))
                }
            },
            record::Record::NamedStruct(constr, args) => {
                match constr.as_ref() {
                    "redistributionspan.Span" => {
                        Ok(redistributionspan_Span{entity: record::arg_extract::<redistributionspan_entid_t>(args, "entity")?, tns: record::arg_extract::<std_Set<redistributionspan_tnid_t>>(args, "tns")?})
                    },
                    c => Result::Err(format!("unknown constructor {} of type redistributionspan_Span in {:?}", c, *val))
                }
            },
            v => {
                Result::Err(format!("not a struct {:?}", *v))
            }
        }
    }
}
decl_struct_into_record!(redistributionspan_Span, <>, entity, tns);
decl_record_mutator_struct!(redistributionspan_Span, <>, entity: redistributionspan_entid_t, tns: std_Set<redistributionspan_tnid_t>);
impl fmt::Display for redistributionspan_Span {
    fn fmt(&self, __formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            redistributionspan_Span{entity,tns} => {
                __formatter.write_str("redistributionspan.Span{")?;
                fmt::Debug::fmt(entity, __formatter)?;
                __formatter.write_str(",")?;
                fmt::Debug::fmt(tns, __formatter)?;
                __formatter.write_str("}")
            }
        }
    }
}
impl fmt::Debug for redistributionspan_Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}
type redistributionspan_entid_t = u32;
type redistributionspan_tnid_t = u16;
#[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum std_Either<A, B> {
    std_Left {
        l: A
    },
    std_Right {
        r: B
    }
}
impl <A: Val, B: Val> Abomonation for std_Either<A, B>{}
impl <A: record::FromRecord + Default,B: record::FromRecord + Default> record::FromRecord for std_Either<A,B> {
    fn from_record(val: &record::Record) -> Result<Self, String> {
        match val {
            record::Record::PosStruct(constr, args) => {
                match constr.as_ref() {
                    "std.Left" if args.len() == 1 => {
                        Ok(std_Either::std_Left{l: <A>::from_record(&args[0])?})
                    },
                    "std.Right" if args.len() == 1 => {
                        Ok(std_Either::std_Right{r: <B>::from_record(&args[0])?})
                    },
                    c => Result::Err(format!("unknown constructor {} of type std_Either in {:?}", c, *val))
                }
            },
            record::Record::NamedStruct(constr, args) => {
                match constr.as_ref() {
                    "std.Left" => {
                        Ok(std_Either::std_Left{l: record::arg_extract::<A>(args, "l")?})
                    },
                    "std.Right" => {
                        Ok(std_Either::std_Right{r: record::arg_extract::<B>(args, "r")?})
                    },
                    c => Result::Err(format!("unknown constructor {} of type std_Either in {:?}", c, *val))
                }
            },
            v => {
                Result::Err(format!("not a struct {:?}", *v))
            }
        }
    }
}
decl_enum_into_record!(std_Either, <A,B>, std_Left{l}, std_Right{r});
decl_record_mutator_enum!(std_Either, <A,B>, std_Left{l: A}, std_Right{r: B});
impl <A: fmt::Debug, B: fmt::Debug> fmt::Display for std_Either<A, B> {
    fn fmt(&self, __formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            std_Either::std_Left{l} => {
                __formatter.write_str("std.Left{")?;
                fmt::Debug::fmt(l, __formatter)?;
                __formatter.write_str("}")
            },
            std_Either::std_Right{r} => {
                __formatter.write_str("std.Right{")?;
                fmt::Debug::fmt(r, __formatter)?;
                __formatter.write_str("}")
            }
        }
    }
}
impl <A: fmt::Debug, B: fmt::Debug> fmt::Debug for std_Either<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}
impl  <A: Default, B: Default> Default for std_Either<A, B> {
    fn default() -> Self {
        std_Either::std_Left{l : Default::default()}
    }
}
#[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum std_Option<A> {
    std_Some {
        x: A
    },
    std_None
}
impl <A: Val> Abomonation for std_Option<A>{}
impl <A: record::FromRecord + Default> record::FromRecord for std_Option<A> {
    fn from_record(val: &record::Record) -> Result<Self, String> {
        match val {
            record::Record::PosStruct(constr, args) => {
                match constr.as_ref() {
                    "std.Some" if args.len() == 1 => {
                        Ok(std_Option::std_Some{x: <A>::from_record(&args[0])?})
                    },
                    "std.None" if args.len() == 0 => {
                        Ok(std_Option::std_None{})
                    },
                    c => Result::Err(format!("unknown constructor {} of type std_Option in {:?}", c, *val))
                }
            },
            record::Record::NamedStruct(constr, args) => {
                match constr.as_ref() {
                    "std.Some" => {
                        Ok(std_Option::std_Some{x: record::arg_extract::<A>(args, "x")?})
                    },
                    "std.None" => {
                        Ok(std_Option::std_None{})
                    },
                    c => Result::Err(format!("unknown constructor {} of type std_Option in {:?}", c, *val))
                }
            },
            v => {
                Result::Err(format!("not a struct {:?}", *v))
            }
        }
    }
}
decl_enum_into_record!(std_Option, <A>, std_Some{x}, std_None{});
decl_record_mutator_enum!(std_Option, <A>, std_Some{x: A}, std_None{});
impl <A: fmt::Debug> fmt::Display for std_Option<A> {
    fn fmt(&self, __formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            std_Option::std_Some{x} => {
                __formatter.write_str("std.Some{")?;
                fmt::Debug::fmt(x, __formatter)?;
                __formatter.write_str("}")
            },
            std_Option::std_None{} => {
                __formatter.write_str("std.None{")?;
                __formatter.write_str("}")
            }
        }
    }
}
impl <A: fmt::Debug> fmt::Debug for std_Option<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}
impl  <A: Default> Default for std_Option<A> {
    fn default() -> Self {
        std_Option::std_Some{x : Default::default()}
    }
}
pub fn relname2id(rname: &str) -> Option<Relations> {
   match rname {
        "redistributionspan.DdlogBinding" => Some(Relations::redistributionspan_DdlogBinding),
        "redistributionspan.DdlogDependency" => Some(Relations::redistributionspan_DdlogDependency),
        "redistributionspan.DdlogNode" => Some(Relations::redistributionspan_DdlogNode),
        "redistributionspan.Span" => Some(Relations::redistributionspan_Span),
       _  => None
   }
}
pub fn output_relname_to_id(rname: &str) -> Option<Relations> {
   match rname {
        "redistributionspan.Span" => Some(Relations::redistributionspan_Span),
       _  => None
   }
}
pub fn input_relname_to_id(rname: &str) -> Option<Relations> {
   match rname {
        "redistributionspan.DdlogBinding" => Some(Relations::redistributionspan_DdlogBinding),
        "redistributionspan.DdlogDependency" => Some(Relations::redistributionspan_DdlogDependency),
        "redistributionspan.DdlogNode" => Some(Relations::redistributionspan_DdlogNode),
       _  => None
   }
}
pub fn relid2rel(rid: RelId) -> Option<Relations> {
   match rid {
        0 => Some(Relations::redistributionspan_DdlogBinding),
        1 => Some(Relations::redistributionspan_DdlogDependency),
        2 => Some(Relations::redistributionspan_DdlogNode),
        3 => Some(Relations::redistributionspan_Span),
       _  => None
   }
}
pub fn relid2name(rid: RelId) -> Option<&'static str> {
   match rid {
        0 => Some(&"redistributionspan.DdlogBinding"),
        1 => Some(&"redistributionspan.DdlogDependency"),
        2 => Some(&"redistributionspan.DdlogNode"),
        3 => Some(&"redistributionspan.Span"),
       _  => None
   }
}
lazy_static! {
    pub static ref RELIDMAP: FnvHashMap<Relations, &'static str> = {
        let mut m = FnvHashMap::default();
        m.insert(Relations::redistributionspan_DdlogBinding, "redistributionspan.DdlogBinding");
        m.insert(Relations::redistributionspan_DdlogDependency, "redistributionspan.DdlogDependency");
        m.insert(Relations::redistributionspan_DdlogNode, "redistributionspan.DdlogNode");
        m.insert(Relations::redistributionspan_Span, "redistributionspan.Span");
        m
   };
}
lazy_static! {
    pub static ref INPUT_RELIDMAP: FnvHashMap<Relations, &'static str> = {
        let mut m = FnvHashMap::default();
        m.insert(Relations::redistributionspan_DdlogBinding, "redistributionspan.DdlogBinding");
        m.insert(Relations::redistributionspan_DdlogDependency, "redistributionspan.DdlogDependency");
        m.insert(Relations::redistributionspan_DdlogNode, "redistributionspan.DdlogNode");
        m
    };
}
lazy_static! {
    pub static ref OUTPUT_RELIDMAP: FnvHashMap<Relations, &'static str> = {
        let mut m = FnvHashMap::default();
        m.insert(Relations::redistributionspan_Span, "redistributionspan.Span");
        m
    };
}
pub fn relval_from_record(rel: Relations, rec: &record::Record) -> Result<Value, String> {
    match rel {
        Relations::redistributionspan_DdlogBinding => {
            Ok(Value::redistributionspan_DdlogBinding(<redistributionspan_DdlogBinding>::from_record(rec)?))
        },
        Relations::redistributionspan_DdlogDependency => {
            Ok(Value::redistributionspan_DdlogDependency(<redistributionspan_DdlogDependency>::from_record(rec)?))
        },
        Relations::redistributionspan_DdlogNode => {
            Ok(Value::redistributionspan_DdlogNode(<redistributionspan_DdlogNode>::from_record(rec)?))
        },
        Relations::redistributionspan_Span => {
            Ok(Value::redistributionspan_Span(boxed::Box::new(<redistributionspan_Span>::from_record(rec)?)))
        }
    }
}
pub fn relkey_from_record(rel: Relations, rec: &record::Record) -> Result<Value, String> {
    match rel {
        _ => Err(format!("relation {:?} does not have a primary key", rel))
    }
}
#[derive(Copy,Clone,Debug,PartialEq,Eq,Hash)]
pub enum Relations {
    redistributionspan_DdlogBinding = 0,
    redistributionspan_DdlogDependency = 1,
    redistributionspan_DdlogNode = 2,
    redistributionspan_Span = 3
}
#[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
pub enum Value {
    bit32(u32),
    tuple0__(()),
    tuple2__bit32_bit16((u32, u16)),
    tuple2__bit32_bit32((u32, u32)),
    tuple2__bit32_std_Set__bit16(boxed::Box<(u32, std_Set<u16>)>),
    tuple3__bit32_std_Set__bit16_bit32(boxed::Box<(u32, std_Set<u16>, u32)>),
    tuple2__std_Set__bit16_bit32(boxed::Box<(std_Set<u16>, u32)>),
    redistributionspan_DdlogBinding(redistributionspan_DdlogBinding),
    redistributionspan_DdlogDependency(redistributionspan_DdlogDependency),
    redistributionspan_DdlogNode(redistributionspan_DdlogNode),
    redistributionspan_Span(boxed::Box<redistributionspan_Span>)
}
unsafe_abomonate!(Value);
impl Default for Value {
    fn default() -> Value {Value::tuple0__(())}
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::bit32 (v) => write!(f, "{:?}", *v),
            Value::tuple0__ (v) => write!(f, "{:?}", *v),
            Value::tuple2__bit32_bit16 (v) => write!(f, "{:?}", *v),
            Value::tuple2__bit32_bit32 (v) => write!(f, "{:?}", *v),
            Value::tuple2__bit32_std_Set__bit16 (v) => write!(f, "{:?}", *v),
            Value::tuple3__bit32_std_Set__bit16_bit32 (v) => write!(f, "{:?}", *v),
            Value::tuple2__std_Set__bit16_bit32 (v) => write!(f, "{:?}", *v),
            Value::redistributionspan_DdlogBinding (v) => write!(f, "{:?}", *v),
            Value::redistributionspan_DdlogDependency (v) => write!(f, "{:?}", *v),
            Value::redistributionspan_DdlogNode (v) => write!(f, "{:?}", *v),
            Value::redistributionspan_Span (v) => write!(f, "{:?}", *v)
        }
    }
}
decl_val_enum_into_record!(Value, <>, bit32(x), tuple0__(x), tuple2__bit32_bit16(x), tuple2__bit32_bit32(x), tuple2__bit32_std_Set__bit16(x), tuple3__bit32_std_Set__bit16_bit32(x), tuple2__std_Set__bit16_bit32(x), redistributionspan_DdlogBinding(x), redistributionspan_DdlogDependency(x), redistributionspan_DdlogNode(x), redistributionspan_Span(x));
decl_record_mutator_val_enum!(Value, <>, bit32(u32), tuple0__(()), tuple2__bit32_bit16((u32, u16)), tuple2__bit32_bit32((u32, u32)), tuple2__bit32_std_Set__bit16((u32, std_Set<u16>)), tuple3__bit32_std_Set__bit16_bit32((u32, std_Set<u16>, u32)), tuple2__std_Set__bit16_bit32((std_Set<u16>, u32)), redistributionspan_DdlogBinding(redistributionspan_DdlogBinding), redistributionspan_DdlogDependency(redistributionspan_DdlogDependency), redistributionspan_DdlogNode(redistributionspan_DdlogNode), redistributionspan_Span(redistributionspan_Span));
/* fn log_log(module: & log_module_t, level: & log_log_level_t, msg: & String) -> bool */
/* fn std___builtin_2string<X: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(x: & X) -> String */
/* fn std_deref<A: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(x: & std_Ref<A>) -> A */
/* fn std_group2map<K: Eq + Ord + Clone + Hash + PartialEq + PartialOrd,V: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(g: & std_Group<(K, V)>) -> std_Map<K, V> */
/* fn std_group2set<A: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(g: & std_Group<A>) -> std_Set<A> */
/* fn std_group2vec<A: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(g: & std_Group<A>) -> std_Vec<A> */
/* fn std_group_count<A: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(g: & std_Group<A>) -> u64 */
/* fn std_group_first<A: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(g: & std_Group<A>) -> A */
/* fn std_group_max<A: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(g: & std_Group<A>) -> A */
/* fn std_group_min<A: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(g: & std_Group<A>) -> A */
/* fn std_group_nth<A: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(g: & std_Group<A>, n: & u64) -> std_Option<A> */
/* fn std_group_set_unions<A: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(g: & std_Group<std_Set<A>>) -> std_Set<A> */
/* fn std_group_setref_unions<A: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(g: & std_Group<std_Ref<std_Set<A>>>) -> std_Ref<std_Set<A>> */
/* fn std_group_sum<A: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(g: & std_Group<A>) -> A */
/* fn std_hash128<X: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(x: & X) -> u128 */
/* fn std_hash64<X: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(x: & X) -> u64 */
/* fn std_hex<X: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(x: & X) -> String */
/* fn std_htonl(x: & u32) -> u32 */
/* fn std_htons(x: & u16) -> u16 */
/* fn std_map_contains_key<K: Eq + Ord + Clone + Hash + PartialEq + PartialOrd,V: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(m: & std_Map<K, V>, k: & K) -> bool */
/* fn std_map_empty<K: Eq + Ord + Clone + Hash + PartialEq + PartialOrd,V: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>() -> std_Map<K, V> */
/* fn std_map_get<K: Eq + Ord + Clone + Hash + PartialEq + PartialOrd,V: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(m: & std_Map<K, V>, k: & K) -> std_Option<V> */
/* fn std_map_insert<K: Eq + Ord + Clone + Hash + PartialEq + PartialOrd,V: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(m: &mut std_Map<K, V>, k: & K, v: & V) -> () */
/* fn std_map_insert_imm<K: Eq + Ord + Clone + Hash + PartialEq + PartialOrd,V: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(m: & std_Map<K, V>, k: & K, v: & V) -> std_Map<K, V> */
/* fn std_map_is_empty<K: Eq + Ord + Clone + Hash + PartialEq + PartialOrd,V: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(m: & std_Map<K, V>) -> bool */
/* fn std_map_remove<K: Eq + Ord + Clone + Hash + PartialEq + PartialOrd,V: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(m: &mut std_Map<K, V>, k: & K) -> () */
/* fn std_map_singleton<K: Eq + Ord + Clone + Hash + PartialEq + PartialOrd,V: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(k: & K, v: & V) -> std_Map<K, V> */
/* fn std_map_union<K: Eq + Ord + Clone + Hash + PartialEq + PartialOrd,V: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(m1: & std_Map<K, V>, m2: & std_Map<K, V>) -> std_Map<K, V> */
/* fn std_max<A: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(x: & A, y: & A) -> A */
/* fn std_min<A: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(x: & A, y: & A) -> A */
/* fn std_ntohl(x: & u32) -> u32 */
/* fn std_ntohs(x: & u16) -> u16 */
/* fn std_parse_dec_i64(s: & String) -> std_Option<i64> */
/* fn std_parse_dec_u64(s: & String) -> std_Option<u64> */
/* fn std_pow32<A: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(base: & A, exp: & u32) -> A */
/* fn std_range<A: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(from: & A, to: & A, step: & A) -> std_Vec<A> */
/* fn std_ref_new<A: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(x: & A) -> std_Ref<A> */
/* fn std_set2vec<A: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(s: & std_Set<A>) -> std_Vec<A> */
/* fn std_set_contains<X: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(s: & std_Set<X>, v: & X) -> bool */
/* fn std_set_empty<X: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>() -> std_Set<X> */
/* fn std_set_insert<X: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(s: &mut std_Set<X>, v: & X) -> () */
/* fn std_set_insert_imm<X: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(s: & std_Set<X>, v: & X) -> std_Set<X> */
/* fn std_set_is_empty<X: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(s: & std_Set<X>) -> bool */
/* fn std_set_nth<X: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(s: & std_Set<X>, n: & u64) -> std_Option<X> */
/* fn std_set_singleton<X: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(x: & X) -> std_Set<X> */
/* fn std_set_size<X: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(s: & std_Set<X>) -> u64 */
/* fn std_set_union<X: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(s1: & std_Set<X>, s2: & std_Set<X>) -> std_Set<X> */
/* fn std_set_unions<X: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(sets: & std_Vec<std_Set<X>>) -> std_Set<X> */
/* fn std_str_to_lower(s: & String) -> String */
/* fn std_string_contains(s1: & String, s2: & String) -> bool */
/* fn std_string_join(strings: & std_Vec<String>, sep: & String) -> String */
/* fn std_string_len(s: & String) -> u64 */
/* fn std_string_split(s: & String, sep: & String) -> std_Vec<String> */
/* fn std_string_substr(s: & String, start: & u64, end: & u64) -> String */
/* fn std_vec2set<A: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(s: & std_Vec<A>) -> std_Set<A> */
/* fn std_vec_contains<X: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(v: & std_Vec<X>, x: & X) -> bool */
/* fn std_vec_empty<A: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>() -> std_Vec<A> */
/* fn std_vec_is_empty<X: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(v: & std_Vec<X>) -> bool */
/* fn std_vec_len<X: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(v: & std_Vec<X>) -> u64 */
/* fn std_vec_nth<X: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(v: & std_Vec<X>, n: & u64) -> std_Option<X> */
/* fn std_vec_push<X: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(v: &mut std_Vec<X>, x: & X) -> () */
/* fn std_vec_push_imm<X: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(v: & std_Vec<X>, x: & X) -> std_Vec<X> */
/* fn std_vec_singleton<X: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(x: & X) -> std_Vec<X> */
fn std_group_unzip<X: Eq + Ord + Clone + Hash + PartialEq + PartialOrd,Y: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(g: & std_Group<(X, Y)>) -> (std_Vec<X>, std_Vec<Y>)
{   let ref mut xs : std_Vec<X> = std_vec_empty();
    let ref mut ys : std_Vec<Y> = std_vec_empty();
    for ref v in g.iter() {
        {
            let (ref mut x, ref mut y) = v.clone();
            std_vec_push(xs, x);
            std_vec_push(ys, y)
        }
    };
    (xs.clone(), ys.clone())
}
fn std_is_none<A: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(x: & std_Option<A>) -> bool
{   match (*x) {
        std_Option::std_None{} => true,
        _ => false
    }
}
fn std_is_some<A: Eq + Ord + Clone + Hash + PartialEq + PartialOrd>(x: & std_Option<A>) -> bool
{   match (*x) {
        std_Option::std_Some{x: _} => true,
        _ => false
    }
}
pub fn prog(__update_cb: Box<dyn CBFn<Value>>) -> Program<Value> {
    let redistributionspan_DdlogBinding = Relation {
                                              name:         "redistributionspan.DdlogBinding".to_string(),
                                              input:        true,
                                              distinct:     false,
                                              key_func:     None,
                                              id:           Relations::redistributionspan_DdlogBinding as RelId,
                                              rules:        vec![
                                                  ],
                                              arrangements: vec![
                                                  Arrangement::Map{
                                                     name: r###"redistributionspan.DdlogBinding{.tn=_, .entity=_0}"###.to_string(),
                                                      afun: &{fn __f(__v: Value) -> Option<(Value,Value)>
                                                      {
                                                          let __cloned = __v.clone();
                                                          {
                                                              if let Value::redistributionspan_DdlogBinding(__box) = __v {
                                                                  match __box {
                                                                  redistributionspan_DdlogBinding{tn: _, entity: ref _0} => Some(Value::bit32(_0.clone())),
                                                                  _ => None
                                                                  }
                                                              } else { None }
                                                          }.map(|x|(x,__cloned))
                                                      }
                                                      __f}
                                                  }],
                                              change_cb:    None
                                          };
    let redistributionspan_DdlogDependency = Relation {
                                                 name:         "redistributionspan.DdlogDependency".to_string(),
                                                 input:        true,
                                                 distinct:     false,
                                                 key_func:     None,
                                                 id:           Relations::redistributionspan_DdlogDependency as RelId,
                                                 rules:        vec![
                                                     ],
                                                 arrangements: vec![
                                                     Arrangement::Map{
                                                        name: r###"redistributionspan.DdlogDependency{.parent=_, .child=_0}"###.to_string(),
                                                         afun: &{fn __f(__v: Value) -> Option<(Value,Value)>
                                                         {
                                                             let __cloned = __v.clone();
                                                             {
                                                                 if let Value::redistributionspan_DdlogDependency(__box) = __v {
                                                                     match __box {
                                                                     redistributionspan_DdlogDependency{parent: _, child: ref _0} => Some(Value::bit32(_0.clone())),
                                                                     _ => None
                                                                     }
                                                                 } else { None }
                                                             }.map(|x|(x,__cloned))
                                                         }
                                                         __f}
                                                     }],
                                                 change_cb:    None
                                             };
    let redistributionspan_DdlogNode = Relation {
                                           name:         "redistributionspan.DdlogNode".to_string(),
                                           input:        true,
                                           distinct:     false,
                                           key_func:     None,
                                           id:           Relations::redistributionspan_DdlogNode as RelId,
                                           rules:        vec![
                                               ],
                                           arrangements: vec![
                                               Arrangement::Map{
                                                  name: r###"redistributionspan.DdlogNode{.id=_0}"###.to_string(),
                                                   afun: &{fn __f(__v: Value) -> Option<(Value,Value)>
                                                   {
                                                       let __cloned = __v.clone();
                                                       {
                                                           if let Value::redistributionspan_DdlogNode(__box) = __v {
                                                               match __box {
                                                               redistributionspan_DdlogNode{id: ref _0} => Some(Value::bit32(_0.clone())),
                                                               _ => None
                                                               }
                                                           } else { None }
                                                       }.map(|x|(x,__cloned))
                                                   }
                                                   __f}
                                               }],
                                           change_cb:    None
                                       };
    let redistributionspan_Span = Relation {
                                      name:         "redistributionspan.Span".to_string(),
                                      input:        false,
                                      distinct:     false,
                                      key_func:     None,
                                      id:           Relations::redistributionspan_Span as RelId,
                                      rules:        vec![
                                          /* redistributionspan.Span(.entity=(entity: redistributionspan.entid_t), .tns=bindings) :- redistributionspan.DdlogNode(.id=entity), redistributionspan.DdlogBinding(.tn=tn, .entity=entity), var bindings = Aggregate((entity), std.group2set(tn)). */
                                          Rule::ArrangementRule {
                                              description: "redistributionspan.Span(.entity=(entity: redistributionspan.entid_t), .tns=bindings) :- redistributionspan.DdlogNode(.id=entity), redistributionspan.DdlogBinding(.tn=tn, .entity=entity), var bindings = Aggregate((entity), std.group2set(tn)).".to_string(),
                                              arr: ( Relations::redistributionspan_DdlogNode as RelId, 0),
                                              xform: XFormArrangement::Join{
                                                         description: "redistributionspan.DdlogNode(.id=entity), redistributionspan.DdlogBinding(.tn=tn, .entity=entity)".to_string(),
                                                         ffun: None,
                                                         arrangement: (Relations::redistributionspan_DdlogBinding as RelId,0),
                                                         jfun: &{fn __f(_: &Value ,__v1: &Value,__v2: &Value) -> Option<Value>
                                                         {
                                                             let entity = match *__v1{
                                                                 Value::redistributionspan_DdlogNode(ref __box) => {
                                                                     match *__box {
                                                                     redistributionspan_DdlogNode{id: ref entity} => entity,
                                                                     _ => return None
                                                                     }
                                                                 },
                                                                 _ => return None
                                                             };
                                                             let tn = match *__v2{
                                                                 Value::redistributionspan_DdlogBinding(ref __box) => {
                                                                     match *__box {
                                                                     redistributionspan_DdlogBinding{tn: ref tn, entity: _} => tn,
                                                                     _ => return None
                                                                     }
                                                                 },
                                                                 _ => return None
                                                             };
                                                             Some(Value::tuple2__bit32_bit16((entity.clone(), tn.clone())))
                                                         }
                                                         __f},
                                                         next: Box::new(Some(XFormCollection::Arrange {
                                                                                 description: "arrange redistributionspan.DdlogNode(.id=entity), redistributionspan.DdlogBinding(.tn=tn, .entity=entity) by (entity)" .to_string(),
                                                                                 afun: &{fn __f(__v: Value) -> Option<(Value,Value)>
                                                                                 {
                                                                                     let (entity, tn) = match __v {
                                                                                         Value::tuple2__bit32_bit16(ref __box) => {
                                                                                             match *__box {
                                                                                                 (ref entity, ref tn) => (entity, tn),
                                                                                                 _ => unreachable!(),
                                                                                             }
                                                                                         },
                                                                                         _ => unreachable!()
                                                                                     };
                                                                                     Some((Value::bit32(entity.clone()), Value::tuple2__bit32_bit16((entity.clone(), tn.clone()))))
                                                                                 }
                                                                                 __f},
                                                                                 next: Box::new(XFormArrangement::Aggregate{
                                                                                                    description: "redistributionspan.DdlogNode(.id=entity), redistributionspan.DdlogBinding(.tn=tn, .entity=entity), var bindings = Aggregate((entity), std.group2set(tn))".to_string(),
                                                                                                    ffun: None,
                                                                                                    aggfun: &{fn __f(__key: &Value, group: &[(&Value, Weight)]) -> Value
                                                                                                {
                                                                                                    let entity = match *__key {
                                                                                                        Value::bit32(ref __box) => {
                                                                                                            match *__box {
                                                                                                                ref entity => entity,
                                                                                                                _ => unreachable!(),
                                                                                                            }
                                                                                                        },
                                                                                                        _ => unreachable!()
                                                                                                    };
                                                                                                    let bindings = std_group2set::<u16>(&std_Group::new(group, &{fn __f(__v: &Value) ->  redistributionspan_tnid_t
                                                                                                                                                               {
                                                                                                                                                                   let (entity, tn) = match __v {
                                                                                                                                                                       Value::tuple2__bit32_bit16(ref __box) => {
                                                                                                                                                                           match *__box {
                                                                                                                                                                               (ref entity, ref tn) => (entity, tn),
                                                                                                                                                                               _ => unreachable!(),
                                                                                                                                                                           }
                                                                                                                                                                       },
                                                                                                                                                                       _ => unreachable!()
                                                                                                                                                                   };
                                                                                                                                                                   tn.clone()
                                                                                                                                                               }
                                                                                                                                                               __f}));
                                                                                                    Value::tuple2__std_Set__bit16_bit32(boxed::Box::new((bindings.clone(), entity.clone())))
                                                                                                }
                                                                                                __f},
                                                                                                    next: Box::new(Some(XFormCollection::FilterMap{
                                                                                                                            description: "head of redistributionspan.Span(.entity=(entity: redistributionspan.entid_t), .tns=bindings) :- redistributionspan.DdlogNode(.id=entity), redistributionspan.DdlogBinding(.tn=tn, .entity=entity), var bindings = Aggregate((entity), std.group2set(tn))." .to_string(),
                                                                                                                            fmfun: &{fn __f(__v: Value) -> Option<Value>
                                                                                                                            {
                                                                                                                                let (bindings, entity) = match __v {
                                                                                                                                    Value::tuple2__std_Set__bit16_bit32(ref __box) => {
                                                                                                                                        match **__box {
                                                                                                                                            (ref bindings, ref entity) => (bindings, entity),
                                                                                                                                            _ => unreachable!(),
                                                                                                                                        }
                                                                                                                                    },
                                                                                                                                    _ => unreachable!()
                                                                                                                                };
                                                                                                                                Some(Value::redistributionspan_Span(boxed::Box::new(redistributionspan_Span{entity: entity.clone(), tns: bindings.clone()})))
                                                                                                                            }
                                                                                                                            __f},
                                                                                                                            next: Box::new(None)
                                                                                                                        }))
                                                                                                })
                                                                             }))
                                                     }
                                          },
                                          /* redistributionspan.Span(.entity=parent, .tns=tns) :- redistributionspan.DdlogNode(.id=parent), redistributionspan.DdlogDependency(.parent=child, .child=parent), redistributionspan.Span(.entity=child, .tns=child_tns), var tns = Aggregate((parent), std.group_set_unions(child_tns)). */
                                          Rule::ArrangementRule {
                                              description: "redistributionspan.Span(.entity=parent, .tns=tns) :- redistributionspan.DdlogNode(.id=parent), redistributionspan.DdlogDependency(.parent=child, .child=parent), redistributionspan.Span(.entity=child, .tns=child_tns), var tns = Aggregate((parent), std.group_set_unions(child_tns)).".to_string(),
                                              arr: ( Relations::redistributionspan_DdlogNode as RelId, 0),
                                              xform: XFormArrangement::Join{
                                                         description: "redistributionspan.DdlogNode(.id=parent), redistributionspan.DdlogDependency(.parent=child, .child=parent)".to_string(),
                                                         ffun: None,
                                                         arrangement: (Relations::redistributionspan_DdlogDependency as RelId,0),
                                                         jfun: &{fn __f(_: &Value ,__v1: &Value,__v2: &Value) -> Option<Value>
                                                         {
                                                             let parent = match *__v1{
                                                                 Value::redistributionspan_DdlogNode(ref __box) => {
                                                                     match *__box {
                                                                     redistributionspan_DdlogNode{id: ref parent} => parent,
                                                                     _ => return None
                                                                     }
                                                                 },
                                                                 _ => return None
                                                             };
                                                             let child = match *__v2{
                                                                 Value::redistributionspan_DdlogDependency(ref __box) => {
                                                                     match *__box {
                                                                     redistributionspan_DdlogDependency{parent: ref child, child: _} => child,
                                                                     _ => return None
                                                                     }
                                                                 },
                                                                 _ => return None
                                                             };
                                                             Some(Value::tuple2__bit32_bit32((child.clone(), parent.clone())))
                                                         }
                                                         __f},
                                                         next: Box::new(Some(XFormCollection::Arrange {
                                                                                 description: "arrange redistributionspan.DdlogNode(.id=parent), redistributionspan.DdlogDependency(.parent=child, .child=parent) by (child)" .to_string(),
                                                                                 afun: &{fn __f(__v: Value) -> Option<(Value,Value)>
                                                                                 {
                                                                                     let (child, parent) = match __v {
                                                                                         Value::tuple2__bit32_bit32(ref __box) => {
                                                                                             match *__box {
                                                                                                 (ref child, ref parent) => (child, parent),
                                                                                                 _ => unreachable!(),
                                                                                             }
                                                                                         },
                                                                                         _ => unreachable!()
                                                                                     };
                                                                                     Some((Value::bit32(child.clone()), Value::tuple2__bit32_bit32((child.clone(), parent.clone()))))
                                                                                 }
                                                                                 __f},
                                                                                 next: Box::new(XFormArrangement::Join{
                                                                                                    description: "redistributionspan.DdlogNode(.id=parent), redistributionspan.DdlogDependency(.parent=child, .child=parent), redistributionspan.Span(.entity=child, .tns=child_tns)".to_string(),
                                                                                                    ffun: None,
                                                                                                    arrangement: (Relations::redistributionspan_Span as RelId,0),
                                                                                                    jfun: &{fn __f(_: &Value ,__v1: &Value,__v2: &Value) -> Option<Value>
                                                                                                    {
                                                                                                        let (child, parent) = match *__v1 {
                                                                                                            Value::tuple2__bit32_bit32(ref __box) => {
                                                                                                                match *__box {
                                                                                                                    (ref child, ref parent) => (child, parent),
                                                                                                                    _ => unreachable!(),
                                                                                                                }
                                                                                                            },
                                                                                                            _ => unreachable!()
                                                                                                        };
                                                                                                        let child_tns = match *__v2{
                                                                                                            Value::redistributionspan_Span(ref __box) => {
                                                                                                                match **__box {
                                                                                                                redistributionspan_Span{entity: _, tns: ref child_tns} => child_tns,
                                                                                                                _ => return None
                                                                                                                }
                                                                                                            },
                                                                                                            _ => return None
                                                                                                        };
                                                                                                        Some(Value::tuple3__bit32_std_Set__bit16_bit32(boxed::Box::new((child.clone(), child_tns.clone(), parent.clone()))))
                                                                                                    }
                                                                                                    __f},
                                                                                                    next: Box::new(Some(XFormCollection::Arrange {
                                                                                                                            description: "arrange redistributionspan.DdlogNode(.id=parent), redistributionspan.DdlogDependency(.parent=child, .child=parent), redistributionspan.Span(.entity=child, .tns=child_tns) by (parent)" .to_string(),
                                                                                                                            afun: &{fn __f(__v: Value) -> Option<(Value,Value)>
                                                                                                                            {
                                                                                                                                let (child, child_tns, parent) = match __v {
                                                                                                                                    Value::tuple3__bit32_std_Set__bit16_bit32(ref __box) => {
                                                                                                                                        match **__box {
                                                                                                                                            (ref child, ref child_tns, ref parent) => (child, child_tns, parent),
                                                                                                                                            _ => unreachable!(),
                                                                                                                                        }
                                                                                                                                    },
                                                                                                                                    _ => unreachable!()
                                                                                                                                };
                                                                                                                                Some((Value::bit32(parent.clone()), Value::tuple3__bit32_std_Set__bit16_bit32(boxed::Box::new((child.clone(), child_tns.clone(), parent.clone())))))
                                                                                                                            }
                                                                                                                            __f},
                                                                                                                            next: Box::new(XFormArrangement::Aggregate{
                                                                                                                                               description: "redistributionspan.DdlogNode(.id=parent), redistributionspan.DdlogDependency(.parent=child, .child=parent), redistributionspan.Span(.entity=child, .tns=child_tns), var tns = Aggregate((parent), std.group_set_unions(child_tns))".to_string(),
                                                                                                                                               ffun: None,
                                                                                                                                               aggfun: &{fn __f(__key: &Value, group: &[(&Value, Weight)]) -> Value
                                                                                                                                           {
                                                                                                                                               let parent = match *__key {
                                                                                                                                                   Value::bit32(ref __box) => {
                                                                                                                                                       match *__box {
                                                                                                                                                           ref parent => parent,
                                                                                                                                                           _ => unreachable!(),
                                                                                                                                                       }
                                                                                                                                                   },
                                                                                                                                                   _ => unreachable!()
                                                                                                                                               };
                                                                                                                                               let tns = std_group_set_unions::<u16>(&std_Group::new(group, &{fn __f(__v: &Value) ->  std_Set<redistributionspan_tnid_t>
                                                                                                                                                                                                            {
                                                                                                                                                                                                                let (child, child_tns, parent) = match __v {
                                                                                                                                                                                                                    Value::tuple3__bit32_std_Set__bit16_bit32(ref __box) => {
                                                                                                                                                                                                                        match **__box {
                                                                                                                                                                                                                            (ref child, ref child_tns, ref parent) => (child, child_tns, parent),
                                                                                                                                                                                                                            _ => unreachable!(),
                                                                                                                                                                                                                        }
                                                                                                                                                                                                                    },
                                                                                                                                                                                                                    _ => unreachable!()
                                                                                                                                                                                                                };
                                                                                                                                                                                                                child_tns.clone()
                                                                                                                                                                                                            }
                                                                                                                                                                                                            __f}));
                                                                                                                                               Value::tuple2__bit32_std_Set__bit16(boxed::Box::new((parent.clone(), tns.clone())))
                                                                                                                                           }
                                                                                                                                           __f},
                                                                                                                                               next: Box::new(Some(XFormCollection::FilterMap{
                                                                                                                                                                       description: "head of redistributionspan.Span(.entity=parent, .tns=tns) :- redistributionspan.DdlogNode(.id=parent), redistributionspan.DdlogDependency(.parent=child, .child=parent), redistributionspan.Span(.entity=child, .tns=child_tns), var tns = Aggregate((parent), std.group_set_unions(child_tns))." .to_string(),
                                                                                                                                                                       fmfun: &{fn __f(__v: Value) -> Option<Value>
                                                                                                                                                                       {
                                                                                                                                                                           let (parent, tns) = match __v {
                                                                                                                                                                               Value::tuple2__bit32_std_Set__bit16(ref __box) => {
                                                                                                                                                                                   match **__box {
                                                                                                                                                                                       (ref parent, ref tns) => (parent, tns),
                                                                                                                                                                                       _ => unreachable!(),
                                                                                                                                                                                   }
                                                                                                                                                                               },
                                                                                                                                                                               _ => unreachable!()
                                                                                                                                                                           };
                                                                                                                                                                           Some(Value::redistributionspan_Span(boxed::Box::new(redistributionspan_Span{entity: parent.clone(), tns: tns.clone()})))
                                                                                                                                                                       }
                                                                                                                                                                       __f},
                                                                                                                                                                       next: Box::new(None)
                                                                                                                                                                   }))
                                                                                                                                           })
                                                                                                                        }))
                                                                                                })
                                                                             }))
                                                     }
                                          }],
                                      arrangements: vec![
                                          Arrangement::Map{
                                             name: r###"redistributionspan.Span{.entity=_0, .tns=_}"###.to_string(),
                                              afun: &{fn __f(__v: Value) -> Option<(Value,Value)>
                                              {
                                                  let __cloned = __v.clone();
                                                  {
                                                      if let Value::redistributionspan_Span(__box) = __v {
                                                          match *__box {
                                                          redistributionspan_Span{entity: ref _0, tns: _} => Some(Value::bit32(_0.clone())),
                                                          _ => None
                                                          }
                                                      } else { None }
                                                  }.map(|x|(x,__cloned))
                                              }
                                              __f}
                                          }],
                                      change_cb:    Some(sync::Arc::new(sync::Mutex::new(__update_cb.clone())))
                                  };
    Program {
        nodes: vec![
            ProgNode::Rel{rel: redistributionspan_DdlogBinding},
            ProgNode::Rel{rel: redistributionspan_DdlogDependency},
            ProgNode::Rel{rel: redistributionspan_DdlogNode},
            ProgNode::SCC{rels: vec![redistributionspan_Span]}
        ],
        init_data: vec![
        ]
    }
}