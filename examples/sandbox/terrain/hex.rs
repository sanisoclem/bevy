use bevy::prelude::*;
use bevy::math::Mat2;


//const pixel2hex: Mat2 = Mat2::from_cols_array(&[3.0f32.sqrt()/3.0, 0.0, -1.0/3.0, 2.0/3.0]);


pub trait HexLayout<HexCoord, SpaceCoord> {
    //type HexCoordIterator: Iterator<Item=HexCoord>;

    // fn distance_step(&self, h1: HexCoord, h2: HexCoord) -> i32;
    // fn distance(&self, h1: HexCoord, h2: HexCoord) ->  f32;

    fn hex_to_space(&self, hex: HexCoord) -> SpaceCoord;
    fn space_to_hex(&self, space: SpaceCoord) -> HexCoord;
    //fn get_neighbors(&self, hex: HexCoord, distance: i32) ->  Self::HexCoordIterator;
    //fn get_ring(&self, center: HexCoord, distance: i32) ->  Self::HexCoordIterator;
}

pub struct CubeHexCoord(pub i32, pub i32, pub i32);

impl CubeHexCoord {
    pub fn from_axis_coord(q: i32, r: i32) -> Self {
        CubeHexCoord(q, r, -(q+r))
    }
}
impl Into<Vec2> for CubeHexCoord {
    fn into(self) -> Vec2 {
        Vec2::new(self.0 as f32, self.1 as f32)
    }
}

pub struct CubeHexLayout {
    space_origin: CubeHexCoord,
    size: f32
}
impl CubeHexLayout {
    pub fn hex_coord_from_fractional_coord(&self, frac: Vec2) -> CubeHexCoord {
        let x = frac.x();
        let y = frac.y();
        let z = -(frac.x() + frac.y());
        let mut rx = x.round();
        let mut ry = y.round();
        let mut rz = z.round();

        let x_diff = (rx - x).abs();
        let y_diff = (ry - y).abs();
        let z_diff = (rz - z).abs();

        if x_diff > y_diff && x_diff > z_diff {
            rx = -ry-rz;
        }
        else if  y_diff > z_diff {
            ry = -rx-rz;
        }
        else {
            rz = -rx-ry;
        }

        CubeHexCoord(rx as i32, ry as i32, rz as i32)
    }
}

impl HexLayout<CubeHexCoord, Vec2> for CubeHexLayout {
    //type HexCoordIterator;

    fn hex_to_space(&self, hex: CubeHexCoord) -> Vec2 {
        // TODO: calculate when space 0,0 is not hex 0,0
        let hex2pixel = Mat2::from_cols_array(&[3.0f32.sqrt(), 0.0, 3.0f32.sqrt()/2.0, 3.0/2.0]);
        hex2pixel.mul_vec2(hex.into()) * self.size
    }

    fn space_to_hex(&self, space: Vec2) -> CubeHexCoord {
        // TODO: calculate when space 0,0 is not hex 0,0
        let space2hex = Mat2::from_cols_array(&[3.0f32.sqrt()/3.0, 0.0, -1.0/3.0, 2.0/3.0]);
        let frac = space2hex.mul_vec2(space) / self.size;
        self.hex_coord_from_fractional_coord(frac)
    }

    // fn get_neighbors(&self, hex: CubeHexCoord, distance: i32) ->  Self::HexCoordIterator {
    //     todo!()
    // }

    // fn get_ring(&self, center: CubeHexCoord, distance: i32) ->  Self::HexCoordIterator {
    //     todo!()
    // }
}