use {
    crate::{
        globals::{
            BIFORM,
        },
        vecspace::{
            matrixified::{
                Matrix, Vector, common_matrix
            },
            enums::MatrixType,
        },
    },
};


pub fn set_common_biform(m_type: MatrixType) {
    unsafe {
        BIFORM = Some(common_matrix(m_type));
    }
}


