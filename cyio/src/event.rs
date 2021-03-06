use cgmath::Vector2;
#[derive(PartialEq, Debug, Clone)]
pub enum Touch {
    None,
    One(Vector2<usize>),
    Two(Vector2<usize>, Vector2<usize>),
}

impl From<[u8; 16]> for Touch {
    fn from(event: [u8; 16]) -> Self {
        match event {
            [_, _, _, lx1, mx1, ly1, my1, _, _, _, _, 0x01, _, _, _, _] => Touch::One(
                (
                    utils::SCREEN_SIZE.x as usize
                        - ((lx1 as usize + ((mx1 as usize) << 8)) * utils::SCREEN_SIZE.x as usize)
                            / utils::TOUCH_SIZE.x as usize,
                    utils::SCREEN_SIZE.y as usize
                        - ((ly1 as usize + ((my1 as usize) << 8)) * utils::SCREEN_SIZE.y as usize)
                            / utils::TOUCH_SIZE.y as usize,
                )
                    .into(),
            ),
            [_, _, _, lx1, mx1, ly1, my1, lx2, mx2, ly2, my2, 0x02, _, _, _, _] => Touch::Two(
                (
                    utils::SCREEN_SIZE.x as usize
                        - ((lx1 as usize + ((mx1 as usize) << 8)) * utils::SCREEN_SIZE.x as usize)
                            / utils::TOUCH_SIZE.x as usize,
                    utils::SCREEN_SIZE.y as usize
                        - ((ly1 as usize + ((my1 as usize) << 8)) * utils::SCREEN_SIZE.y as usize)
                            / utils::TOUCH_SIZE.y as usize,
                )
                    .into(),
                (
                    utils::SCREEN_SIZE.x as usize
                        - ((lx2 as usize + ((mx2 as usize) << 8)) * utils::SCREEN_SIZE.x as usize)
                            / utils::TOUCH_SIZE.x as usize,
                    utils::SCREEN_SIZE.y as usize
                        - ((ly2 as usize + ((my2 as usize) << 8)) * utils::SCREEN_SIZE.y as usize)
                            / utils::TOUCH_SIZE.y as usize,
                )
                    .into(),
            ),
            [_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _] => Touch::None,
        }
    }
}
#[derive(PartialEq, Debug, Clone)]
pub enum Key {
    Left,
    Right,
    Home,
    Power,
    Unknown([u8; 16]),
}

impl From<[u8; 16]> for Key {
    fn from(event: [u8; 16]) -> Self {
        match event {
            [_, _, _, 0x31, _, _, _, _, _, _, _, _, _, _, _, _] => Key::Home,
            [_, _, _, 0x32, _, _, _, _, _, _, _, _, _, _, _, _] => Key::Right,
            [_, _, _, 0x33, _, _, _, _, _, _, _, _, _, _, _, _] => Key::Left,
            [_, _, _, 0x6f, _, _, _, _, _, _, _, _, _, _, _, _] => Key::Power,
            _ => Key::Unknown(event),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Event {
    TouchPressed(Touch),
    TouchMove(Touch),
    TouchReleased,
    Key(Key),
    Unknown([u8; 16]),
}

impl From<[u8; 16]> for Event {
    fn from(event: [u8; 16]) -> Self {
        match event {
            [0x6b, 0x80, 0x10, _, _, _, _, _, _, _, _, _, _, _, _, _] => {
                Event::Key(Key::from(event))
            }
            [0x74, 0xc0, 0x10, _, _, _, _, _, _, _, _, _, _, _, _, _] => {
                Event::TouchPressed(Touch::from(event))
            }
            [0x74, 0x80, 0x10, _, _, _, _, _, _, _, _, _, _, _, _, _] => {
                Event::TouchMove(Touch::from(event))
            }
            [0x74, 0x40, 0x10, _, _, _, _, _, _, _, _, _, _, _, _, _] => Event::TouchReleased,
            _ => Event::Unknown(event),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_home() {
        assert_eq!(
            Event::from([
                0x6b, 0x80, 0x10, 0x31, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00,
                0x00, 0x00
            ]),
            Event::Key(Key::Home)
        );
    }

    #[test]
    fn key_right() {
        assert_eq!(
            Event::from([
                0x6b, 0x80, 0x10, 0x32, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00,
                0x00, 0x00
            ]),
            Event::Key(Key::Right)
        );
    }

    #[test]
    fn key_left() {
        assert_eq!(
            Event::from([
                0x6b, 0x80, 0x10, 0x33, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00,
                0x00, 0x00
            ]),
            Event::Key(Key::Left)
        );
    }
    #[test]
    fn key_power() {
        assert_eq!(
            Event::from([
                0x6b, 0x80, 0x10, 0x6f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00,
                0x00, 0x00
            ]),
            Event::Key(Key::Power)
        );
    }
}
