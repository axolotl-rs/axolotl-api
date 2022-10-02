use std::fmt::Debug;

use rand::Rng;
use serde_json::Value;

use crate::game::Game;
use crate::world_gen::noise::density::builtin::one_param::OneArgBuiltInFunction;
use crate::world_gen::noise::density::builtin::two_param::TwoParamBuiltInFunction;
use crate::world_gen::noise::density::loading::{DensityLoader, FunctionArgument};
use crate::world_gen::noise::density::perlin::Perlin;
use crate::world_gen::noise::density::shift::NoiseFunctions;
use crate::world_gen::noise::Noise;

pub mod builtin;
pub mod cache;
mod clamp;
mod interpolated;
pub mod loading;
pub mod perlin;
mod shift;
pub mod spline;

pub enum BuildDefResult {
    InvalidFormat,
    DescriptiveError(&'static str),
    NotFound(FunctionArgument),
}

impl From<&'static str> for BuildDefResult {
    fn from(s: &'static str) -> Self {
        BuildDefResult::DescriptiveError(s)
    }
}

/// The Current Density State

pub trait DensityState {
    type Random: Rng;
    type Perlin: Perlin<Noise=Noise, Seed=[u8; 16]>;
    fn seed(&self) -> [u8; 16];

    fn get_random(&self) -> Self::Random;

    fn get_x(&self) -> i64;

    fn get_y(&self) -> i64;

    fn get_z(&self) -> i64;

    fn get_perlin(&self) -> &Self::Perlin;

    fn build_from_def<G: Game, P: Perlin<Noise=Noise, Seed=[u8; 16]>>(&self, game: &G, def: FunctionArgument) -> Function<P>;
}

/// The DensityFunction is a generic trait for all density functions.
///
/// You pass in a DensityState which contains all the functions and noises that are available.
///
pub trait DensityFunction<'function, P: Perlin<Noise=Noise, Seed=[u8; 16]>>: Debug + Clone {
    type FunctionDefinition;

    fn new<G, DS: DensityState<Perlin = P>>(game: &G, state: &'function DS, def: Self::FunctionDefinition) -> Self
        where
            G: Game;
    fn compute<State: DensityState>(&self, state: &State) -> f64;
    /// The maximum value that this function can return.
    fn max(&self) -> f64 {
        f64::MAX
    }
    /// The minimum value that this function can return.
    fn min(&self) -> f64 {
        f64::MIN
    }

    fn build_definition(
        value: FunctionArgument,
        _state: &mut impl DensityLoader,
    ) -> Result<Self::FunctionDefinition, BuildDefResult<>> {
        Err(BuildDefResult::NotFound(value))
    }
}

#[derive(Debug, Clone)]
pub struct Constant(f64);

/// A Function is a wrapper around a DensityFunction.
impl< P: Perlin<Noise=Noise, Seed=[u8; 16]>> DensityFunction<'_,P> for Constant {
    type FunctionDefinition = f64;

    fn new<G, DS: DensityState>(_: &G, _: &DS, def: Self::FunctionDefinition) -> Self {
        Self(def)
    }

    fn compute<State: DensityState>(&self, _: &State) -> f64 {
        self.0
    }
    fn max(&self) -> f64 {
        self.0
    }
    fn min(&self) -> f64 {
        self.0
    }
}

#[derive(Debug, Clone)]
pub enum Function<'function, P: Perlin<Noise=Noise, Seed=[u8; 16]>> {
    /// A constant value
    Constant(f64),
    Interpolated(Box<interpolated::Interpolated<P>>),
    Clamp(Box<clamp::Clamp<'function, P>>),
    OneParam(Box<OneArgBuiltInFunction<'function, P>>),
    TwoParam(Box<TwoParamBuiltInFunction<'function, P>>),
    AllInCellCache(Box<cache::all_in_cell::AllInCellCache<'function, P>>),
    FlatCache(Box<cache::flat::FlatCache<'function, P>>),
    TwoDCellCache(Box<cache::two_d::TwoDCache<'function, P>>),
    OnceCache(Box<cache::once::OnceCache<'function, P>>),
    Noise(NoiseFunctions<P>),
}

impl<'function, P: Perlin<Noise=Noise, Seed=[u8; 16]>> DensityFunction<'_,P> for Function<'function, P> {
    type FunctionDefinition = ();

    fn new<G, DS: DensityState>(game: &G, state: &DS, def: Self::FunctionDefinition) -> Self where G: Game {
        todo!()
    }


    #[inline]
    fn compute<State: DensityState>(&self, state: &State) -> f64 {
        match self {
            Function::Constant(fun) => *fun,
            Function::Interpolated(fun) => fun.compute(state),
            Function::OneParam(builtin) => builtin.compute(state),
            Function::TwoParam(builtin) => builtin.compute(state),
            Function::Clamp(fun) => fun.compute(state),
            Function::AllInCellCache(fun) => fun.compute(state),
            Function::FlatCache(fun) => fun.compute(state),
            Function::TwoDCellCache(fun) => fun.compute(state),
            Function::OnceCache(fun) => fun.compute(state),
            Function::Noise(value) => {
                value.compute(state)
            }
        }
    }
    #[inline]
    fn max(&self) -> f64 {
        match self {
            Function::Constant(fun) => *fun,
            Function::Interpolated(fun) => fun.max(),
            Function::OneParam(builtin) => builtin.max(),
            Function::TwoParam(builtin) => builtin.max(),
            Function::Clamp(fun) => fun.max(),
            Function::AllInCellCache(fun) => fun.max(),
            Function::FlatCache(fun) => fun.max(),
            Function::TwoDCellCache(fun) => fun.max(),
            Function::OnceCache(fun) => fun.max(),
            Function::Noise(value) => {
                value.max()
            }
        }
    }
    #[inline]
    fn min(&self) -> f64 {
        match self {
            Function::Constant(fun) => *fun,
            Function::Interpolated(fun) => fun.min(),
            Function::OneParam(builtin) => builtin.min(),
            Function::TwoParam(builtin) => builtin.min(),
            Function::Clamp(fun) => fun.min(),
            Function::AllInCellCache(fun) => fun.min(),
            Function::FlatCache(fun) => fun.min(),
            Function::TwoDCellCache(fun) => fun.min(),
            Function::OnceCache(fun) => fun.min(),
            Function::Noise(value) => {
                value.min()
            }
        }
    }
}
