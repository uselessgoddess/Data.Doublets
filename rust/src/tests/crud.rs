use bumpalo::Bump;
use rand::{random, thread_rng, Rng};
use std::alloc::{AllocError, Allocator, Global, Layout, System};
use std::default::default;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::ptr::{null_mut, NonNull};
use std::time::Instant;

use crate::doublets::data::{AddrToRaw, IGenericLinks, LinksConstants};
use crate::doublets::mem::splited;
use crate::doublets::{ILinks, ILinksExtensions, Link, LinksError};
//use crate::doublets::decorators::{CascadeUsagesResolver, NonNullDeletionResolver};
//use crate::doublets::mem::splited;
use crate::doublets::mem::united::Links;
use crate::mem::{AllocMem, FileMappedMem, HeapMem, Mem, ResizeableBase, ResizeableMem};
use crate::test_extensions::ILinksTestExtensions;
use crate::tests::make_links;
use crate::tests::make_mem;
/*
#[test]
fn random_creations_and_deletions() {
    std::fs::remove_file("db.links");

    let mem = make_mem();
    let mut links = make_links(mem).unwrap();
    let mut links = links.decorators_kit();

    let instant = Instant::now();
    links.test_random_creations_and_deletions(1000);

    println!("{:?}", instant.elapsed());
}
*/
#[test]
fn mapping() {
    std::fs::remove_file("mapping_file");
    let mut mem = FileMappedMem::new("mapping_file").unwrap();

    mem.reserve_mem(13370).unwrap();
    mem.reserve_mem(2280).unwrap();

    let mut file = std::fs::File::open("mapping_file").unwrap();
    assert_eq!(13370, file.metadata().unwrap().len() as usize);
}

#[test]
fn billion_points_file_mapped() {
    std::fs::remove_file("mem_bpfm");

    let mem = FileMappedMem::new("mem_bpfm").unwrap();
    let mut links = Links::<usize, _>::new(mem).unwrap();

    let instant = Instant::now();

    for _ in 0..1_000_000 {
        links.create_point().unwrap();
    }

    println!("{:?}", instant.elapsed());
}

#[test]
fn billion_points_heap_mem() -> Result<(), LinksError<usize>> {
    let mem = HeapMem::new()?;
    let mut links = Links::<usize, _>::new(mem)?;

    let instant = Instant::now();

    for _ in 0..1_000_000 {
        links.create_point()?;
    }

    println!("{:?}", instant.elapsed());
    Ok(())
}

#[test]
fn billion_points_bump_alloc() {
    let bump = Bump::new();
    let mem = AllocMem::new(&bump).unwrap();
    let mut links = Links::<usize, _>::new(mem).unwrap();

    let instant = Instant::now();

    for _ in 0..1_000_000 {
        let source = thread_rng().gen_range(1..=100);
        let target = thread_rng().gen_range(1..=100);
        let result: Result<_, LinksError<_>> = try {
            links.create_and_update(source, target)?;
        };
        if let Err(e) = result {
            //println!("{}", e);
        }
    }

    println!("{:?}", instant.elapsed());
}

#[test]
fn many_points_and_searches() {
    let bump = Bump::new();
    let mem = AllocMem::new(System).unwrap();
    let mut links = Links::<usize, _>::new(mem).unwrap();

    let instant = Instant::now();
    for _ in 0..1_000_000 {
        links.create_point().unwrap();
    }
    println!("{:?}", instant.elapsed());

    let instant = Instant::now();
    for i in 1..=1_000_000 {
        // links.search_or(i, i, 0);
        links.get_link(i);
    }
    println!("{:?}", instant.elapsed());
}

// TODO: Create `TempFileMappedMem`

#[test]
fn billion_points_file_mapped_splited() {
    std::fs::remove_file("data_bpfms");
    std::fs::remove_file("index_bpfms");

    let mem = FileMappedMem::new("data_bpfms").unwrap();
    let index = FileMappedMem::new("index_bpfms").unwrap();
    let mut links = splited::Links::<usize, _, _>::new(mem, index).unwrap();

    let instant = Instant::now();

    for _ in 0..1_000_000 {
        links.create_point().unwrap();
    }

    println!("{:?}", instant.elapsed());
}

#[test]
fn billion_points_heap_mem_splited() {
    let mem = HeapMem::new().unwrap();
    let index = HeapMem::new().unwrap();
    let mut links = splited::Links::<usize, _, _>::new(mem, index).unwrap();

    let instant = Instant::now();

    for _ in 0..1_000_000 {
        links.create_point().unwrap();
    }

    println!("{:?}", instant.elapsed());
}

#[test]
fn billion_points_bump_alloc_splited() {
    let bump = Bump::new();
    let mut mem = AllocMem::new(&bump).unwrap();
    let mut index = AllocMem::new(&bump).unwrap();
    index.reserve_mem(1023 * 1023).unwrap();
    let mut links = splited::Links::<usize, _, _>::new(mem, index).unwrap();

    println!("{}", links.get_header().reserved);

    let instant = Instant::now();

    for _ in 0..1_000_000 {
        links.create_point().unwrap();
    }

    println!("{}", links.get_header().reserved);
    println!("{}", 65535 * 2);

    println!("{:?}", instant.elapsed());
}

#[test]
fn many_points_and_searches_splited() {
    let mem = AllocMem::new(System).unwrap();
    let index = AllocMem::new(System).unwrap();
    let mut links = splited::Links::<usize, _, _>::new(mem, index).unwrap();

    let instant = Instant::now();
    for _ in 0..1_000_000 {
        links.create_point().unwrap();
    }
    println!("{:?}", instant.elapsed());

    let instant = Instant::now();
    for i in 1..=1_000_000 {
        links.search_or(i, i, 0);
    }
    println!("{:?}", instant.elapsed());
}

#[test]
fn playground() {
    std::fs::remove_file("data");
    std::fs::remove_file("index");

    let mem = FileMappedMem::new("data").unwrap();
    let index = FileMappedMem::new("index").unwrap();
    let mut links = splited::Links::<usize, _, _>::with_constants(
        mem,
        index,
        LinksConstants::via_only_external(true),
    )
    .unwrap();
    //let mut links = Links::<usize, _>::with_constants(mem, LinksConstants::via_only_external(true));
    //let mut links = CascadeUsagesResolver::new(links);

    links.create_point().unwrap();
    links.create_point().unwrap();
    links.create_point().unwrap();

    links.update(1, 1, 2).unwrap();
    links.update(2, 1, 3).unwrap();

    //links.delete_usages(1);
    links.delete(2).unwrap();
    links.delete(1).unwrap();

    println!("{}\n", links.is_unused(2));

    let get = |index| {
        println!("{:?}", links.get_data_part(index));
        println!("{:?}", links.get_index_part(index));
    };

    get(1);
    get(2);
    get(3);

    //return;

    links.each(|link| {
        println!("{}", link);
        println!("{:?}", links.get_link(link.index));
        println!("{:?}", links.exist(link.index));
        links.constants().r#continue
    });
}
