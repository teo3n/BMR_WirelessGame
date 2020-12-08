use crate::ws2812::{ Ws2812, RGB };
use embedded_hal::digital::v2::OutputPin;

pub const WIDTH: usize = 16;
pub const HEIGHT: usize = 16;
pub const PIXEL_COUNT: usize = WIDTH * HEIGHT;

pub struct Gameboard<'a, T>
{
    matrix: [[RGB; WIDTH]; HEIGHT],
    ws: &'a mut Ws2812<'a, T, PIXEL_COUNT>,
}

impl<'a, T> Gameboard<'a, T>
where T: OutputPin
{

    //initialize new Gameboard with given arguments
    pub fn new(ws: &'a mut Ws2812::<'a, T, PIXEL_COUNT>) -> Self
    {
        return Gameboard
        {
            matrix: [[RGB::zero(); WIDTH]; HEIGHT],
            ws
        };
    }

    // swaps colors between s_z, s_y and d_x, d_y if possible
    pub fn swap(&mut self, s_x : usize, s_y : usize, d_x : usize, d_y : usize)
    {
        let g_board_width = self.matrix.len();
        let g_board_height = self.matrix[0].len();
        if s_x >= g_board_width || d_x >= g_board_width || 
            s_y >= g_board_height || d_y >= g_board_height
        {
            //Err("Coords out of range")
        }
        else if s_x == d_x && s_y == d_y
        {
            //Ok(true)
        }
        else
        {
            let temp_color = self.matrix[s_x][s_y];
            self.matrix[s_x][s_y] = self.matrix[d_x][d_y];
            self.matrix[d_x][d_y] = temp_color;
            //Ok(true)
        }
    }

    pub fn set_color(&mut self, x : usize, y : usize, new_color : RGB)
    {
        let g_board_width = self.matrix.len();
        let g_board_height = self.matrix[0].len();
        if x >= g_board_width || y >= g_board_height
        {
            //Err("Coords out of range")
        }
        else
        {
            self.matrix[x][y] = new_color;
            self.set_color_in_buffer(x,y,new_color);
            //Ok(true)
        }
    }

    // TODO call this function with interrupt to provide appropriate 
    // refresh rate for the screen
    pub fn update_matrix(&mut self)
    {
        self.ws.write_leds()
    }
    // private methods

    fn set_color_in_buffer(&mut self, x: usize, y : usize, new_color : RGB)
    {
        let mut index: u32 = 0;
        match x % 2 
        {
            0 => index += (x * WIDTH + y) as u32,
            1 => index += (x * WIDTH + 15 - y) as u32,
            _ => (),
        }
        self.ws.set_color(new_color, index)
    }

    

}