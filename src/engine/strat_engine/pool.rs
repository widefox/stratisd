// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;
use std::vec::Vec;

use uuid::Uuid;
use devicemapper::Device;

use engine::EngineError;
use engine::EngineResult;
use engine::ErrorEnum;
use engine::Pool;
use engine::Filesystem;
use engine::Dev;
use engine::RenameAction;

use super::blockdev::BlockDev;
use super::filesystem::StratFilesystem;
use super::util::resolve_devices;

use super::consts::*;

#[derive(Debug)]
pub struct StratPool {
    pub name: String,
    pub pool_uuid: Uuid,
    pub cache_devs: BTreeMap<Uuid, BlockDev>,
    pub block_devs: BTreeMap<Uuid, BlockDev>,
    pub filesystems: BTreeMap<String, StratFilesystem>,
    pub raid_level: u16,
}

impl StratPool {
    pub fn new(name: &str,
               devices: BTreeSet<Device>,
               raid_level: u16,
               force: bool)
               -> EngineResult<StratPool> {
        let pool_uuid = Uuid::new_v4();
        let bds = try!(BlockDev::initialize(&pool_uuid, devices, MIN_MDA_SIZE, force));

        Ok(StratPool {
            name: name.to_owned(),
            pool_uuid: pool_uuid,
            cache_devs: BTreeMap::new(),
            block_devs: bds,
            filesystems: BTreeMap::new(),
            raid_level: raid_level,
        })
    }
}

impl Pool for StratPool {
    fn create_filesystems<'a, 'b, 'c>(&'a mut self,
                                      _specs: &[(&'b str, &'c str, Option<u64>)])
                                      -> EngineResult<Vec<&'b str>> {
        Ok(vec![])
    }

    fn create_snapshot<'a, 'b, 'c>(&'a mut self,
                                   _snapshot_name: &'b str,
                                   _source: &'c str)
                                   -> EngineResult<&'b str> {
        unimplemented!()
    }

    fn add_blockdevs(&mut self, paths: &[&Path], force: bool) -> EngineResult<Vec<PathBuf>> {
        let devices = try!(resolve_devices(paths));
        let mut bds = try!(BlockDev::initialize(&self.pool_uuid, devices, MIN_MDA_SIZE, force));
        let bdev_paths = bds.iter().map(|p| p.1.devnode.clone()).collect();
        self.block_devs.append(&mut bds);
        Ok(bdev_paths)
    }

    fn add_cachedevs(&mut self, paths: &[&Path], force: bool) -> EngineResult<Vec<PathBuf>> {
        let devices = try!(resolve_devices(paths));
        let mut bds = try!(BlockDev::initialize(&self.pool_uuid, devices, MIN_MDA_SIZE, force));
        let bdev_paths = bds.iter().map(|p| p.1.devnode.clone()).collect();
        self.cache_devs.append(&mut bds);
        Ok(bdev_paths)
    }

    fn destroy_filesystems<'a, 'b>(&'a mut self,
                                   fs_names: &[&'b str])
                                   -> EngineResult<Vec<&'b str>> {
        destroy_filesystems!{self; fs_names}
    }

    fn filesystems(&mut self) -> BTreeMap<&str, &mut Filesystem> {
        unimplemented!()
    }

    fn remove_blockdev(&mut self, _path: &Path) -> EngineResult<()> {
        unimplemented!()
    }

    fn remove_cachedev(&mut self, _path: &Path) -> EngineResult<()> {
        unimplemented!()
    }

    fn blockdevs(&mut self) -> Vec<&mut Dev> {
        unimplemented!()
    }

    fn cachedevs(&mut self) -> Vec<&mut Dev> {
        unimplemented!()
    }

    fn rename_filesystem(&mut self, old_name: &str, new_name: &str) -> EngineResult<RenameAction> {
        rename_filesystem!{self; old_name; new_name}
    }
}
