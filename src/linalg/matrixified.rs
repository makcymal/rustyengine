use {
    std::{
        ops::{
            Neg,
            Add, AddAssign,
            Sub, SubAssign,
            Mul, MulAssign,
            Div, DivAssign,
            BitXor, BitXorAssign,
        },
        iter::zip,
        fmt::{ Debug, Display },
    },
    num_traits::Num,
    super::enums::{
        MatrixifiedError::{self, *},
        MatrixLine::{self, *},
    },
};


#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Size {
    y: usize,
    x: usize,
}

impl Size {
    pub fn transpose(&mut self) {
        (self.y, self.x) = (self.x, self.y);
    }

    pub fn get_transposed(&self) -> Size {
        let mut size = self.clone();
        size.transpose();
        size
    }
}

impl From<(usize, usize)> for Size {
    fn from((y, x): (usize, usize)) -> Self {
        Self { y, x }
    }
}

type Pos = Size;
type Cache<T> = Option<T>;


pub trait Matrixified<'iter, T, const R: usize, const C: usize>
    where T: Num + Copy + Debug
{
    fn size(&self) -> Size;

    fn transpose(&mut self);

    fn row_iter(&'iter self, row: usize)
                -> Result<Box<dyn Iterator<Item=T> + 'iter>, MatrixifiedError>;
    fn col_iter(&'iter self, col: usize)
                -> Result<Box<dyn Iterator<Item=T> + 'iter>, MatrixifiedError>;

    fn row_iter_mut(&'iter self, row: usize)
                -> Result<Box<dyn Iterator<Item=&mut T> + 'iter>, MatrixifiedError>;

    fn check_pos(&self, pos: &Pos) -> Result<(), MatrixifiedError> {
        match (pos.y < 0, self.size().y <= pos.y, pos.x < 0, self.size().x <= pos.x) {
            (false, false, false, false) => Ok(()),
            (true, false, false, false) => Err(RowBelowAcceptable),
            (false, true, false, false) => Err(RowAboveAcceptable),
            (false, false, true, false) => Err(ColBelowAcceptable),
            (false, false, false, true) => Err(ColAboveAcceptable),
            _ => Err(InvalidIndex),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Matrix<T, const R: usize, const C: usize>
    where T: Num + Copy + Debug
{
    inner: [[T; C]; R],
    // default to false
    transposed: bool,
    determinant: Cache<T>,
    // change only within transpose() method
    // since transposed is false by default: (y, x) = (R, C)
    size: Size,
}

impl<T, const R: usize, const C: usize> Matrix<T, R, C>
    where T: Num + Copy + Debug
{
    pub fn zeros() -> Self {
        Matrix {
            inner: [[T::zero(); C]; R],
            transposed: false,
            determinant: None,
            size: (R, C).into(),
        }
    }

    pub fn ones() -> Result<Self, MatrixifiedError> {
        if R != C {
            Err(NonSquareMatrix)
        } else {
            let mut matrix = [[T::zero(); C]; R];
            for i in 0..R {
                matrix[i][i] = T::one();
            }
            Ok(Matrix {
                inner: matrix,
                transposed: false,
                determinant: None,
                size: (R, C).into(),
            })
        }
    }
}

impl<'iter, T, const R: usize, const C: usize> Matrixified<'iter, T, R, C> for Matrix<T, R, C>
    where T: Num + Copy + Debug
{
    fn size(&self) -> Size {
        self.size
    }

    fn transpose(&mut self) {
        self.transposed = !self.transposed;
        self.size.transpose();
    }

    fn row_iter(&'iter self, row: usize)
                -> Result<Box<dyn Iterator<Item=T> + 'iter>, MatrixifiedError>
    {
        let (pos, dir, iterations) = match self.transposed {
            false => ((row, 0).into(), Row, self.size.x),
            true => ((0, row).into(), Col, self.size.y),
        };
        if let Err(err) = self.check_pos(&pos) {
            Err(err)
        } else {
            Ok(Box::new(MatrixIter::<'iter, T, R, C>::new(self, pos, dir, iterations)))
        }
    }

    fn col_iter(&'iter self, col: usize)
                -> Result<Box<dyn Iterator<Item=T> + 'iter>, MatrixifiedError>
    {
        let (pos, dir, iterations) = match self.transposed {
            false => ((0, col).into(), Col, self.size.y),
            true => ((col, 0).into(), Row, self.size.x),
        };
        if let Err(err) = self.check_pos(&pos) {
            Err(err)
        } else {
            Ok(Box::new(MatrixIter::<'iter, T, R, C>::new(self, pos, dir, iterations)))
        }
    }

    fn row_iter_mut(&'iter self, row: usize) -> Result<Box<dyn Iterator<Item=&mut T> + 'iter>, MatrixifiedError> {
        todo!()
    }
}


#[derive(Debug)]
// it's guaranteed that iterator is always created with valid pos
// furthermore, next() calls can't invalidate pos
pub struct MatrixIter<'iter, T, const R: usize, const C: usize>
    where T: Num + Copy + Debug
{
    matrix: &'iter Matrix<T, R, C>,
    pos: Pos,
    dir: MatrixLine,
    // how many times next() can be called
    iterations: usize,
}

impl<'iter, T, const R: usize, const C: usize> MatrixIter<'iter, T, R, C>
    where T: Num + Copy + Debug
{
    fn new(matrix: &'iter Matrix<T, R, C>, pos: Pos, dir: MatrixLine, iterations: usize) -> Self {
        Self { matrix, pos, dir, iterations }
    }
}

impl<'iter, T, const R: usize, const C: usize> Iterator for MatrixIter<'iter, T, R, C>
    where T: Num + Copy + Debug
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // since matrix can't be borrowed mutably while iterator lives such a checks make sense
        if self.iterations == 0 {
            return None;
        }

        let item = self.matrix.inner[self.pos.y][self.pos.x];
        match self.dir {
            Row => self.pos.x += 1,
            Col => self.pos.y += 1,
        };

        // the only place where this counter changes
        self.iterations -= 1;
        Some(item)
    }
}


pub struct MatrixIterMut<'iter, T, const R: usize, const C: usize>
    where T: Num + Copy + Debug
{
    matrix: &'iter mut Matrix<T, R, C>,
    pos: Pos,
    dir: MatrixLine,
    // how many times next() can be called
    iterations: usize,
}

impl<'iter, T, const R: usize, const C: usize> MatrixIterMut<'iter, T, R, C>
    where T: Num + Copy + Debug
{
    fn new(matrix: &'iter mut Matrix<T, R, C>, pos: Pos, dir: MatrixLine, iterations: usize) -> Self {
        Self { matrix, pos, dir, iterations }
    }
}

impl<'iter, 'item, T, const R: usize, const C: usize> Iterator for MatrixIterMut<'iter, T, R, C>
    where T: Num + Copy + Debug
{
    type Item = &'iter mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // since matrix can't be borrowed mutably while iterator lives such a checks make sense
        if self.iterations == 0 {
            return None;
        }

        let item = self.matrix.inner[self.pos.y][self.pos.x].get_mut();
        match self.dir {
            Row => self.pos.x += 1,
            Col => self.pos.y += 1,
        };

        // the only place where this counter changes
        self.iterations -= 1;
        Some(item)
    }
}




#[derive(Debug, Clone)]
pub struct Vector<T, const L: usize>
    where T: Num + Copy + Debug
{
    inner: [T; L],
    // if false then it's row, else it's column
    // false by default
    transposed: bool,
    // when created equals to (1, L)
    size: Size,
}

impl<T, const L: usize> Vector<T, L>
    where T: Num + Copy + Debug
{
    pub fn zeros() -> Self {
        Vector {
            inner: [T::zero(); L],
            transposed: false,
            size: (1, L).into(),
        }
    }
}

impl<'iter, T, const L: usize> Matrixified<'iter, T, 1, L> for Vector<T, L>
    where T: Num + Copy + Debug
{
    fn size(&self) -> Size {
        self.size
    }

    fn transpose(&mut self) {
        self.transposed = !self.transposed;
        self.size.transpose();
    }

    fn row_iter(&'iter self, row: usize)
                -> Result<Box<dyn Iterator<Item=T> + 'iter>, MatrixifiedError>
    {
        let (pos, dir, iterations) = match self.transposed {
            false => ((row, 0).into(), Row, self.size.x),
            true => ((0, row).into(), Col, self.size.x),    // here self.size.x = 1
        };
        if let Err(err) = self.check_pos(&pos) {
            Err(err)
        } else {
            Ok(Box::new(VectorIter::<'iter, T, L>::new(self, pos, dir, iterations)))
        }
    }

    fn col_iter(&'iter self, col: usize)
                -> Result<Box<dyn Iterator<Item=T> + 'iter>, MatrixifiedError>
    {
        let (pos, dir, iterations) = match self.transposed {
            false => ((0, col).into(), Col, self.size.y),   // here self.size.y = 1
            true => ((col, 0).into(), Row, self.size.y),
        };
        if let Err(err) = self.check_pos(&pos) {
            Err(err)
        } else {
            Ok(Box::new(VectorIter::<'iter, T, L>::new(self, pos, dir, iterations)))
        }
    }

    fn row_iter_mut(&'iter self, row: usize) -> Result<Box<dyn Iterator<Item=&mut T> + 'iter>, MatrixifiedError> {
        todo!()
    }
}

#[derive(Debug)]
// it's guaranteed that iterator is always created with valid pos
// furthermore, next() calls can't invalidate pos
pub struct VectorIter<'iter, T, const L: usize>
    where T: Num + Copy + Debug
{
    vector: &'iter Vector<T, L>,
    pos: Pos,
    dir: MatrixLine,
    // how many times next() can be called
    iterations: usize,
}

impl<'iter, T, const L: usize> VectorIter<'iter, T, L>
    where T: Num + Copy + Debug
{
    fn new(vector: &'iter Vector<T, L>, pos: Pos, dir: MatrixLine, iterations: usize) -> Self {
        Self { vector, pos, dir, iterations }
    }
}

impl<'iter, T, const L: usize> Iterator for VectorIter<'iter, T, L>
    where T: Num + Copy + Debug
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // since matrix can't be borrowed mutably while iterator lives such a checks make sense
        if self.iterations == 0 {
            return None;
        }

        let item = self.vector.inner[self.pos.x];
        if self.dir == Row {
            self.pos.x += 1;
        }

        // the only place where this counter changes
        self.iterations -= 1;
        Some(item)
    }
}


#[inline(always)]
// returns whether dyn Matrixified was transposed relying on const generics and size() impl
fn is_transposed(original_size: Size, actual_size: Size) -> bool {
    assert!(original_size == actual_size || original_size == actual_size.get_transposed());
    if original_size == actual_size {
        false
    } else { true }
}

impl<'iter, T, const R: usize, const C: usize> Add for &'iter dyn Matrixified<'iter, T, R, C>
    where T: Num + Copy + Debug + 'static
{
    type Output = Result<Matrix<T, R, C>, MatrixifiedError>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.size() != rhs.size() {
            return Err(InappropriateSizes);
        }

        let mut sum = Matrix::<T, R, C>::zeros();
        if is_transposed((R, C).into(), self.size()) {
            sum.transpose();
        }

        for row in 0..sum.size().y {
            let mut lhs_iter = self.row_iter(row).unwrap();
            let mut rhs_iter = rhs.row_iter(row).unwrap();
            for (col, item) in zip(lhs_iter, rhs_iter).map(|(a, b)| a + b).enumerate() {
                sum.inner[row][col] = item;
            }
        }
        Ok(sum)
    }
}

// impl<'iter, T, const R: usize, const C: usize> Neg for &mut dyn Matrixified<'iter, T, R, C>
//     where T: Num + Copy + Debug + 'static
// {
//     type Output = Self;
//
//     fn neg(self) -> Self::Output {
//         for row in 0..self.size().y {
//             let mut row_iter = self.row_iter(row).unwrap();
//         }
//     }
// }
