// Copyright (C) 2016 Chris Liebert

// Frustum extracting and testing algorithm used from
// http://www.racer.nl/reference/vfc_markmorley.htm

#[derive(Debug)]
pub struct Frustum {
    // left right bottom top near and far planes
    planes: [[f32; 4]; 6],
}

impl Frustum {
    pub fn create(&self, modl: &[f32; 16], proj: &[f32; 16]) -> Frustum {
        // Combine the two matrices (multiply projection by modelview)
        let clip: [f32; 16] =
            [modl[0] * proj[0] + modl[1] * proj[4] + modl[2] * proj[8] + modl[3] * proj[12],
             modl[0] * proj[1] + modl[1] * proj[5] + modl[2] * proj[9] + modl[3] * proj[13],
             modl[0] * proj[2] + modl[1] * proj[6] + modl[2] * proj[10] + modl[3] * proj[14],
             modl[0] * proj[3] + modl[1] * proj[7] + modl[2] * proj[11] + modl[3] * proj[15],
             modl[4] * proj[0] + modl[5] * proj[4] + modl[6] * proj[8] + modl[7] * proj[12],
             modl[4] * proj[1] + modl[5] * proj[5] + modl[6] * proj[9] + modl[7] * proj[13],
             modl[4] * proj[2] + modl[5] * proj[6] + modl[6] * proj[10] + modl[7] * proj[14],
             modl[4] * proj[3] + modl[5] * proj[7] + modl[6] * proj[11] + modl[7] * proj[15],
             modl[8] * proj[0] + modl[9] * proj[4] + modl[10] * proj[8] + modl[11] * proj[12],
             modl[8] * proj[1] + modl[9] * proj[5] + modl[10] * proj[9] + modl[11] * proj[13],
             modl[8] * proj[2] + modl[9] * proj[6] + modl[10] * proj[10] + modl[11] * proj[14],
             modl[8] * proj[3] + modl[9] * proj[7] + modl[10] * proj[11] + modl[11] * proj[15],
             modl[12] * proj[0] + modl[13] * proj[4] + modl[14] * proj[8] + modl[15] * proj[12],
             modl[12] * proj[1] + modl[13] * proj[5] + modl[14] * proj[9] + modl[15] * proj[13],
             modl[12] * proj[2] + modl[13] * proj[6] + modl[14] * proj[10] + modl[15] * proj[14],
             modl[12] * proj[3] + modl[13] * proj[7] + modl[14] * proj[11] + modl[15] * proj[15]];

        // Extract the numbers for the RIGHT plane

        let rdx: f32 = clip[3] - clip[0];
        let rdy: f32 = clip[7] - clip[4];
        let rdz: f32 = clip[11] - clip[8];
        let rdw: f32 = clip[15] - clip[12];

        // Divisor to normilize right plane
        let rd: f32 = (rdx.powi(2) + rdy.powi(2) + rdz.powi(2)).sqrt();

        // Extract the numbers for the LEFT plane
        let ldx: f32 = clip[3] + clip[0];
        let ldy: f32 = clip[7] + clip[4];
        let ldz: f32 = clip[11] + clip[8];
        let ldw: f32 = clip[15] + clip[12];

        // Divisor to normilize left plane
        let ld: f32 = (ldx.powi(2) + ldy.powi(2) + ldz.powi(2)).sqrt();

        // Extract the BOTTOM plane */
        let bdx: f32 = clip[3] + clip[1];
        let bdy: f32 = clip[7] + clip[5];
        let bdz: f32 = clip[11] + clip[9];
        let bdw: f32 = clip[15] + clip[13];

        // Divisor to normilize bottom plane
        let bd: f32 = (bdx.powi(2) + bdy.powi(2) + bdz.powi(2)).sqrt();

        // Extract the TOP plane */
        let tdx: f32 = clip[3] - clip[1];
        let tdy: f32 = clip[7] - clip[5];
        let tdz: f32 = clip[11] - clip[9];
        let tdw: f32 = clip[15] - clip[13];

        // Divisor to normilize top plane
        let td: f32 = (tdx.powi(2) + tdy.powi(2) + tdz.powi(2)).sqrt();

        // Extract the FAR plane */
        let fdx: f32 = clip[3] - clip[2];
        let fdy: f32 = clip[7] - clip[6];
        let fdz: f32 = clip[11] - clip[10];
        let fdw: f32 = clip[15] - clip[14];

        // Divisor to normilize far plane
        let fd: f32 = (fdx.powi(2) + fdy.powi(2) + fdz.powi(2)).sqrt();

        // Extract the NEAR plane */
        let ndx: f32 = clip[3] + clip[2];
        let ndy: f32 = clip[7] + clip[6];
        let ndz: f32 = clip[11] + clip[10];
        let ndw: f32 = clip[15] + clip[14];

        // Divisor to normilize near plane
        let nd: f32 = (fdx.powi(2) + fdy.powi(2) + fdz.powi(2)).sqrt();

        Frustum {
            planes: [[rdx / rd, rdy / rd, rdz / rd, rdw / rd],
                     [ldx / ld, ldy / ld, ldz / ld, ldw / ld],
                     [bdx / bd, bdy / bd, bdz / bd, bdw / bd],
                     [tdx / td, tdy / td, tdz / td, tdw / td],
                     [fdx / fd, fdy / fd, fdz / fd, fdw / fd],
                     [ndx / nd, ndy / nd, ndz / nd, ndw / nd]],
        }
    }

    pub fn cube_intersecting(&self, x: f32, y: f32, z: f32, size: f32) -> bool {
        for p in 0..6 {
            if !((self.planes[p][0] * (x - size) + self.planes[p][1] * (y - size) + self.planes[p][2] * (z - size) + self.planes[p][3] > 0.0f32) ||
                 (self.planes[p][0] * (x + size) + self.planes[p][1] * (y - size) + self.planes[p][2] * (z - size) + self.planes[p][3] > 0.0f32) ||
                 (self.planes[p][0] * (x - size) + self.planes[p][1] * (y + size) + self.planes[p][2] * (z - size) + self.planes[p][3] > 0.0f32) ||
                 (self.planes[p][0] * (x + size) + self.planes[p][1] * (y + size) + self.planes[p][2] * (z - size) + self.planes[p][3] > 0.0f32) ||
                 (self.planes[p][0] * (x - size) + self.planes[p][1] * (y - size) + self.planes[p][2] * (z + size) + self.planes[p][3] > 0.0f32) ||
                 (self.planes[p][0] * (x + size) + self.planes[p][1] * (y - size) + self.planes[p][2] * (z + size) + self.planes[p][3] > 0.0f32) ||
                 (self.planes[p][0] * (x - size) + self.planes[p][1] * (y + size) + self.planes[p][2] * (z + size) + self.planes[p][3] > 0.0f32) ||
                 (self.planes[p][0] * (x + size) + self.planes[p][1] * (y + size) + self.planes[p][2] * (z + size) + self.planes[p][3] > 0.0f32)) {
                return false;
            }
        }
        true
    }

    pub fn point_intersecting(&self, x: f32, y: f32, z: f32) -> bool {
        for p in 0..6 {
            if self.planes[p][0] * x + self.planes[p][1] * y + self.planes[p][2] * z <= 0.0f32 {
                return false;
            }
        }
        true
    }

    pub fn sphere_intersecting(&self, x: f32, y: f32, z: f32, r: f32) -> bool {
        for p in 0..6 {
            if (self.planes[p][0] * x + self.planes[p][1] * y + self.planes[p][2] * z) <= -r {
                return false;
            }
        }
        true
    }
}
