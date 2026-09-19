use rmk::types::action::{EncoderAction, KeyAction};
use rmk::{a, encoder, k, layer, mo};

pub(crate) const COL: usize = 16;        // 总列数：左手 10 + 右手 6
pub(crate) const ROW: usize = 5;         // 行数
pub(crate) const NUM_LAYER: usize = 8;   // 层数
pub(crate) const NUM_ENCODER: usize = 2; // 编码器数量

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        // ---- 第 0 层：基础层 ----
        layer!([
            // 左手 10 列（主键盘 6 + 小键盘 4），右手 6 列占位
            [k!(Grave), k!(Kc1), k!(Kc2), k!(Kc3), k!(Kc4), k!(Kc5), k!(Kp7), k!(Kp8), k!(Kp9), k!(KpPlus),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [k!(Tab), k!(Q), k!(W), k!(E), k!(R), k!(T), k!(Kp4), k!(Kp5), k!(Kp6), k!(KpMinus),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [k!(CapsLock), k!(A), k!(S), k!(D), k!(F), k!(G), k!(Kp1), k!(Kp2), k!(Kp3), k!(KpEnter),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [k!(LShift), k!(Z), k!(X), k!(C), k!(V), k!(B), k!(Kp0), k!(KpDot), k!(KpSlash), a!(No),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [k!(LCtrl), k!(LGui), a!(No), k!(LAlt), a!(No), k!(Space), k!(KpAsterisk), k!(Kp0), mo!(1), a!(No),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),
        // ---- 第 1 层：Fn 层 ----
        layer!([
            [k!(Grave), k!(F1), k!(F2), k!(F3), k!(F4), k!(F5), a!(No), a!(No), a!(No), a!(No),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [mo!(3), a!(No), a!(No), mo!(0), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),
        // ---- 第 2 层：Sleep / PrtSc 层 ----
        layer!([
            [k!(Stop), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), k!(PrintScreen),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [k!(Home), k!(End), k!(PageUp), k!(PageDown), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),
        // ---- 第 3 层：数字小键盘层 ----
        layer!([
            [k!(Escape), k!(KpMinus), k!(KpPlus), k!(KpSlash), k!(Kp1), k!(Kp2), k!(Kp3), k!(Kp4), k!(Kp5), k!(Kp6),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [k!(Kp7), k!(Kp8), k!(Kp9), k!(Tab), k!(Q), k!(W), k!(E), k!(R), k!(T), k!(Y),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [k!(Kp4), k!(Kp5), k!(Kp6), k!(KpAsterisk), k!(A), k!(S), k!(D), k!(F), k!(G), k!(H),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [k!(Kp1), k!(Kp2), k!(Kp3), k!(LShift), k!(Z), k!(X), k!(C), k!(V), k!(B), k!(N),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [k!(Backspace), k!(KpDot), k!(Kp0), k!(LCtrl), k!(LGui), mo!(1), a!(No), k!(LAlt), k!(Space), k!(Left),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),
        // ---- 第 4~7 层：暂空，全部占位 ----
        layer!([
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),
        layer!([
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),
        layer!([
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),
        layer!([
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
             a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ])
    ]
}

pub const fn get_default_encoder_map() -> [[EncoderAction; NUM_ENCODER]; NUM_LAYER] {
    [
        [
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
        ],
        [
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
        ],
        [
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
        ],
        [
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
        ],
        [
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
        ],
        [
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
        ],
        [
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
        ],
        [
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
            encoder!(k!(KbVolumeUp), k!(KbVolumeDown)),
        ],
    ]
}