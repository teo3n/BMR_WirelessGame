pub mod ws2812;
use ws2812::{ Ws2812, RGB };

pub const MAX_WIDHT: usize = 16;
pub const MAX_HEIGHT: usize = 16;

#[derive(Clone, Copy)]
pub struct gameboard<const WIDHT: usize, const HEIGHT : usize>
{
    matrix: [[RGB;WIDHT];HEIGHT],
    ws: &Ws2812,
}

impl<const WIDHT: usize, const HEIGHT : usize> gameboard<WIDHT,HEIGHT>{

    //initialize new gameboard with given arguments
    pub fn new(widht: usize, height: usize, ws: &Ws2812) -> Result<Self, E>
    {
        if widht > MAX_WIDHT
        {
            Err("Widht cannot be larger than MAX_WIDHT")
        } 
        else if height > MAX_HEIGHT
        {
            Err("Height cannot be larger than MAX_HEIGHT")
        }
        else
        {
            Ok(gameboard
                {matrix: [[RGB::zero():widht];height],
                ws: ws}
            )
        }
    }

    // swaps colors between s_z, s_y and d_x, d_y if possible
    pub fn swap(&mut self, s_x : usize, s_y : usize, d_x : usize, d_y : usize) -> Result<bool, E>
    {
        let g_board_width = self.matrix.len();
        let g_board_height = self.matrix[0].len();
        if s_x >= g_board_width || d_x >= g_board_width || 
            s_y >= g_board_height || d_y >= g_board_height
        {
            Err("Coords out of range")
        }
        else if s_x == d_x && s_y == d_y
        {
            Ok(true)
        }
        else
        {
            let temp_color = matrix[s_x][s_y];
            matrix[s_x][s_y] = matrix[d_x][d_y];
            matrix[d_x][d_y] = temp_color;
            Ok(true)
        }
    }

    pub fn set_color(&mut self, x : usize, y : usize, new_color : RGB) -> Result<bool, E>
    {
        let g_board_width = self.matrix.len();
        let g_board_height = self.matrix[0].len();
        if x >= g_board_width || y >= g_board_height
        {
            Err("Coords out of range")
        }
        else
        {
            matrix[x][y] = new_color;
            self.set_color_in_buffer(x,y,new_color);
            Ok(true)
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
            0 => index += (x*MAX_WIDHT + y) as u32,
            1 => index += (x*MAX_WIDHT + 15-y) u32,
            _ => (),
        }
        self.ws.set_color(new_color, index)
    }

    

}