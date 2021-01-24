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
            self.set_color(s_x,s_y,self.matrix[d_x][d_y]);
            self.set_color(d_x,d_y,temp_color);
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
    pub fn get_color(&mut self, x : usize, y : usize) -> RGB
    {
        let g_board_width = self.matrix.len();
        let g_board_height = self.matrix[0].len();
        if x >= g_board_width || y >= g_board_height
        {
            //Err("Coords out of range")
        }
        else
        {
            return self.matrix[x][y];
        }
        return RGB { r: 0x00, g: 0x00, b: 0x00 };
    }

    // Empties screen entirely and draws new data to it.
    pub fn flush(&mut self)
    {
        for i in 0..PIXEL_COUNT
        {
            self.ws.set_color(RGB::zero(),i as u32);
        }
        self.update_matrix();

        for y in 0..HEIGHT
        {
            for x in 0..WIDTH
            {
                self.set_color_in_buffer(x,y,self.matrix[x][y]);
            }
        }
        self.update_matrix();
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
        match y % 2 
        {
            0 => index += (y * WIDTH + x) as u32,
            1 => index += (y * WIDTH + 15 - x) as u32,
            _ => (),
        }
        self.ws.set_color(new_color, index)
    }

}