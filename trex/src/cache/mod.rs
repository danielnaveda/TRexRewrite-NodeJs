pub mod ownable;
pub mod gdfs_cache;

use chrono::{Duration, UTC};
use lru_cache::LruCache;
use lru_size_cache::{HasSize as LruHasSize, LruSizeCache};
use owning_ref::{MutexGuardRef, MutexGuardRefMut};
use self::gdfs_cache::{GDSFCache, HasCost, HasSize};
use self::ownable::Ownable;
use std::borrow::Borrow;
use std::collections::hash_map::{Entry, HashMap, RandomState};
use std::hash::{BuildHasher, Hash, Hasher, SipHasher};
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = SipHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub trait Cache {
    type K;
    type V;
    fn store(&mut self, k: Self::K, v: Self::V) -> Result<&Self::V, Self::V>;
    fn fetch(&mut self, k: &Self::K) -> Option<&Self::V>;
    fn contains(&mut self, k: &Self::K) -> bool;
    fn remove(&mut self, k: &Self::K) -> Option<Self::V>;
}

pub trait Fetcher<K, V> {
    fn fetch(&self, &K) -> V;
}

pub enum FetchedValue<'a, C: Cache + ?Sized + 'a> {
    Cached(MutexGuardRef<'a, C, C::V>),
    Uncached(C::V),
}

impl<'a, C: Cache + ?Sized + 'a> Deref for FetchedValue<'a, C> {
    type Target = C::V;
    fn deref(&self) -> &Self::Target {
        match *self {
            FetchedValue::Cached(ref val) => val,
            FetchedValue::Uncached(ref val) => val,
        }
    }
}

#[derive(Default)]
pub struct HitMissCounter {
    hit: AtomicUsize,
    ind_hit: AtomicUsize,
    miss: AtomicUsize,
    last: AtomicUsize,
    hit_time: AtomicUsize,
    miss_time: AtomicUsize,
}

impl HitMissCounter {
    pub fn hit(&self, hash: u64, time: Duration) {
        if hash as usize != self.last.load(Ordering::SeqCst) {
            self.ind_hit.fetch_add(1, Ordering::SeqCst);
        }
        self.hit.fetch_add(1, Ordering::SeqCst);
        self.hit_time.fetch_add(time.num_nanoseconds().unwrap() as usize, Ordering::SeqCst);
        self.last.store(hash as usize, Ordering::SeqCst)
    }
    pub fn miss(&self, hash: u64, time: Duration) {
        self.miss.fetch_add(1, Ordering::SeqCst);
        self.miss_time.fetch_add(time.num_nanoseconds().unwrap() as usize, Ordering::SeqCst);
        self.last.store(hash as usize, Ordering::SeqCst)
    }
}

impl Drop for HitMissCounter {
    fn drop(&mut self) {
        let hit = self.hit.load(Ordering::SeqCst);
        let ind_hit = self.ind_hit.load(Ordering::SeqCst);
        let miss = self.miss.load(Ordering::SeqCst);
        let ratio = (miss as f32 / (hit + miss) as f32) * 100.0;
        let ind_ratio = (miss as f32 / (ind_hit + miss) as f32) * 100.0;
        let avg_hit_time = self.hit_time.load(Ordering::SeqCst) / (hit + 1);
        let avg_miss_time = self.miss_time.load(Ordering::SeqCst) / (miss + 1);
        println!("Fetcher stats: {} hits, {} ind_hit, {} miss ({}%, {}%), avg hit/miss time {}ns \
                  {}ns",
                 hit,
                 ind_hit,
                 miss,
                 ratio,
                 ind_ratio,
                 avg_hit_time,
                 avg_miss_time);
    }
}

pub struct CachedFetcher<C: ?Sized, F>
    where C: Cache,
          F: Fetcher<C::K, C::V>
{
    // TODO make member variables private
    pub cache: Arc<Mutex<C>>,
    pub fetcher: F,
    stat: Arc<HitMissCounter>,
}

impl<C, F> CachedFetcher<C, F>
    where C: Cache + Default,
          F: Fetcher<C::K, C::V>
{
    pub fn new(fetcher: F, stat: Arc<HitMissCounter>) -> Self {
        CachedFetcher {
            cache: Arc::default(),
            fetcher: fetcher,
            stat: stat,
        }
    }
}

impl<C: ?Sized, F> CachedFetcher<C, F>
    where C: Cache,
          F: Fetcher<C::K, C::V>
{
    pub fn with_cache(cache: Arc<Mutex<C>>, fetcher: F, stat: Arc<HitMissCounter>) -> Self {
        CachedFetcher {
            cache: cache,
            fetcher: fetcher,
            stat: stat,
        }
    }
}

impl<C: ?Sized, F> CachedFetcher<C, F>
    where C: Cache,
          F: Fetcher<C::K, C::V>
{
    pub fn fetch<Q>(&self, key: Q) -> FetchedValue<C>
        where Q: Borrow<C::K> + Ownable<C::K> + Hash
    {
        let start = UTC::now();
        let mut cache = MutexGuardRefMut::new(self.cache.lock().unwrap());
        if cache.contains(key.borrow()) {
            self.stat.hit(hash(&key), UTC::now() - start);
            FetchedValue::Cached(cache.map(|cache| cache.fetch(key.borrow()).unwrap()).into())
        } else {
            drop(cache);
            let value = self.fetcher.fetch(key.borrow());
            self.stat.miss(hash(&key), UTC::now() - start);
            MutexGuardRefMut::new(self.cache.lock().unwrap())
                .try_map(|cache| cache.store(key.into_owned(), value))
                .map(FetchedValue::Cached)
                .unwrap_or_else(FetchedValue::Uncached)
        }
    }
}

#[derive(Clone, Debug)]
pub struct DummyCache<K, V>(PhantomData<K>, PhantomData<V>);

impl<K, V> Cache for DummyCache<K, V> {
    type K = K;
    type V = V;
    fn store(&mut self, _: Self::K, v: Self::V) -> Result<&Self::V, Self::V> { Err(v) }
    fn fetch(&mut self, _: &Self::K) -> Option<&Self::V> { None }
    fn contains(&mut self, _: &Self::K) -> bool { false }
    fn remove(&mut self, _: &Self::K) -> Option<Self::V> { None }
}

impl<K, V> Default for DummyCache<K, V> {
    fn default() -> Self { DummyCache(Default::default(), Default::default()) }
}

impl<K, V, S> Cache for HashMap<K, V, S>
    where K: Eq + Hash,
          S: BuildHasher
{
    type K = K;
    type V = V;
    fn store(&mut self, k: Self::K, v: Self::V) -> Result<&Self::V, Self::V> {
        match self.entry(k) {
            Entry::Occupied(mut entry) => {
                entry.insert(v);
                Ok(entry.into_mut())
            }
            Entry::Vacant(entry) => Ok(entry.insert(v)),
        }
    }
    fn fetch(&mut self, k: &Self::K) -> Option<&Self::V> { self.get(k) }
    fn contains(&mut self, k: &Self::K) -> bool { self.contains_key(k) }
    fn remove(&mut self, k: &Self::K) -> Option<Self::V> { HashMap::remove(self, k) }
}

impl<K, V, S> Cache for LruCache<K, V, S>
    where K: Eq + Hash,
          S: BuildHasher
{
    type K = K;
    type V = V;
    fn store(&mut self, k: Self::K, v: Self::V) -> Result<&Self::V, Self::V> {
        self.insert(k, v);
        Ok(self.iter_mut().next_back().unwrap().1)
    }
    fn fetch(&mut self, k: &Self::K) -> Option<&Self::V> { self.get_mut(k).map(|it| it as &_) }
    fn contains(&mut self, k: &Self::K) -> bool { self.contains_key(k) }
    fn remove(&mut self, k: &Self::K) -> Option<Self::V> { LruCache::remove(self, k) }
}

impl<K, V, S> Cache for LruSizeCache<K, V, S>
    where K: Eq + Hash,
          S: BuildHasher,
          V: LruHasSize
{
    type K = K;
    type V = V;
    fn store(&mut self, k: Self::K, v: Self::V) -> Result<&Self::V, Self::V> {
        self.insert(k, v);
        Ok(self.iter().next_back().unwrap().1)
    }
    fn fetch(&mut self, k: &Self::K) -> Option<&Self::V> { self.get(k) }
    fn contains(&mut self, k: &Self::K) -> bool { self.contains_key(k) }
    fn remove(&mut self, k: &Self::K) -> Option<Self::V> { LruSizeCache::remove(self, k) }
}

pub struct ModHasher<H: Hasher> {
    hasher: H,
    modulus: u64,
}

impl<H: Hasher> Hasher for ModHasher<H> {
    fn finish(&self) -> u64 { self.hasher.finish() % self.modulus }
    fn write(&mut self, bytes: &[u8]) { self.hasher.write(bytes) }

    fn write_u8(&mut self, i: u8) { self.hasher.write_u8(i) }
    fn write_u16(&mut self, i: u16) { self.hasher.write_u16(i) }
    fn write_u32(&mut self, i: u32) { self.hasher.write_u32(i) }
    fn write_u64(&mut self, i: u64) { self.hasher.write_u64(i) }
    fn write_usize(&mut self, i: usize) { self.hasher.write_usize(i) }
    fn write_i8(&mut self, i: i8) { self.hasher.write_i8(i) }
    fn write_i16(&mut self, i: i16) { self.hasher.write_i16(i) }
    fn write_i32(&mut self, i: i32) { self.hasher.write_i32(i) }
    fn write_i64(&mut self, i: i64) { self.hasher.write_i64(i) }
    fn write_isize(&mut self, i: isize) { self.hasher.write_isize(i) }
}

pub struct ModBuildHasher<S: BuildHasher = RandomState> {
    hash_builder: S,
    modulus: u64,
}

impl<S: BuildHasher + Default> ModBuildHasher<S> {
    pub fn new(modulus: u64) -> Self {
        ModBuildHasher {
            hash_builder: S::default(),
            modulus: modulus,
        }
    }
}

impl<S: BuildHasher> ModBuildHasher<S> {
    pub fn with_modulus_and_hasher(modulus: u64, hash_builder: S) -> Self {
        ModBuildHasher {
            hash_builder: hash_builder,
            modulus: modulus,
        }
    }
}

impl<S: BuildHasher> BuildHasher for ModBuildHasher<S> {
    type Hasher = ModHasher<S::Hasher>;
    fn build_hasher(&self) -> Self::Hasher {
        ModHasher {
            hasher: self.hash_builder.build_hasher(),
            modulus: self.modulus,
        }
    }
}

pub type CollisionCache<K, V, S = RandomState> = HashMap<K, V, ModBuildHasher<S>>;

impl<K, V, S> Cache for GDSFCache<K, V, S>
    where K: Eq + Hash,
          V: HasSize + HasCost,
          S: BuildHasher
{
    type K = K;
    type V = V;
    fn store(&mut self, k: Self::K, v: Self::V) -> Result<&Self::V, Self::V> { self.insert(k, v) }
    fn fetch(&mut self, k: &Self::K) -> Option<&Self::V> { self.get(k) }
    fn contains(&mut self, k: &Self::K) -> bool { self.contains_key(k) }
    fn remove(&mut self, k: &Self::K) -> Option<Self::V> { GDSFCache::remove(self, k) }
}
