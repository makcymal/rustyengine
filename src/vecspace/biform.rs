use {
    std::ops::Rem,
    crate::{
        globals::{
            BIFORM, Flt,
        },
        vecspace::{
            matrixified::{
                Matrixified,
                Matrix, Vector,
                common_matrix,
            },
            enums::MatrixType,
        },
    },
};


pub fn set_common_biform(m_type: MatrixType) {
    unsafe {
        BIFORM = common_matrix(m_type);
    }
}

pub fn set_biform_inner(inner: Vec<Vec<Flt>>) {
    unsafe {
        BIFORM = Matrix::from(inner);
    }
}

impl Rem for &Vector {
    type Output = Flt;

    fn rem(self, rhs: Self) -> Self::Output {
        let mut output;
        if self.size().is_vertical() {
            // if self is Col(n)
            unsafe {
                output = &BIFORM * self;
                // now output is Col(n)
            }
            output.transpose();
            // now output is Row(n)
        } else {
            // if self is Row(n)
            unsafe {
                output = self * &BIFORM;
                // now output is Row(n)
            }
        }

        if rhs.size().is_horizontal() {
            // if rhs is Row(n)
            output.transpose();
            (rhs * &output)[(0, 0)]
        } else {
            // if rhs is Col(n)
            (&output * rhs)[(0, 0)]
        }
    }
}
