#![no_std]
#![warn(missing_docs, unreachable_pub)]

//! # Material Design Colors for Rust
//!
//! Easily get color variants based on the Material Design Pallete. All colors
//! are represented as 0xRRGGBB with the first two bytes not used. To access a
//! color you may use the `const fn` under [`MatColor`] or access the `const`
//! arrays that are publicly available. Also, check out [`MatColor::new`] to
//! create Material Design colors based on [`MatColorVariant`] and [`MatColorAccent`]
//! on demand.
//!
//! For More information, check out the [Official Website](https://material.io/design/color/the-color-system.html).
//!
//! <br />
//!
//! <div style="overflow-x: auto;">
//!     <div style="display: flex; align-items: center; font-size: 12px; text-align: center;"><div style="width: 100px;"></div><div style="width: 40px">C50</div><div style="width: 40px">C100</div><div style="width: 40px">C200</div><div style="width: 40px">C300</div><div style="width: 40px">C400</div><div style="width: 40px">C500</div><div style="width: 40px">C600</div><div style="width: 40px">C700</div><div style="width: 40px">C800</div><div style="width: 40px">C900</div><div style="width: 40px">A100</div><div style="width: 40px">A200</div><div style="width: 40px">A400</div><div style="width: 40px">A700</div></div>
//!     <div style="display: flex; align-items: center;"><div style="width: 100px;">Red</div><div style="width: 40px; height: 40px; background-color: #FFEBEE;"></div><div style="width: 40px; height: 40px; background-color: #FFCDD2;"></div><div style="width: 40px; height: 40px; background-color: #EF9A9A;"></div><div style="width: 40px; height: 40px; background-color: #E57373;"></div><div style="width: 40px; height: 40px; background-color: #EF5350;"></div><div style="width: 40px; height: 40px; background-color: #F44336;"></div><div style="width: 40px; height: 40px; background-color: #E53935;"></div><div style="width: 40px; height: 40px; background-color: #D32F2F;"></div><div style="width: 40px; height: 40px; background-color: #C62828;"></div><div style="width: 40px; height: 40px; background-color: #B71C1C;"></div><div style="width: 40px; height: 40px; background-color: #FF8A80;"></div><div style="width: 40px; height: 40px; background-color: #FF5252;"></div><div style="width: 40px; height: 40px; background-color: #FF1744;"></div><div style="width: 40px; height: 40px; background-color: #D50000;"></div></div>
//!     <div style="display: flex; align-items: center;"><div style="width: 100px;">Pink</div><div style="width: 40px; height: 40px; background-color: #FCE4EC;"></div><div style="width: 40px; height: 40px; background-color: #F8BBD0;"></div><div style="width: 40px; height: 40px; background-color: #F48FB1;"></div><div style="width: 40px; height: 40px; background-color: #F06292;"></div><div style="width: 40px; height: 40px; background-color: #EC407A;"></div><div style="width: 40px; height: 40px; background-color: #E91E63;"></div><div style="width: 40px; height: 40px; background-color: #D81B60;"></div><div style="width: 40px; height: 40px; background-color: #C2185B;"></div><div style="width: 40px; height: 40px; background-color: #AD1457;"></div><div style="width: 40px; height: 40px; background-color: #880E4F;"></div><div style="width: 40px; height: 40px; background-color: #FF80AB;"></div><div style="width: 40px; height: 40px; background-color: #FF4081;"></div><div style="width: 40px; height: 40px; background-color: #F50057;"></div><div style="width: 40px; height: 40px; background-color: #C51162;"></div></div>
//!     <div style="display: flex; align-items: center;"><div style="width: 100px;">Purple</div><div style="width: 40px; height: 40px; background-color: #F3E5F5;"></div><div style="width: 40px; height: 40px; background-color: #E1BEE7;"></div><div style="width: 40px; height: 40px; background-color: #CE93D8;"></div><div style="width: 40px; height: 40px; background-color: #BA68C8;"></div><div style="width: 40px; height: 40px; background-color: #AB47BC;"></div><div style="width: 40px; height: 40px; background-color: #9C27B0;"></div><div style="width: 40px; height: 40px; background-color: #8E24AA;"></div><div style="width: 40px; height: 40px; background-color: #7B1FA2;"></div><div style="width: 40px; height: 40px; background-color: #6A1B9A;"></div><div style="width: 40px; height: 40px; background-color: #4A148C;"></div><div style="width: 40px; height: 40px; background-color: #EA80FC;"></div><div style="width: 40px; height: 40px; background-color: #E040FB;"></div><div style="width: 40px; height: 40px; background-color: #D500F9;"></div><div style="width: 40px; height: 40px; background-color: #AA00FF;"></div></div>
//!     <div style="display: flex; align-items: center;"><div style="width: 100px;">DeepPurple</div><div style="width: 40px; height: 40px; background-color: #EDE7F6;"></div><div style="width: 40px; height: 40px; background-color: #D1C4E9;"></div><div style="width: 40px; height: 40px; background-color: #B39DDB;"></div><div style="width: 40px; height: 40px; background-color: #9575CD;"></div><div style="width: 40px; height: 40px; background-color: #7E57C2;"></div><div style="width: 40px; height: 40px; background-color: #673AB7;"></div><div style="width: 40px; height: 40px; background-color: #5E35B1;"></div><div style="width: 40px; height: 40px; background-color: #512DA8;"></div><div style="width: 40px; height: 40px; background-color: #4527A0;"></div><div style="width: 40px; height: 40px; background-color: #311B92;"></div><div style="width: 40px; height: 40px; background-color: #B388FF;"></div><div style="width: 40px; height: 40px; background-color: #7C4DFF;"></div><div style="width: 40px; height: 40px; background-color: #651FFF;"></div><div style="width: 40px; height: 40px; background-color: #6200EA;"></div></div>
//!     <div style="display: flex; align-items: center;"><div style="width: 100px;">Indigo</div><div style="width: 40px; height: 40px; background-color: #E8EAF6;"></div><div style="width: 40px; height: 40px; background-color: #C5CAE9;"></div><div style="width: 40px; height: 40px; background-color: #9FA8DA;"></div><div style="width: 40px; height: 40px; background-color: #7986CB;"></div><div style="width: 40px; height: 40px; background-color: #5C6BC0;"></div><div style="width: 40px; height: 40px; background-color: #3F51B5;"></div><div style="width: 40px; height: 40px; background-color: #3949AB;"></div><div style="width: 40px; height: 40px; background-color: #303F9F;"></div><div style="width: 40px; height: 40px; background-color: #283593;"></div><div style="width: 40px; height: 40px; background-color: #1A237E;"></div><div style="width: 40px; height: 40px; background-color: #8C9EFF;"></div><div style="width: 40px; height: 40px; background-color: #536DFE;"></div><div style="width: 40px; height: 40px; background-color: #3D5AFE;"></div><div style="width: 40px; height: 40px; background-color: #304FFE;"></div></div>
//!     <div style="display: flex; align-items: center;"><div style="width: 100px;">Blue</div><div style="width: 40px; height: 40px; background-color: #E3F2FD;"></div><div style="width: 40px; height: 40px; background-color: #BBDEFB;"></div><div style="width: 40px; height: 40px; background-color: #90CAF9;"></div><div style="width: 40px; height: 40px; background-color: #64B5F6;"></div><div style="width: 40px; height: 40px; background-color: #42A5F5;"></div><div style="width: 40px; height: 40px; background-color: #2196F3;"></div><div style="width: 40px; height: 40px; background-color: #1E88E5;"></div><div style="width: 40px; height: 40px; background-color: #1976D2;"></div><div style="width: 40px; height: 40px; background-color: #1565C0;"></div><div style="width: 40px; height: 40px; background-color: #0D47A1;"></div><div style="width: 40px; height: 40px; background-color: #82B1FF;"></div><div style="width: 40px; height: 40px; background-color: #448AFF;"></div><div style="width: 40px; height: 40px; background-color: #2979FF;"></div><div style="width: 40px; height: 40px; background-color: #2962FF;"></div></div>
//!     <div style="display: flex; align-items: center;"><div style="width: 100px;">LightBlue</div><div style="width: 40px; height: 40px; background-color: #E1F5FE;"></div><div style="width: 40px; height: 40px; background-color: #B3E5FC;"></div><div style="width: 40px; height: 40px; background-color: #81D4FA;"></div><div style="width: 40px; height: 40px; background-color: #4FC3F7;"></div><div style="width: 40px; height: 40px; background-color: #29B6F6;"></div><div style="width: 40px; height: 40px; background-color: #03A9F4;"></div><div style="width: 40px; height: 40px; background-color: #039BE5;"></div><div style="width: 40px; height: 40px; background-color: #0288D1;"></div><div style="width: 40px; height: 40px; background-color: #0277BD;"></div><div style="width: 40px; height: 40px; background-color: #01579B;"></div><div style="width: 40px; height: 40px; background-color: #80D8FF;"></div><div style="width: 40px; height: 40px; background-color: #40C4FF;"></div><div style="width: 40px; height: 40px; background-color: #00B0FF;"></div><div style="width: 40px; height: 40px; background-color: #0091EA;"></div></div>
//!     <div style="display: flex; align-items: center;"><div style="width: 100px;">Cyan</div><div style="width: 40px; height: 40px; background-color: #E0F7FA;"></div><div style="width: 40px; height: 40px; background-color: #B2EBF2;"></div><div style="width: 40px; height: 40px; background-color: #80DEEA;"></div><div style="width: 40px; height: 40px; background-color: #4DD0E1;"></div><div style="width: 40px; height: 40px; background-color: #26C6DA;"></div><div style="width: 40px; height: 40px; background-color: #00BCD4;"></div><div style="width: 40px; height: 40px; background-color: #00ACC1;"></div><div style="width: 40px; height: 40px; background-color: #0097A7;"></div><div style="width: 40px; height: 40px; background-color: #00838F;"></div><div style="width: 40px; height: 40px; background-color: #006064;"></div><div style="width: 40px; height: 40px; background-color: #84FFFF;"></div><div style="width: 40px; height: 40px; background-color: #18FFFF;"></div><div style="width: 40px; height: 40px; background-color: #00E5FF;"></div><div style="width: 40px; height: 40px; background-color: #00B8D4;"></div></div>
//!     <div style="display: flex; align-items: center;"><div style="width: 100px;">Teal</div><div style="width: 40px; height: 40px; background-color: #E0F2F1;"></div><div style="width: 40px; height: 40px; background-color: #B2DFDB;"></div><div style="width: 40px; height: 40px; background-color: #80CBC4;"></div><div style="width: 40px; height: 40px; background-color: #4DB6AC;"></div><div style="width: 40px; height: 40px; background-color: #26A69A;"></div><div style="width: 40px; height: 40px; background-color: #009688;"></div><div style="width: 40px; height: 40px; background-color: #00897B;"></div><div style="width: 40px; height: 40px; background-color: #00796B;"></div><div style="width: 40px; height: 40px; background-color: #00695C;"></div><div style="width: 40px; height: 40px; background-color: #004D40;"></div><div style="width: 40px; height: 40px; background-color: #A7FFEB;"></div><div style="width: 40px; height: 40px; background-color: #64FFDA;"></div><div style="width: 40px; height: 40px; background-color: #1DE9B6;"></div><div style="width: 40px; height: 40px; background-color: #00BFA5;"></div></div>
//!     <div style="display: flex; align-items: center;"><div style="width: 100px;">Green</div><div style="width: 40px; height: 40px; background-color: #E8F5E9;"></div><div style="width: 40px; height: 40px; background-color: #C8E6C9;"></div><div style="width: 40px; height: 40px; background-color: #A5D6A7;"></div><div style="width: 40px; height: 40px; background-color: #81C784;"></div><div style="width: 40px; height: 40px; background-color: #66BB6A;"></div><div style="width: 40px; height: 40px; background-color: #4CAF50;"></div><div style="width: 40px; height: 40px; background-color: #43A047;"></div><div style="width: 40px; height: 40px; background-color: #388E3C;"></div><div style="width: 40px; height: 40px; background-color: #2E7D32;"></div><div style="width: 40px; height: 40px; background-color: #1B5E20;"></div><div style="width: 40px; height: 40px; background-color: #B9F6CA;"></div><div style="width: 40px; height: 40px; background-color: #69F0AE;"></div><div style="width: 40px; height: 40px; background-color: #00E676;"></div><div style="width: 40px; height: 40px; background-color: #00C853;"></div></div>
//!     <div style="display: flex; align-items: center;"><div style="width: 100px;">LightGreen</div><div style="width: 40px; height: 40px; background-color: #F1F8E9;"></div><div style="width: 40px; height: 40px; background-color: #DCEDC8;"></div><div style="width: 40px; height: 40px; background-color: #C5E1A5;"></div><div style="width: 40px; height: 40px; background-color: #AED581;"></div><div style="width: 40px; height: 40px; background-color: #9CCC65;"></div><div style="width: 40px; height: 40px; background-color: #8BC34A;"></div><div style="width: 40px; height: 40px; background-color: #7CB342;"></div><div style="width: 40px; height: 40px; background-color: #689F38;"></div><div style="width: 40px; height: 40px; background-color: #558B2F;"></div><div style="width: 40px; height: 40px; background-color: #33691E;"></div><div style="width: 40px; height: 40px; background-color: #CCFF90;"></div><div style="width: 40px; height: 40px; background-color: #B2FF59;"></div><div style="width: 40px; height: 40px; background-color: #76FF03;"></div><div style="width: 40px; height: 40px; background-color: #64DD17;"></div></div>
//!     <div style="display: flex; align-items: center;"><div style="width: 100px;">Lime</div><div style="width: 40px; height: 40px; background-color: #F9FBE7;"></div><div style="width: 40px; height: 40px; background-color: #F0F4C3;"></div><div style="width: 40px; height: 40px; background-color: #E6EE9C;"></div><div style="width: 40px; height: 40px; background-color: #DCE775;"></div><div style="width: 40px; height: 40px; background-color: #D4E157;"></div><div style="width: 40px; height: 40px; background-color: #CDDC39;"></div><div style="width: 40px; height: 40px; background-color: #C0CA33;"></div><div style="width: 40px; height: 40px; background-color: #AFB42B;"></div><div style="width: 40px; height: 40px; background-color: #9E9D24;"></div><div style="width: 40px; height: 40px; background-color: #827717;"></div><div style="width: 40px; height: 40px; background-color: #F4FF81;"></div><div style="width: 40px; height: 40px; background-color: #EEFF41;"></div><div style="width: 40px; height: 40px; background-color: #C6FF00;"></div><div style="width: 40px; height: 40px; background-color: #AEEA00;"></div></div>
//!     <div style="display: flex; align-items: center;"><div style="width: 100px;">Yellow</div><div style="width: 40px; height: 40px; background-color: #FFFDE7;"></div><div style="width: 40px; height: 40px; background-color: #FFF9C4;"></div><div style="width: 40px; height: 40px; background-color: #FFF59D;"></div><div style="width: 40px; height: 40px; background-color: #FFF176;"></div><div style="width: 40px; height: 40px; background-color: #FFEE58;"></div><div style="width: 40px; height: 40px; background-color: #FFEB3B;"></div><div style="width: 40px; height: 40px; background-color: #FDD835;"></div><div style="width: 40px; height: 40px; background-color: #FBC02D;"></div><div style="width: 40px; height: 40px; background-color: #F9A825;"></div><div style="width: 40px; height: 40px; background-color: #F57F17;"></div><div style="width: 40px; height: 40px; background-color: #FFFF8D;"></div><div style="width: 40px; height: 40px; background-color: #FFFF00;"></div><div style="width: 40px; height: 40px; background-color: #FFEA00;"></div><div style="width: 40px; height: 40px; background-color: #FFD600;"></div></div>
//!     <div style="display: flex; align-items: center;"><div style="width: 100px;">Amber</div><div style="width: 40px; height: 40px; background-color: #FFF8E1;"></div><div style="width: 40px; height: 40px; background-color: #FFECB3;"></div><div style="width: 40px; height: 40px; background-color: #FFE082;"></div><div style="width: 40px; height: 40px; background-color: #FFD54F;"></div><div style="width: 40px; height: 40px; background-color: #FFCA28;"></div><div style="width: 40px; height: 40px; background-color: #FFC107;"></div><div style="width: 40px; height: 40px; background-color: #FFB300;"></div><div style="width: 40px; height: 40px; background-color: #FFA000;"></div><div style="width: 40px; height: 40px; background-color: #FF8F00;"></div><div style="width: 40px; height: 40px; background-color: #FF6F00;"></div><div style="width: 40px; height: 40px; background-color: #FFE57F;"></div><div style="width: 40px; height: 40px; background-color: #FFD740;"></div><div style="width: 40px; height: 40px; background-color: #FFC400;"></div><div style="width: 40px; height: 40px; background-color: #FFAB00;"></div></div>
//!     <div style="display: flex; align-items: center;"><div style="width: 100px;">Orange</div><div style="width: 40px; height: 40px; background-color: #FFF3E0;"></div><div style="width: 40px; height: 40px; background-color: #FFE0B2;"></div><div style="width: 40px; height: 40px; background-color: #FFCC80;"></div><div style="width: 40px; height: 40px; background-color: #FFB74D;"></div><div style="width: 40px; height: 40px; background-color: #FFA726;"></div><div style="width: 40px; height: 40px; background-color: #FF9800;"></div><div style="width: 40px; height: 40px; background-color: #FB8C00;"></div><div style="width: 40px; height: 40px; background-color: #F57C00;"></div><div style="width: 40px; height: 40px; background-color: #EF6C00;"></div><div style="width: 40px; height: 40px; background-color: #E65100;"></div><div style="width: 40px; height: 40px; background-color: #FFD180;"></div><div style="width: 40px; height: 40px; background-color: #FFAB40;"></div><div style="width: 40px; height: 40px; background-color: #FF9100;"></div><div style="width: 40px; height: 40px; background-color: #FF6D00;"></div></div>
//!     <div style="display: flex; align-items: center;"><div style="width: 100px;">DeepOrange</div><div style="width: 40px; height: 40px; background-color: #FBE9E7;"></div><div style="width: 40px; height: 40px; background-color: #FFCCBC;"></div><div style="width: 40px; height: 40px; background-color: #FFAB91;"></div><div style="width: 40px; height: 40px; background-color: #FF8A65;"></div><div style="width: 40px; height: 40px; background-color: #FF7043;"></div><div style="width: 40px; height: 40px; background-color: #FF5722;"></div><div style="width: 40px; height: 40px; background-color: #F4511E;"></div><div style="width: 40px; height: 40px; background-color: #E64A19;"></div><div style="width: 40px; height: 40px; background-color: #D84315;"></div><div style="width: 40px; height: 40px; background-color: #BF360C;"></div><div style="width: 40px; height: 40px; background-color: #FF9E80;"></div><div style="width: 40px; height: 40px; background-color: #FF6E40;"></div><div style="width: 40px; height: 40px; background-color: #FF3D00;"></div><div style="width: 40px; height: 40px; background-color: #DD2C00;"></div></div>
//!     <div style="display: flex; align-items: center;"><div style="width: 100px;">Brown</div><div style="width: 40px; height: 40px; background-color: #EFEBE9;"></div><div style="width: 40px; height: 40px; background-color: #D7CCC8;"></div><div style="width: 40px; height: 40px; background-color: #BCAAA4;"></div><div style="width: 40px; height: 40px; background-color: #A1887F;"></div><div style="width: 40px; height: 40px; background-color: #8D6E63;"></div><div style="width: 40px; height: 40px; background-color: #795548;"></div><div style="width: 40px; height: 40px; background-color: #6D4C41;"></div><div style="width: 40px; height: 40px; background-color: #5D4037;"></div><div style="width: 40px; height: 40px; background-color: #4E342E;"></div><div style="width: 40px; height: 40px; background-color: #3E2723;"></div></div>
//!     <div style="display: flex; align-items: center;"><div style="width: 100px;">Gray</div><div style="width: 40px; height: 40px; background-color: #FAFAFA;"></div><div style="width: 40px; height: 40px; background-color: #F5F5F5;"></div><div style="width: 40px; height: 40px; background-color: #EEEEEE;"></div><div style="width: 40px; height: 40px; background-color: #E0E0E0;"></div><div style="width: 40px; height: 40px; background-color: #BDBDBD;"></div><div style="width: 40px; height: 40px; background-color: #9E9E9E;"></div><div style="width: 40px; height: 40px; background-color: #757575;"></div><div style="width: 40px; height: 40px; background-color: #616161;"></div><div style="width: 40px; height: 40px; background-color: #424242;"></div><div style="width: 40px; height: 40px; background-color: #212121;"></div></div>
//!     <div style="display: flex; align-items: center;"><div style="width: 100px;">BlueGray</div><div style="width: 40px; height: 40px; background-color: #ECEFF1;"></div><div style="width: 40px; height: 40px; background-color: #CFD8DC;"></div><div style="width: 40px; height: 40px; background-color: #B0BEC5;"></div><div style="width: 40px; height: 40px; background-color: #90A4AE;"></div><div style="width: 40px; height: 40px; background-color: #78909C;"></div><div style="width: 40px; height: 40px; background-color: #607D8B;"></div><div style="width: 40px; height: 40px; background-color: #546E7A;"></div><div style="width: 40px; height: 40px; background-color: #455A64;"></div><div style="width: 40px; height: 40px; background-color: #37474F;"></div><div style="width: 40px; height: 40px; background-color: #263238;"></div></div>
//! </div>
//!
//! ## Examples
//!
//! ```
//! use matdesign_color::MatColor;
//!
//! let color = MatColor.red().c300();
//! ```

#[cfg(test)]
use strum::EnumIter;
#[cfg(test)]
extern crate std;

/// Main access point to all colors.
pub struct MatColor;

/// Red color accents from lightest to darkest
/// <div style="display: flex; align-items: center;"><div style="width: 100px;">Red</div><div style="width: 40px; height: 40px; background-color: #FFEBEE;"></div><div style="width: 40px; height: 40px; background-color: #FFCDD2;"></div><div style="width: 40px; height: 40px; background-color: #EF9A9A;"></div><div style="width: 40px; height: 40px; background-color: #E57373;"></div><div style="width: 40px; height: 40px; background-color: #EF5350;"></div><div style="width: 40px; height: 40px; background-color: #F44336;"></div><div style="width: 40px; height: 40px; background-color: #E53935;"></div><div style="width: 40px; height: 40px; background-color: #D32F2F;"></div><div style="width: 40px; height: 40px; background-color: #C62828;"></div><div style="width: 40px; height: 40px; background-color: #B71C1C;"></div><div style="width: 40px; height: 40px; background-color: #FF8A80;"></div><div style="width: 40px; height: 40px; background-color: #FF5252;"></div><div style="width: 40px; height: 40px; background-color: #FF1744;"></div><div style="width: 40px; height: 40px; background-color: #D50000;"></div></div>
pub const MAT_COLORS_RED: [u32; 14] = [
    0xFFEBEE, 0xFFCDD2, 0xEF9A9A, 0xE57373, 0xEF5350, 0xF44336, 0xE53935, 0xD32F2F, 0xC62828,
    0xB71C1C, 0xFF8A80, 0xFF5252, 0xFF1744, 0xD50000,
];
/// Pink color accents from lightest to darkest
/// <div style="display: flex; align-items: center;"><div style="width: 100px;">Pink</div><div style="width: 40px; height: 40px; background-color: #FCE4EC;"></div><div style="width: 40px; height: 40px; background-color: #F8BBD0;"></div><div style="width: 40px; height: 40px; background-color: #F48FB1;"></div><div style="width: 40px; height: 40px; background-color: #F06292;"></div><div style="width: 40px; height: 40px; background-color: #EC407A;"></div><div style="width: 40px; height: 40px; background-color: #E91E63;"></div><div style="width: 40px; height: 40px; background-color: #D81B60;"></div><div style="width: 40px; height: 40px; background-color: #C2185B;"></div><div style="width: 40px; height: 40px; background-color: #AD1457;"></div><div style="width: 40px; height: 40px; background-color: #880E4F;"></div><div style="width: 40px; height: 40px; background-color: #FF80AB;"></div><div style="width: 40px; height: 40px; background-color: #FF4081;"></div><div style="width: 40px; height: 40px; background-color: #F50057;"></div><div style="width: 40px; height: 40px; background-color: #C51162;"></div></div>
pub const MAT_COLORS_PINK: [u32; 14] = [
    0xFCE4EC, 0xF8BBD0, 0xF48FB1, 0xF06292, 0xEC407A, 0xE91E63, 0xD81B60, 0xC2185B, 0xAD1457,
    0x880E4F, 0xFF80AB, 0xFF4081, 0xF50057, 0xC51162,
];
/// Purple color accents from lightest to darkest
/// <div style="display: flex; align-items: center;"><div style="width: 100px;">Purple</div><div style="width: 40px; height: 40px; background-color: #F3E5F5;"></div><div style="width: 40px; height: 40px; background-color: #E1BEE7;"></div><div style="width: 40px; height: 40px; background-color: #CE93D8;"></div><div style="width: 40px; height: 40px; background-color: #BA68C8;"></div><div style="width: 40px; height: 40px; background-color: #AB47BC;"></div><div style="width: 40px; height: 40px; background-color: #9C27B0;"></div><div style="width: 40px; height: 40px; background-color: #8E24AA;"></div><div style="width: 40px; height: 40px; background-color: #7B1FA2;"></div><div style="width: 40px; height: 40px; background-color: #6A1B9A;"></div><div style="width: 40px; height: 40px; background-color: #4A148C;"></div><div style="width: 40px; height: 40px; background-color: #EA80FC;"></div><div style="width: 40px; height: 40px; background-color: #E040FB;"></div><div style="width: 40px; height: 40px; background-color: #D500F9;"></div><div style="width: 40px; height: 40px; background-color: #AA00FF;"></div></div>
pub const MAT_COLORS_PURPLE: [u32; 14] = [
    0xF3E5F5, 0xE1BEE7, 0xCE93D8, 0xBA68C8, 0xAB47BC, 0x9C27B0, 0x8E24AA, 0x7B1FA2, 0x6A1B9A,
    0x4A148C, 0xEA80FC, 0xE040FB, 0xD500F9, 0xAA00FF,
];
/// Deep Purple color accents from lightest to darkest
/// <div style="display: flex; align-items: center;"><div style="width: 100px;">DeepPurple</div><div style="width: 40px; height: 40px; background-color: #EDE7F6;"></div><div style="width: 40px; height: 40px; background-color: #D1C4E9;"></div><div style="width: 40px; height: 40px; background-color: #B39DDB;"></div><div style="width: 40px; height: 40px; background-color: #9575CD;"></div><div style="width: 40px; height: 40px; background-color: #7E57C2;"></div><div style="width: 40px; height: 40px; background-color: #673AB7;"></div><div style="width: 40px; height: 40px; background-color: #5E35B1;"></div><div style="width: 40px; height: 40px; background-color: #512DA8;"></div><div style="width: 40px; height: 40px; background-color: #4527A0;"></div><div style="width: 40px; height: 40px; background-color: #311B92;"></div><div style="width: 40px; height: 40px; background-color: #B388FF;"></div><div style="width: 40px; height: 40px; background-color: #7C4DFF;"></div><div style="width: 40px; height: 40px; background-color: #651FFF;"></div><div style="width: 40px; height: 40px; background-color: #6200EA;"></div></div>
pub const MAT_COLORS_DEEP_PURPLE: [u32; 14] = [
    0xEDE7F6, 0xD1C4E9, 0xB39DDB, 0x9575CD, 0x7E57C2, 0x673AB7, 0x5E35B1, 0x512DA8, 0x4527A0,
    0x311B92, 0xB388FF, 0x7C4DFF, 0x651FFF, 0x6200EA,
];
/// Indigo color accents from lightest to darkest
/// <div style="display: flex; align-items: center;"><div style="width: 100px;">Indigo</div><div style="width: 40px; height: 40px; background-color: #E8EAF6;"></div><div style="width: 40px; height: 40px; background-color: #C5CAE9;"></div><div style="width: 40px; height: 40px; background-color: #9FA8DA;"></div><div style="width: 40px; height: 40px; background-color: #7986CB;"></div><div style="width: 40px; height: 40px; background-color: #5C6BC0;"></div><div style="width: 40px; height: 40px; background-color: #3F51B5;"></div><div style="width: 40px; height: 40px; background-color: #3949AB;"></div><div style="width: 40px; height: 40px; background-color: #303F9F;"></div><div style="width: 40px; height: 40px; background-color: #283593;"></div><div style="width: 40px; height: 40px; background-color: #1A237E;"></div><div style="width: 40px; height: 40px; background-color: #8C9EFF;"></div><div style="width: 40px; height: 40px; background-color: #536DFE;"></div><div style="width: 40px; height: 40px; background-color: #3D5AFE;"></div><div style="width: 40px; height: 40px; background-color: #304FFE;"></div></div>
pub const MAT_COLORS_INDIGO: [u32; 14] = [
    0xE8EAF6, 0xC5CAE9, 0x9FA8DA, 0x7986CB, 0x5C6BC0, 0x3F51B5, 0x3949AB, 0x303F9F, 0x283593,
    0x1A237E, 0x8C9EFF, 0x536DFE, 0x3D5AFE, 0x304FFE,
];
/// Blue color accents from lightest to darkest
/// <div style="display: flex; align-items: center;"><div style="width: 100px;">Blue</div><div style="width: 40px; height: 40px; background-color: #E3F2FD;"></div><div style="width: 40px; height: 40px; background-color: #BBDEFB;"></div><div style="width: 40px; height: 40px; background-color: #90CAF9;"></div><div style="width: 40px; height: 40px; background-color: #64B5F6;"></div><div style="width: 40px; height: 40px; background-color: #42A5F5;"></div><div style="width: 40px; height: 40px; background-color: #2196F3;"></div><div style="width: 40px; height: 40px; background-color: #1E88E5;"></div><div style="width: 40px; height: 40px; background-color: #1976D2;"></div><div style="width: 40px; height: 40px; background-color: #1565C0;"></div><div style="width: 40px; height: 40px; background-color: #0D47A1;"></div><div style="width: 40px; height: 40px; background-color: #82B1FF;"></div><div style="width: 40px; height: 40px; background-color: #448AFF;"></div><div style="width: 40px; height: 40px; background-color: #2979FF;"></div><div style="width: 40px; height: 40px; background-color: #2962FF;"></div></div>
pub const MAT_COLORS_BLUE: [u32; 14] = [
    0xE3F2FD, 0xBBDEFB, 0x90CAF9, 0x64B5F6, 0x42A5F5, 0x2196F3, 0x1E88E5, 0x1976D2, 0x1565C0,
    0x0D47A1, 0x82B1FF, 0x448AFF, 0x2979FF, 0x2962FF,
];
/// Light Blue color accents from lightest to darkest
/// <div style="display: flex; align-items: center;"><div style="width: 100px;">LightBlue</div><div style="width: 40px; height: 40px; background-color: #E1F5FE;"></div><div style="width: 40px; height: 40px; background-color: #B3E5FC;"></div><div style="width: 40px; height: 40px; background-color: #81D4FA;"></div><div style="width: 40px; height: 40px; background-color: #4FC3F7;"></div><div style="width: 40px; height: 40px; background-color: #29B6F6;"></div><div style="width: 40px; height: 40px; background-color: #03A9F4;"></div><div style="width: 40px; height: 40px; background-color: #039BE5;"></div><div style="width: 40px; height: 40px; background-color: #0288D1;"></div><div style="width: 40px; height: 40px; background-color: #0277BD;"></div><div style="width: 40px; height: 40px; background-color: #01579B;"></div><div style="width: 40px; height: 40px; background-color: #80D8FF;"></div><div style="width: 40px; height: 40px; background-color: #40C4FF;"></div><div style="width: 40px; height: 40px; background-color: #00B0FF;"></div><div style="width: 40px; height: 40px; background-color: #0091EA;"></div></div>
pub const MAT_COLORS_LIGHT_BLUE: [u32; 14] = [
    0xE1F5FE, 0xB3E5FC, 0x81D4FA, 0x4FC3F7, 0x29B6F6, 0x03A9F4, 0x039BE5, 0x0288D1, 0x0277BD,
    0x01579B, 0x80D8FF, 0x40C4FF, 0x00B0FF, 0x0091EA,
];
/// Cyan color accents from lightest to darkest
/// <div style="display: flex; align-items: center;"><div style="width: 100px;">Cyan</div><div style="width: 40px; height: 40px; background-color: #E0F7FA;"></div><div style="width: 40px; height: 40px; background-color: #B2EBF2;"></div><div style="width: 40px; height: 40px; background-color: #80DEEA;"></div><div style="width: 40px; height: 40px; background-color: #4DD0E1;"></div><div style="width: 40px; height: 40px; background-color: #26C6DA;"></div><div style="width: 40px; height: 40px; background-color: #00BCD4;"></div><div style="width: 40px; height: 40px; background-color: #00ACC1;"></div><div style="width: 40px; height: 40px; background-color: #0097A7;"></div><div style="width: 40px; height: 40px; background-color: #00838F;"></div><div style="width: 40px; height: 40px; background-color: #006064;"></div><div style="width: 40px; height: 40px; background-color: #84FFFF;"></div><div style="width: 40px; height: 40px; background-color: #18FFFF;"></div><div style="width: 40px; height: 40px; background-color: #00E5FF;"></div><div style="width: 40px; height: 40px; background-color: #00B8D4;"></div></div>
pub const MAT_COLORS_CYAN: [u32; 14] = [
    0xE0F7FA, 0xB2EBF2, 0x80DEEA, 0x4DD0E1, 0x26C6DA, 0x00BCD4, 0x00ACC1, 0x0097A7, 0x00838F,
    0x006064, 0x84FFFF, 0x18FFFF, 0x00E5FF, 0x00B8D4,
];
/// Teal color accents from lightest to darkest
/// <div style="display: flex; align-items: center;"><div style="width: 100px;">Teal</div><div style="width: 40px; height: 40px; background-color: #E0F2F1;"></div><div style="width: 40px; height: 40px; background-color: #B2DFDB;"></div><div style="width: 40px; height: 40px; background-color: #80CBC4;"></div><div style="width: 40px; height: 40px; background-color: #4DB6AC;"></div><div style="width: 40px; height: 40px; background-color: #26A69A;"></div><div style="width: 40px; height: 40px; background-color: #009688;"></div><div style="width: 40px; height: 40px; background-color: #00897B;"></div><div style="width: 40px; height: 40px; background-color: #00796B;"></div><div style="width: 40px; height: 40px; background-color: #00695C;"></div><div style="width: 40px; height: 40px; background-color: #004D40;"></div><div style="width: 40px; height: 40px; background-color: #A7FFEB;"></div><div style="width: 40px; height: 40px; background-color: #64FFDA;"></div><div style="width: 40px; height: 40px; background-color: #1DE9B6;"></div><div style="width: 40px; height: 40px; background-color: #00BFA5;"></div></div>
pub const MAT_COLORS_TEAL: [u32; 14] = [
    0xE0F2F1, 0xB2DFDB, 0x80CBC4, 0x4DB6AC, 0x26A69A, 0x009688, 0x00897B, 0x00796B, 0x00695C,
    0x004D40, 0xA7FFEB, 0x64FFDA, 0x1DE9B6, 0x00BFA5,
];
/// Green color accents from lightest to darkest
/// <div style="display: flex; align-items: center;"><div style="width: 100px;">Green</div><div style="width: 40px; height: 40px; background-color: #E8F5E9;"></div><div style="width: 40px; height: 40px; background-color: #C8E6C9;"></div><div style="width: 40px; height: 40px; background-color: #A5D6A7;"></div><div style="width: 40px; height: 40px; background-color: #81C784;"></div><div style="width: 40px; height: 40px; background-color: #66BB6A;"></div><div style="width: 40px; height: 40px; background-color: #4CAF50;"></div><div style="width: 40px; height: 40px; background-color: #43A047;"></div><div style="width: 40px; height: 40px; background-color: #388E3C;"></div><div style="width: 40px; height: 40px; background-color: #2E7D32;"></div><div style="width: 40px; height: 40px; background-color: #1B5E20;"></div><div style="width: 40px; height: 40px; background-color: #B9F6CA;"></div><div style="width: 40px; height: 40px; background-color: #69F0AE;"></div><div style="width: 40px; height: 40px; background-color: #00E676;"></div><div style="width: 40px; height: 40px; background-color: #00C853;"></div></div>
pub const MAT_COLORS_GREEN: [u32; 14] = [
    0xE8F5E9, 0xC8E6C9, 0xA5D6A7, 0x81C784, 0x66BB6A, 0x4CAF50, 0x43A047, 0x388E3C, 0x2E7D32,
    0x1B5E20, 0xB9F6CA, 0x69F0AE, 0x00E676, 0x00C853,
];
/// Light Green color accents from lightest to darkest
/// <div style="display: flex; align-items: center;"><div style="width: 100px;">LightGreen</div><div style="width: 40px; height: 40px; background-color: #F1F8E9;"></div><div style="width: 40px; height: 40px; background-color: #DCEDC8;"></div><div style="width: 40px; height: 40px; background-color: #C5E1A5;"></div><div style="width: 40px; height: 40px; background-color: #AED581;"></div><div style="width: 40px; height: 40px; background-color: #9CCC65;"></div><div style="width: 40px; height: 40px; background-color: #8BC34A;"></div><div style="width: 40px; height: 40px; background-color: #7CB342;"></div><div style="width: 40px; height: 40px; background-color: #689F38;"></div><div style="width: 40px; height: 40px; background-color: #558B2F;"></div><div style="width: 40px; height: 40px; background-color: #33691E;"></div><div style="width: 40px; height: 40px; background-color: #CCFF90;"></div><div style="width: 40px; height: 40px; background-color: #B2FF59;"></div><div style="width: 40px; height: 40px; background-color: #76FF03;"></div><div style="width: 40px; height: 40px; background-color: #64DD17;"></div></div>
pub const MAT_COLORS_LIGHT_GREEN: [u32; 14] = [
    0xF1F8E9, 0xDCEDC8, 0xC5E1A5, 0xAED581, 0x9CCC65, 0x8BC34A, 0x7CB342, 0x689F38, 0x558B2F,
    0x33691E, 0xCCFF90, 0xB2FF59, 0x76FF03, 0x64DD17,
];
/// Lime color accents from lightest to darkest
/// <div style="display: flex; align-items: center;"><div style="width: 100px;">Lime</div><div style="width: 40px; height: 40px; background-color: #F9FBE7;"></div><div style="width: 40px; height: 40px; background-color: #F0F4C3;"></div><div style="width: 40px; height: 40px; background-color: #E6EE9C;"></div><div style="width: 40px; height: 40px; background-color: #DCE775;"></div><div style="width: 40px; height: 40px; background-color: #D4E157;"></div><div style="width: 40px; height: 40px; background-color: #CDDC39;"></div><div style="width: 40px; height: 40px; background-color: #C0CA33;"></div><div style="width: 40px; height: 40px; background-color: #AFB42B;"></div><div style="width: 40px; height: 40px; background-color: #9E9D24;"></div><div style="width: 40px; height: 40px; background-color: #827717;"></div><div style="width: 40px; height: 40px; background-color: #F4FF81;"></div><div style="width: 40px; height: 40px; background-color: #EEFF41;"></div><div style="width: 40px; height: 40px; background-color: #C6FF00;"></div><div style="width: 40px; height: 40px; background-color: #AEEA00;"></div></div>
pub const MAT_COLORS_LIME: [u32; 14] = [
    0xF9FBE7, 0xF0F4C3, 0xE6EE9C, 0xDCE775, 0xD4E157, 0xCDDC39, 0xC0CA33, 0xAFB42B, 0x9E9D24,
    0x827717, 0xF4FF81, 0xEEFF41, 0xC6FF00, 0xAEEA00,
];
/// Yellow color accents from lightest to darkest
/// <div style="display: flex; align-items: center;"><div style="width: 100px;">Yellow</div><div style="width: 40px; height: 40px; background-color: #FFFDE7;"></div><div style="width: 40px; height: 40px; background-color: #FFF9C4;"></div><div style="width: 40px; height: 40px; background-color: #FFF59D;"></div><div style="width: 40px; height: 40px; background-color: #FFF176;"></div><div style="width: 40px; height: 40px; background-color: #FFEE58;"></div><div style="width: 40px; height: 40px; background-color: #FFEB3B;"></div><div style="width: 40px; height: 40px; background-color: #FDD835;"></div><div style="width: 40px; height: 40px; background-color: #FBC02D;"></div><div style="width: 40px; height: 40px; background-color: #F9A825;"></div><div style="width: 40px; height: 40px; background-color: #F57F17;"></div><div style="width: 40px; height: 40px; background-color: #FFFF8D;"></div><div style="width: 40px; height: 40px; background-color: #FFFF00;"></div><div style="width: 40px; height: 40px; background-color: #FFEA00;"></div><div style="width: 40px; height: 40px; background-color: #FFD600;"></div></div>
pub const MAT_COLORS_YELLOW: [u32; 14] = [
    0xFFFDE7, 0xFFF9C4, 0xFFF59D, 0xFFF176, 0xFFEE58, 0xFFEB3B, 0xFDD835, 0xFBC02D, 0xF9A825,
    0xF57F17, 0xFFFF8D, 0xFFFF00, 0xFFEA00, 0xFFD600,
];
/// Amber color accents from lightest to darkest
/// <div style="display: flex; align-items: center;"><div style="width: 100px;">Amber</div><div style="width: 40px; height: 40px; background-color: #FFF8E1;"></div><div style="width: 40px; height: 40px; background-color: #FFECB3;"></div><div style="width: 40px; height: 40px; background-color: #FFE082;"></div><div style="width: 40px; height: 40px; background-color: #FFD54F;"></div><div style="width: 40px; height: 40px; background-color: #FFCA28;"></div><div style="width: 40px; height: 40px; background-color: #FFC107;"></div><div style="width: 40px; height: 40px; background-color: #FFB300;"></div><div style="width: 40px; height: 40px; background-color: #FFA000;"></div><div style="width: 40px; height: 40px; background-color: #FF8F00;"></div><div style="width: 40px; height: 40px; background-color: #FF6F00;"></div><div style="width: 40px; height: 40px; background-color: #FFE57F;"></div><div style="width: 40px; height: 40px; background-color: #FFD740;"></div><div style="width: 40px; height: 40px; background-color: #FFC400;"></div><div style="width: 40px; height: 40px; background-color: #FFAB00;"></div></div>
pub const MAT_COLORS_AMBER: [u32; 14] = [
    0xFFF8E1, 0xFFECB3, 0xFFE082, 0xFFD54F, 0xFFCA28, 0xFFC107, 0xFFB300, 0xFFA000, 0xFF8F00,
    0xFF6F00, 0xFFE57F, 0xFFD740, 0xFFC400, 0xFFAB00,
];
/// Orange color accents from lightest to darkest
/// <div style="display: flex; align-items: center;"><div style="width: 100px;">Orange</div><div style="width: 40px; height: 40px; background-color: #FFF3E0;"></div><div style="width: 40px; height: 40px; background-color: #FFE0B2;"></div><div style="width: 40px; height: 40px; background-color: #FFCC80;"></div><div style="width: 40px; height: 40px; background-color: #FFB74D;"></div><div style="width: 40px; height: 40px; background-color: #FFA726;"></div><div style="width: 40px; height: 40px; background-color: #FF9800;"></div><div style="width: 40px; height: 40px; background-color: #FB8C00;"></div><div style="width: 40px; height: 40px; background-color: #F57C00;"></div><div style="width: 40px; height: 40px; background-color: #EF6C00;"></div><div style="width: 40px; height: 40px; background-color: #E65100;"></div><div style="width: 40px; height: 40px; background-color: #FFD180;"></div><div style="width: 40px; height: 40px; background-color: #FFAB40;"></div><div style="width: 40px; height: 40px; background-color: #FF9100;"></div><div style="width: 40px; height: 40px; background-color: #FF6D00;"></div></div>
pub const MAT_COLORS_ORANGE: [u32; 14] = [
    0xFFF3E0, 0xFFE0B2, 0xFFCC80, 0xFFB74D, 0xFFA726, 0xFF9800, 0xFB8C00, 0xF57C00, 0xEF6C00,
    0xE65100, 0xFFD180, 0xFFAB40, 0xFF9100, 0xFF6D00,
];
/// Deep Orange color accents from lightest to darkest
/// <div style="display: flex; align-items: center;"><div style="width: 100px;">DeepOrange</div><div style="width: 40px; height: 40px; background-color: #FBE9E7;"></div><div style="width: 40px; height: 40px; background-color: #FFCCBC;"></div><div style="width: 40px; height: 40px; background-color: #FFAB91;"></div><div style="width: 40px; height: 40px; background-color: #FF8A65;"></div><div style="width: 40px; height: 40px; background-color: #FF7043;"></div><div style="width: 40px; height: 40px; background-color: #FF5722;"></div><div style="width: 40px; height: 40px; background-color: #F4511E;"></div><div style="width: 40px; height: 40px; background-color: #E64A19;"></div><div style="width: 40px; height: 40px; background-color: #D84315;"></div><div style="width: 40px; height: 40px; background-color: #BF360C;"></div><div style="width: 40px; height: 40px; background-color: #FF9E80;"></div><div style="width: 40px; height: 40px; background-color: #FF6E40;"></div><div style="width: 40px; height: 40px; background-color: #FF3D00;"></div><div style="width: 40px; height: 40px; background-color: #DD2C00;"></div></div>
pub const MAT_COLORS_DEEP_ORANGE: [u32; 14] = [
    0xFBE9E7, 0xFFCCBC, 0xFFAB91, 0xFF8A65, 0xFF7043, 0xFF5722, 0xF4511E, 0xE64A19, 0xD84315,
    0xBF360C, 0xFF9E80, 0xFF6E40, 0xFF3D00, 0xDD2C00,
];
/// Brown color accents from lightest to darkest
/// <div style="display: flex; align-items: center;"><div style="width: 100px;">Brown</div><div style="width: 40px; height: 40px; background-color: #EFEBE9;"></div><div style="width: 40px; height: 40px; background-color: #D7CCC8;"></div><div style="width: 40px; height: 40px; background-color: #BCAAA4;"></div><div style="width: 40px; height: 40px; background-color: #A1887F;"></div><div style="width: 40px; height: 40px; background-color: #8D6E63;"></div><div style="width: 40px; height: 40px; background-color: #795548;"></div><div style="width: 40px; height: 40px; background-color: #6D4C41;"></div><div style="width: 40px; height: 40px; background-color: #5D4037;"></div><div style="width: 40px; height: 40px; background-color: #4E342E;"></div><div style="width: 40px; height: 40px; background-color: #3E2723;"></div></div>
pub const MAT_COLORS_BROWN: [u32; 10] = [
    0xEFEBE9, 0xD7CCC8, 0xBCAAA4, 0xA1887F, 0x8D6E63, 0x795548, 0x6D4C41, 0x5D4037, 0x4E342E,
    0x3E2723,
];
/// Gray color accents from lightest to darkest
/// <div style="display: flex; align-items: center;"><div style="width: 100px;">Gray</div><div style="width: 40px; height: 40px; background-color: #FAFAFA;"></div><div style="width: 40px; height: 40px; background-color: #F5F5F5;"></div><div style="width: 40px; height: 40px; background-color: #EEEEEE;"></div><div style="width: 40px; height: 40px; background-color: #E0E0E0;"></div><div style="width: 40px; height: 40px; background-color: #BDBDBD;"></div><div style="width: 40px; height: 40px; background-color: #9E9E9E;"></div><div style="width: 40px; height: 40px; background-color: #757575;"></div><div style="width: 40px; height: 40px; background-color: #616161;"></div><div style="width: 40px; height: 40px; background-color: #424242;"></div><div style="width: 40px; height: 40px; background-color: #212121;"></div></div>
pub const MAT_COLORS_GRAY: [u32; 10] = [
    0xFAFAFA, 0xF5F5F5, 0xEEEEEE, 0xE0E0E0, 0xBDBDBD, 0x9E9E9E, 0x757575, 0x616161, 0x424242,
    0x212121,
];
/// Blue Gray color accents from lightest to darkest
/// <div style="display: flex; align-items: center;"><div style="width: 100px;">BlueGray</div><div style="width: 40px; height: 40px; background-color: #ECEFF1;"></div><div style="width: 40px; height: 40px; background-color: #CFD8DC;"></div><div style="width: 40px; height: 40px; background-color: #B0BEC5;"></div><div style="width: 40px; height: 40px; background-color: #90A4AE;"></div><div style="width: 40px; height: 40px; background-color: #78909C;"></div><div style="width: 40px; height: 40px; background-color: #607D8B;"></div><div style="width: 40px; height: 40px; background-color: #546E7A;"></div><div style="width: 40px; height: 40px; background-color: #455A64;"></div><div style="width: 40px; height: 40px; background-color: #37474F;"></div><div style="width: 40px; height: 40px; background-color: #263238;"></div></div>
pub const MAT_COLORS_BLUE_GRAY: [u32; 10] = [
    0xECEFF1, 0xCFD8DC, 0xB0BEC5, 0x90A4AE, 0x78909C, 0x607D8B, 0x546E7A, 0x455A64, 0x37474F,
    0x263238,
];

const MAT_COLORS_ACCENT: [[u32; 14]; 16] = [
    MAT_COLORS_RED,
    MAT_COLORS_PINK,
    MAT_COLORS_PURPLE,
    MAT_COLORS_DEEP_PURPLE,
    MAT_COLORS_INDIGO,
    MAT_COLORS_BLUE,
    MAT_COLORS_LIGHT_BLUE,
    MAT_COLORS_CYAN,
    MAT_COLORS_TEAL,
    MAT_COLORS_GREEN,
    MAT_COLORS_LIGHT_GREEN,
    MAT_COLORS_LIME,
    MAT_COLORS_YELLOW,
    MAT_COLORS_AMBER,
    MAT_COLORS_ORANGE,
    MAT_COLORS_DEEP_ORANGE,
];

const MAT_COLORS_NO_ACCENT: [[u32; 10]; 3] =
    [MAT_COLORS_BROWN, MAT_COLORS_GRAY, MAT_COLORS_BLUE_GRAY];

/// All the main color variants by name in the 2014 Material Design Pallete.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(test, derive(EnumIter, Hash))]
#[allow(missing_docs)]
pub enum MatColorVariant {
    Red,
    Pink,
    Purple,
    DeepPurple,
    Indigo,
    Blue,
    LightBlue,
    Cyan,
    Teal,
    Green,
    LightGreen,
    Lime,
    Yellow,
    Amber,
    Orange,
    DeepOrange,
    Brown,
    Gray,
    BlueGray,
    Black,
    White,
}

/// All the color accents in the 2014 Material Design Pallete.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(test, derive(EnumIter, Hash))]
#[allow(missing_docs)]
pub enum MatColorAccent {
    C50,
    C100,
    C200,
    C300,
    C400,
    C500,
    C600,
    C700,
    C800,
    C900,
    A100,
    A200,
    A400,
    A700,
}

/// Red color and its accents
pub struct MatColorRed;

#[rustfmt::skip]
impl MatColorRed {
    /// Red\[50\] -> 0xFFEBEE
    /// <div style="padding: 10px; background-color: #FFEBEE"></div>
    pub const fn c50(&self) -> u32 { 0xFFEBEE }
    /// Red\[100\] -> 0xFFCDD2
    /// <div style="padding: 10px; background-color: #FFCDD2"></div>
    pub const fn c100(&self) -> u32 { 0xFFCDD2 }
    /// Red\[200\] -> 0xEF9A9A
    /// <div style="padding: 10px; background-color: #EF9A9A"></div>
    pub const fn c200(&self) -> u32 { 0xEF9A9A }
    /// Red\[300\] -> 0xE57373
    /// <div style="padding: 10px; background-color: #E57373"></div>
    pub const fn c300(&self) -> u32 { 0xE57373 }
    /// Red\[400\] -> 0xEF5350
    /// <div style="padding: 10px; background-color: #EF5350"></div>
    pub const fn c400(&self) -> u32 { 0xEF5350 }
    /// Red\[500\] -> 0xF44336
    /// <div style="padding: 10px; background-color: #F44336"></div>
    pub const fn c500(&self) -> u32 { 0xF44336 }
    /// Red\[600\] -> 0xE53935
    /// <div style="padding: 10px; background-color: #E53935"></div>
    pub const fn c600(&self) -> u32 { 0xE53935 }
    /// Red\[700\] -> 0xD32F2F
    /// <div style="padding: 10px; background-color: #D32F2F"></div>
    pub const fn c700(&self) -> u32 { 0xD32F2F }
    /// Red\[800\] -> 0xC62828
    /// <div style="padding: 10px; background-color: #C62828"></div>
    pub const fn c800(&self) -> u32 { 0xC62828 }
    /// Red\[900\] -> 0xB71C1C
    /// <div style="padding: 10px; background-color: #B71C1C"></div>
    pub const fn c900(&self) -> u32 { 0xB71C1C }
    /// Red\[100\] -> 0xFF8A80
    /// <div style="padding: 10px; background-color: #FF8A80"></div>
    pub const fn a100(&self) -> u32 { 0xFF8A80 }
    /// Red\[200\] -> 0xFF5252
    /// <div style="padding: 10px; background-color: #FF5252"></div>
    pub const fn a200(&self) -> u32 { 0xFF5252 }
    /// Red\[400\] -> 0xFF1744
    /// <div style="padding: 10px; background-color: #FF1744"></div>
    pub const fn a400(&self) -> u32 { 0xFF1744 }
    /// Red\[700\] -> 0xD50000
    /// <div style="padding: 10px; background-color: #D50000"></div>
    pub const fn a700(&self) -> u32 { 0xD50000 }
}

/// Pink color and its accents
pub struct MatColorPink;

#[rustfmt::skip]
impl MatColorPink {
    /// Pink\[50\] -> 0xFCE4EC
    /// <div style="padding: 10px; background-color: #FCE4EC"></div>
    pub const fn c50(&self) -> u32 { 0xFCE4EC }
    /// Pink\[100\] -> 0xF8BBD0
    /// <div style="padding: 10px; background-color: #F8BBD0"></div>
    pub const fn c100(&self) -> u32 { 0xF8BBD0 }
    /// Pink\[200\] -> 0xF48FB1
    /// <div style="padding: 10px; background-color: #F48FB1"></div>
    pub const fn c200(&self) -> u32 { 0xF48FB1 }
    /// Pink\[300\] -> 0xF06292
    /// <div style="padding: 10px; background-color: #F06292"></div>
    pub const fn c300(&self) -> u32 { 0xF06292 }
    /// Pink\[400\] -> 0xEC407A
    /// <div style="padding: 10px; background-color: #EC407A"></div>
    pub const fn c400(&self) -> u32 { 0xEC407A }
    /// Pink\[500\] -> 0xE91E63
    /// <div style="padding: 10px; background-color: #E91E63"></div>
    pub const fn c500(&self) -> u32 { 0xE91E63 }
    /// Pink\[600\] -> 0xD81B60
    /// <div style="padding: 10px; background-color: #D81B60"></div>
    pub const fn c600(&self) -> u32 { 0xD81B60 }
    /// Pink\[700\] -> 0xC2185B
    /// <div style="padding: 10px; background-color: #C2185B"></div>
    pub const fn c700(&self) -> u32 { 0xC2185B }
    /// Pink\[800\] -> 0xAD1457
    /// <div style="padding: 10px; background-color: #AD1457"></div>
    pub const fn c800(&self) -> u32 { 0xAD1457 }
    /// Pink\[900\] -> 0x880E4F
    /// <div style="padding: 10px; background-color: #880E4F"></div>
    pub const fn c900(&self) -> u32 { 0x880E4F }
    /// Pink\[A100\] -> 0xFF80AB
    /// <div style="padding: 10px; background-color: #FF80AB"></div>
    pub const fn a100(&self) -> u32 { 0xFF80AB }
    /// Pink\[A200\] -> 0xFF4081
    /// <div style="padding: 10px; background-color: #FF4081"></div>
    pub const fn a200(&self) -> u32 { 0xFF4081 }
    /// Pink\[A400\] -> 0xF50057
    /// <div style="padding: 10px; background-color: #F50057"></div>
    pub const fn a400(&self) -> u32 { 0xF50057 }
    /// Pink\[A700\] -> 0xC51162
    /// <div style="padding: 10px; background-color: #C51162"></div>
    pub const fn a700(&self) -> u32 { 0xC51162 }
}

/// Purple color and its accents
pub struct MatColorPurple;

#[rustfmt::skip]
impl MatColorPurple {
    /// Purple\[50\] -> 0xF3E5F5
    /// <div style="padding: 10px; background-color: #F3E5F5"></div>
    pub const fn c50(&self) -> u32 { 0xF3E5F5 }
    /// Purple\[100\] -> 0xE1BEE7
    /// <div style="padding: 10px; background-color: #E1BEE7"></div>
    pub const fn c100(&self) -> u32 { 0xE1BEE7 }
    /// Purple\[200\] -> 0xCE93D8
    /// <div style="padding: 10px; background-color: #CE93D8"></div>
    pub const fn c200(&self) -> u32 { 0xCE93D8 }
    /// Purple\[300\] -> 0xBA68C8
    /// <div style="padding: 10px; background-color: #BA68C8"></div>
    pub const fn c300(&self) -> u32 { 0xBA68C8 }
    /// Purple\[400\] -> 0xAB47BC
    /// <div style="padding: 10px; background-color: #AB47BC"></div>
    pub const fn c400(&self) -> u32 { 0xAB47BC }
    /// Purple\[500\] -> 0x9C27B0
    /// <div style="padding: 10px; background-color: #9C27B0"></div>
    pub const fn c500(&self) -> u32 { 0x9C27B0 }
    /// Purple\[600\] -> 0x8E24AA
    /// <div style="padding: 10px; background-color: #8E24AA"></div>
    pub const fn c600(&self) -> u32 { 0x8E24AA }
    /// Purple\[700\] -> 0x7B1FA2
    /// <div style="padding: 10px; background-color: #7B1FA2"></div>
    pub const fn c700(&self) -> u32 { 0x7B1FA2 }
    /// Purple\[800\] -> 0x6A1B9A
    /// <div style="padding: 10px; background-color: #6A1B9A"></div>
    pub const fn c800(&self) -> u32 { 0x6A1B9A }
    /// Purple\[900\] -> 0x4A148C
    /// <div style="padding: 10px; background-color: #4A148C"></div>
    pub const fn c900(&self) -> u32 { 0x4A148C }
    /// Purple\[A100\] -> 0xEA80FC
    /// <div style="padding: 10px; background-color: #EA80FC"></div>
    pub const fn a100(&self) -> u32 { 0xEA80FC }
    /// Purple\[A200\] -> 0xE040FB
    /// <div style="padding: 10px; background-color: #E040FB"></div>
    pub const fn a200(&self) -> u32 { 0xE040FB }
    /// Purple\[A400\] -> 0xD500F9
    /// <div style="padding: 10px; background-color: #D500F9"></div>
    pub const fn a400(&self) -> u32 { 0xD500F9 }
    /// Purple\[A700\] -> 0xAA00FF
    /// <div style="padding: 10px; background-color: #AA00FF"></div>
    pub const fn a700(&self) -> u32 { 0xAA00FF }
}

/// Deep Purple color and its accents
pub struct MatColorDeepPurple;

#[rustfmt::skip]
impl MatColorDeepPurple {
    /// DeepPurple\[50\] -> 0xEDE7F6
    /// <div style="padding: 10px; background-color: #EDE7F6"></div>
    pub const fn c50(&self) -> u32 { 0xEDE7F6 }
    /// DeepPurple\[100\] -> 0xD1C4E9
    /// <div style="padding: 10px; background-color: #D1C4E9"></div>
    pub const fn c100(&self) -> u32 { 0xD1C4E9 }
    /// DeepPurple\[200\] -> 0xB39DDB
    /// <div style="padding: 10px; background-color: #B39DDB"></div>
    pub const fn c200(&self) -> u32 { 0xB39DDB }
    /// DeepPurple\[300\] -> 0x9575CD
    /// <div style="padding: 10px; background-color: #9575CD"></div>
    pub const fn c300(&self) -> u32 { 0x9575CD }
    /// DeepPurple\[400\] -> 0x7E57C2
    /// <div style="padding: 10px; background-color: #7E57C2"></div>
    pub const fn c400(&self) -> u32 { 0x7E57C2 }
    /// DeepPurple\[500\] -> 0x673AB7
    /// <div style="padding: 10px; background-color: #673AB7"></div>
    pub const fn c500(&self) -> u32 { 0x673AB7 }
    /// DeepPurple\[600\] -> 0x5E35B1
    /// <div style="padding: 10px; background-color: #5E35B1"></div>
    pub const fn c600(&self) -> u32 { 0x5E35B1 }
    /// DeepPurple\[700\] -> 0x512DA8
    /// <div style="padding: 10px; background-color: #512DA8"></div>
    pub const fn c700(&self) -> u32 { 0x512DA8 }
    /// DeepPurple\[800\] -> 0x4527A0
    /// <div style="padding: 10px; background-color: #4527A0"></div>
    pub const fn c800(&self) -> u32 { 0x4527A0 }
    /// DeepPurple\[900\] -> 0x311B92
    /// <div style="padding: 10px; background-color: #311B92"></div>
    pub const fn c900(&self) -> u32 { 0x311B92 }
    /// DeepPurple\[A100\] -> 0xB388FF
    /// <div style="padding: 10px; background-color: #B388FF"></div>
    pub const fn a100(&self) -> u32 { 0xB388FF }
    /// DeepPurple\[A200\] -> 0x7C4DFF
    /// <div style="padding: 10px; background-color: #7C4DFF"></div>
    pub const fn a200(&self) -> u32 { 0x7C4DFF }
    /// DeepPurple\[A400\] -> 0x651FFF
    /// <div style="padding: 10px; background-color: #651FFF"></div>
    pub const fn a400(&self) -> u32 { 0x651FFF }
    /// DeepPurple\[A700\] -> 0x6200EA
    /// <div style="padding: 10px; background-color: #6200EA"></div>
    pub const fn a700(&self) -> u32 { 0x6200EA }
}

/// Indigo color and its accents
pub struct MatColorIndigo;

#[rustfmt::skip]
impl MatColorIndigo {
    /// Indigo\[50\] -> 0xE8EAF6
    /// <div style="padding: 10px; background-color: #E8EAF6"></div>
    pub const fn c50(&self) -> u32 { 0xE8EAF6 }
    /// Indigo\[100\] -> 0xC5CAE9
    /// <div style="padding: 10px; background-color: #C5CAE9"></div>
    pub const fn c100(&self) -> u32 { 0xC5CAE9 }
    /// Indigo\[200\] -> 0x9FA8DA
    /// <div style="padding: 10px; background-color: #9FA8DA"></div>
    pub const fn c200(&self) -> u32 { 0x9FA8DA }
    /// Indigo\[300\] -> 0x7986CB
    /// <div style="padding: 10px; background-color: #7986CB"></div>
    pub const fn c300(&self) -> u32 { 0x7986CB }
    /// Indigo\[400\] -> 0x5C6BC0
    /// <div style="padding: 10px; background-color: #5C6BC0"></div>
    pub const fn c400(&self) -> u32 { 0x5C6BC0 }
    /// Indigo\[500\] -> 0x3F51B5
    /// <div style="padding: 10px; background-color: #3F51B5"></div>
    pub const fn c500(&self) -> u32 { 0x3F51B5 }
    /// Indigo\[600\] -> 0x3949AB
    /// <div style="padding: 10px; background-color: #3949AB"></div>
    pub const fn c600(&self) -> u32 { 0x3949AB }
    /// Indigo\[700\] -> 0x303F9F
    /// <div style="padding: 10px; background-color: #303F9F"></div>
    pub const fn c700(&self) -> u32 { 0x303F9F }
    /// Indigo\[800\] -> 0x283593
    /// <div style="padding: 10px; background-color: #283593"></div>
    pub const fn c800(&self) -> u32 { 0x283593 }
    /// Indigo\[900\] -> 0x1A237E
    /// <div style="padding: 10px; background-color: #1A237E"></div>
    pub const fn c900(&self) -> u32 { 0x1A237E }
    /// Indigo\[A100\] -> 0x8C9EFF
    /// <div style="padding: 10px; background-color: #8C9EFF"></div>
    pub const fn a100(&self) -> u32 { 0x8C9EFF }
    /// Indigo\[A200\] -> 0x536DFE
    /// <div style="padding: 10px; background-color: #536DFE"></div>
    pub const fn a200(&self) -> u32 { 0x536DFE }
    /// Indigo\[A400\] -> 0x3D5AFE
    /// <div style="padding: 10px; background-color: #3D5AFE"></div>
    pub const fn a400(&self) -> u32 { 0x3D5AFE }
    /// Indigo\[A700\] -> 0x304FFE
    /// <div style="padding: 10px; background-color: #304FFE"></div>
    pub const fn a700(&self) -> u32 { 0x304FFE }
}

/// Blue color and its accents
pub struct MatColorBlue;

#[rustfmt::skip]
impl MatColorBlue {
    /// Blue\[50\] -> 0xE3F2FD
    /// <div style="padding: 10px; background-color: #E3F2FD"></div>
    pub const fn c50(&self) -> u32 { 0xE3F2FD }
    /// Blue\[100\] -> 0xBBDEFB
    /// <div style="padding: 10px; background-color: #BBDEFB"></div>
    pub const fn c100(&self) -> u32 { 0xBBDEFB }
    /// Blue\[200\] -> 0x90CAF9
    /// <div style="padding: 10px; background-color: #90CAF9"></div>
    pub const fn c200(&self) -> u32 { 0x90CAF9 }
    /// Blue\[300\] -> 0x64B5F6
    /// <div style="padding: 10px; background-color: #64B5F6"></div>
    pub const fn c300(&self) -> u32 { 0x64B5F6 }
    /// Blue\[400\] -> 0x42A5F5
    /// <div style="padding: 10px; background-color: #42A5F5"></div>
    pub const fn c400(&self) -> u32 { 0x42A5F5 }
    /// Blue\[500\] -> 0x2196F3
    /// <div style="padding: 10px; background-color: #2196F3"></div>
    pub const fn c500(&self) -> u32 { 0x2196F3 }
    /// Blue\[600\] -> 0x1E88E5
    /// <div style="padding: 10px; background-color: #1E88E5"></div>
    pub const fn c600(&self) -> u32 { 0x1E88E5 }
    /// Blue\[700\] -> 0x1976D2
    /// <div style="padding: 10px; background-color: #1976D2"></div>
    pub const fn c700(&self) -> u32 { 0x1976D2 }
    /// Blue\[800\] -> 0x1565C0
    /// <div style="padding: 10px; background-color: #1565C0"></div>
    pub const fn c800(&self) -> u32 { 0x1565C0 }
    /// Blue\[900\] -> 0x0D47A1
    /// <div style="padding: 10px; background-color: #0D47A1"></div>
    pub const fn c900(&self) -> u32 { 0x0D47A1 }
    /// Blue\[A100\] -> 0x82B1FF
    /// <div style="padding: 10px; background-color: #82B1FF"></div>
    pub const fn a100(&self) -> u32 { 0x82B1FF }
    /// Blue\[A200\] -> 0x448AFF
    /// <div style="padding: 10px; background-color: #448AFF"></div>
    pub const fn a200(&self) -> u32 { 0x448AFF }
    /// Blue\[A400\] -> 0x2979FF
    /// <div style="padding: 10px; background-color: #2979FF"></div>
    pub const fn a400(&self) -> u32 { 0x2979FF }
    /// Blue\[A700\] -> 0x2962FF
    /// <div style="padding: 10px; background-color: #2962FF"></div>
    pub const fn a700(&self) -> u32 { 0x2962FF }
}

/// Light Blue color and its accents
pub struct MatColorLightBlue;

#[rustfmt::skip]
impl MatColorLightBlue {
    /// LightBlue\[50\] -> 0xE1F5FE
    /// <div style="padding: 10px; background-color: #E1F5FE"></div>
    pub const fn c50(&self) -> u32 { 0xE1F5FE }
    /// LightBlue\[100\] -> 0xB3E5FC
    /// <div style="padding: 10px; background-color: #B3E5FC"></div>
    pub const fn c100(&self) -> u32 { 0xB3E5FC }
    /// LightBlue\[200\] -> 0x81D4FA
    /// <div style="padding: 10px; background-color: #81D4FA"></div>
    pub const fn c200(&self) -> u32 { 0x81D4FA }
    /// LightBlue\[300\] -> 0x4FC3F7
    /// <div style="padding: 10px; background-color: #4FC3F7"></div>
    pub const fn c300(&self) -> u32 { 0x4FC3F7 }
    /// LightBlue\[400\] -> 0x29B6F6
    /// <div style="padding: 10px; background-color: #29B6F6"></div>
    pub const fn c400(&self) -> u32 { 0x29B6F6 }
    /// LightBlue\[500\] -> 0x03A9F4
    /// <div style="padding: 10px; background-color: #03A9F4"></div>
    pub const fn c500(&self) -> u32 { 0x03A9F4 }
    /// LightBlue\[600\] -> 0x039BE5
    /// <div style="padding: 10px; background-color: #039BE5"></div>
    pub const fn c600(&self) -> u32 { 0x039BE5 }
    /// LightBlue\[700\] -> 0x0288D1
    /// <div style="padding: 10px; background-color: #0288D1"></div>
    pub const fn c700(&self) -> u32 { 0x0288D1 }
    /// LightBlue\[800\] -> 0x0277BD
    /// <div style="padding: 10px; background-color: #0277BD"></div>
    pub const fn c800(&self) -> u32 { 0x0277BD }
    /// LightBlue\[900\] -> 0x01579B
    /// <div style="padding: 10px; background-color: #01579B"></div>
    pub const fn c900(&self) -> u32 { 0x01579B }
    /// LightBlue\[A100\] -> 0x80D8FF
    /// <div style="padding: 10px; background-color: #80D8FF"></div>
    pub const fn a100(&self) -> u32 { 0x80D8FF }
    /// LightBlue\[A200\] -> 0x40C4FF
    /// <div style="padding: 10px; background-color: #40C4FF"></div>
    pub const fn a200(&self) -> u32 { 0x40C4FF }
    /// LightBlue\[A400\] -> 0x00B0FF
    /// <div style="padding: 10px; background-color: #00B0FF"></div>
    pub const fn a400(&self) -> u32 { 0x00B0FF }
    /// LightBlue\[A700\] -> 0x0091EA
    /// <div style="padding: 10px; background-color: #0091EA"></div>
    pub const fn a700(&self) -> u32 { 0x0091EA }
}

/// Cyan color and its accents
pub struct MatColorCyan;

#[rustfmt::skip]
impl MatColorCyan {
    /// Cyan\[50\] -> 0xE0F7FA
    /// <div style="padding: 10px; background-color: #E0F7FA"></div>
    pub const fn c50(&self) -> u32 { 0xE0F7FA }
    /// Cyan\[100\] -> 0xB2EBF2
    /// <div style="padding: 10px; background-color: #B2EBF2"></div>
    pub const fn c100(&self) -> u32 { 0xB2EBF2 }
    /// Cyan\[200\] -> 0x80DEEA
    /// <div style="padding: 10px; background-color: #80DEEA"></div>
    pub const fn c200(&self) -> u32 { 0x80DEEA }
    /// Cyan\[300\] -> 0x4DD0E1
    /// <div style="padding: 10px; background-color: #4DD0E1"></div>
    pub const fn c300(&self) -> u32 { 0x4DD0E1 }
    /// Cyan\[400\] -> 0x26C6DA
    /// <div style="padding: 10px; background-color: #26C6DA"></div>
    pub const fn c400(&self) -> u32 { 0x26C6DA }
    /// Cyan\[500\] -> 0x00BCD4
    /// <div style="padding: 10px; background-color: #00BCD4"></div>
    pub const fn c500(&self) -> u32 { 0x00BCD4 }
    /// Cyan\[600\] -> 0x00ACC1
    /// <div style="padding: 10px; background-color: #00ACC1"></div>
    pub const fn c600(&self) -> u32 { 0x00ACC1 }
    /// Cyan\[700\] -> 0x0097A7
    /// <div style="padding: 10px; background-color: #0097A7"></div>
    pub const fn c700(&self) -> u32 { 0x0097A7 }
    /// Cyan\[800\] -> 0x00838F
    /// <div style="padding: 10px; background-color: #00838F"></div>
    pub const fn c800(&self) -> u32 { 0x00838F }
    /// Cyan\[900\] -> 0x006064
    /// <div style="padding: 10px; background-color: #006064"></div>
    pub const fn c900(&self) -> u32 { 0x006064 }
    /// Cyan\[A100\] -> 0x84FFFF
    /// <div style="padding: 10px; background-color: #84FFFF"></div>
    pub const fn a100(&self) -> u32 { 0x84FFFF }
    /// Cyan\[A200\] -> 0x18FFFF
    /// <div style="padding: 10px; background-color: #18FFFF"></div>
    pub const fn a200(&self) -> u32 { 0x18FFFF }
    /// Cyan\[A400\] -> 0x00E5FF
    /// <div style="padding: 10px; background-color: #00E5FF"></div>
    pub const fn a400(&self) -> u32 { 0x00E5FF }
    /// Cyan\[A700\] -> 0x00B8D4
    /// <div style="padding: 10px; background-color: #00B8D4"></div>
    pub const fn a700(&self) -> u32 { 0x00B8D4 }
}

/// Teal color and its accents
pub struct MatColorTeal;

#[rustfmt::skip]
impl MatColorTeal {
    /// Teal\[50\] -> 0xE0F2F1
    /// <div style="padding: 10px; background-color: #E0F2F1"></div>
    pub const fn c50(&self) -> u32 { 0xE0F2F1 }
    /// Teal\[100\] -> 0xB2DFDB
    /// <div style="padding: 10px; background-color: #B2DFDB"></div>
    pub const fn c100(&self) -> u32 { 0xB2DFDB }
    /// Teal\[200\] -> 0x80CBC4
    /// <div style="padding: 10px; background-color: #80CBC4"></div>
    pub const fn c200(&self) -> u32 { 0x80CBC4 }
    /// Teal\[300\] -> 0x4DB6AC
    /// <div style="padding: 10px; background-color: #4DB6AC"></div>
    pub const fn c300(&self) -> u32 { 0x4DB6AC }
    /// Teal\[400\] -> 0x26A69A
    /// <div style="padding: 10px; background-color: #26A69A"></div>
    pub const fn c400(&self) -> u32 { 0x26A69A }
    /// Teal\[500\] -> 0x009688
    /// <div style="padding: 10px; background-color: #009688"></div>
    pub const fn c500(&self) -> u32 { 0x009688 }
    /// Teal\[600\] -> 0x00897B
    /// <div style="padding: 10px; background-color: #00897B"></div>
    pub const fn c600(&self) -> u32 { 0x00897B }
    /// Teal\[700\] -> 0x00796B
    /// <div style="padding: 10px; background-color: #00796B"></div>
    pub const fn c700(&self) -> u32 { 0x00796B }
    /// Teal\[800\] -> 0x00695C
    /// <div style="padding: 10px; background-color: #00695C"></div>
    pub const fn c800(&self) -> u32 { 0x00695C }
    /// Teal\[900\] -> 0x004D40
    /// <div style="padding: 10px; background-color: #004D40"></div>
    pub const fn c900(&self) -> u32 { 0x004D40 }
    /// Teal\[A100\] -> 0xA7FFEB
    /// <div style="padding: 10px; background-color: #A7FFEB"></div>
    pub const fn a100(&self) -> u32 { 0xA7FFEB }
    /// Teal\[A200\] -> 0x64FFDA
    /// <div style="padding: 10px; background-color: #64FFDA"></div>
    pub const fn a200(&self) -> u32 { 0x64FFDA }
    /// Teal\[A400\] -> 0x1DE9B6
    /// <div style="padding: 10px; background-color: #1DE9B6"></div>
    pub const fn a400(&self) -> u32 { 0x1DE9B6 }
    /// Teal\[A700\] -> 0x00BFA5
    /// <div style="padding: 10px; background-color: #00BFA5"></div>
    pub const fn a700(&self) -> u32 { 0x00BFA5 }
}

/// Green color and its accents
pub struct MatColorGreen;

#[rustfmt::skip]
impl MatColorGreen {
    /// Green\[50\] -> 0xE8F5E9
    /// <div style="padding: 10px; background-color: #E8F5E9"></div>
    pub const fn c50(&self) -> u32 { 0xE8F5E9 }
    /// Green\[100\] -> 0xC8E6C9
    /// <div style="padding: 10px; background-color: #C8E6C9"></div>
    pub const fn c100(&self) -> u32 { 0xC8E6C9 }
    /// Green\[200\] -> 0xA5D6A7
    /// <div style="padding: 10px; background-color: #A5D6A7"></div>
    pub const fn c200(&self) -> u32 { 0xA5D6A7 }
    /// Green\[300\] -> 0x81C784
    /// <div style="padding: 10px; background-color: #81C784"></div>
    pub const fn c300(&self) -> u32 { 0x81C784 }
    /// Green\[400\] -> 0x66BB6A
    /// <div style="padding: 10px; background-color: #66BB6A"></div>
    pub const fn c400(&self) -> u32 { 0x66BB6A }
    /// Green\[500\] -> 0x4CAF50
    /// <div style="padding: 10px; background-color: #4CAF50"></div>
    pub const fn c500(&self) -> u32 { 0x4CAF50 }
    /// Green\[600\] -> 0x43A047
    /// <div style="padding: 10px; background-color: #43A047"></div>
    pub const fn c600(&self) -> u32 { 0x43A047 }
    /// Green\[700\] -> 0x388E3C
    /// <div style="padding: 10px; background-color: #388E3C"></div>
    pub const fn c700(&self) -> u32 { 0x388E3C }
    /// Green\[800\] -> 0x2E7D32
    /// <div style="padding: 10px; background-color: #2E7D32"></div>
    pub const fn c800(&self) -> u32 { 0x2E7D32 }
    /// Green\[900\] -> 0x1B5E20
    /// <div style="padding: 10px; background-color: #1B5E20"></div>
    pub const fn c900(&self) -> u32 { 0x1B5E20 }
    /// Green\[A100\] -> 0xB9F6CA
    /// <div style="padding: 10px; background-color: #B9F6CA"></div>
    pub const fn a100(&self) -> u32 { 0xB9F6CA }
    /// Green\[A200\] -> 0x69F0AE
    /// <div style="padding: 10px; background-color: #69F0AE"></div>
    pub const fn a200(&self) -> u32 { 0x69F0AE }
    /// Green\[A400\] -> 0x00E676
    /// <div style="padding: 10px; background-color: #00E676"></div>
    pub const fn a400(&self) -> u32 { 0x00E676 }
    /// Green\[A700\] -> 0x00C853
    /// <div style="padding: 10px; background-color: #00C853"></div>
    pub const fn a700(&self) -> u32 { 0x00C853 }
}

/// Light Green color and its accents
pub struct MatColorLightGreen;

#[rustfmt::skip]
impl MatColorLightGreen {
    /// LightGreen\[50\] -> 0xF1F8E9
    /// <div style="padding: 10px; background-color: #F1F8E9"></div>
    pub const fn c50(&self) -> u32 { 0xF1F8E9 }
    /// LightGreen\[100\] -> 0xDCEDC8
    /// <div style="padding: 10px; background-color: #DCEDC8"></div>
    pub const fn c100(&self) -> u32 { 0xDCEDC8 }
    /// LightGreen\[200\] -> 0xC5E1A5
    /// <div style="padding: 10px; background-color: #C5E1A5"></div>
    pub const fn c200(&self) -> u32 { 0xC5E1A5 }
    /// LightGreen\[300\] -> 0xAED581
    /// <div style="padding: 10px; background-color: #AED581"></div>
    pub const fn c300(&self) -> u32 { 0xAED581 }
    /// LightGreen\[400\] -> 0x9CCC65
    /// <div style="padding: 10px; background-color: #9CCC65"></div>
    pub const fn c400(&self) -> u32 { 0x9CCC65 }
    /// LightGreen\[500\] -> 0x8BC34A
    /// <div style="padding: 10px; background-color: #8BC34A"></div>
    pub const fn c500(&self) -> u32 { 0x8BC34A }
    /// LightGreen\[600\] -> 0x7CB342
    /// <div style="padding: 10px; background-color: #7CB342"></div>
    pub const fn c600(&self) -> u32 { 0x7CB342 }
    /// LightGreen\[700\] -> 0x689F38
    /// <div style="padding: 10px; background-color: #689F38"></div>
    pub const fn c700(&self) -> u32 { 0x689F38 }
    /// LightGreen\[800\] -> 0x558B2F
    /// <div style="padding: 10px; background-color: #558B2F"></div>
    pub const fn c800(&self) -> u32 { 0x558B2F }
    /// LightGreen\[900\] -> 0x33691E
    /// <div style="padding: 10px; background-color: #33691E"></div>
    pub const fn c900(&self) -> u32 { 0x33691E }
    /// LightGreen\[A100\] -> 0xCCFF90
    /// <div style="padding: 10px; background-color: #CCFF90"></div>
    pub const fn a100(&self) -> u32 { 0xCCFF90 }
    /// LightGreen\[A200\] -> 0xB2FF59
    /// <div style="padding: 10px; background-color: #B2FF59"></div>
    pub const fn a200(&self) -> u32 { 0xB2FF59 }
    /// LightGreen\[A400\] -> 0x76FF03
    /// <div style="padding: 10px; background-color: #76FF03"></div>
    pub const fn a400(&self) -> u32 { 0x76FF03 }
    /// LightGreen\[A700\] -> 0x64DD17
    /// <div style="padding: 10px; background-color: #64DD17"></div>
    pub const fn a700(&self) -> u32 { 0x64DD17 }
}

/// Lime color and its accents
pub struct MatColorLime;

#[rustfmt::skip]
impl MatColorLime {
    /// Lime\[50\] -> 0xF9FBE7
    /// <div style="padding: 10px; background-color: #F9FBE7"></div>
    pub const fn c50(&self) -> u32 { 0xF9FBE7 }
    /// Lime\[100\] -> 0xF0F4C3
    /// <div style="padding: 10px; background-color: #F0F4C3"></div>
    pub const fn c100(&self) -> u32 { 0xF0F4C3 }
    /// Lime\[200\] -> 0xE6EE9C
    /// <div style="padding: 10px; background-color: #E6EE9C"></div>
    pub const fn c200(&self) -> u32 { 0xE6EE9C }
    /// Lime\[300\] -> 0xDCE775
    /// <div style="padding: 10px; background-color: #DCE775"></div>
    pub const fn c300(&self) -> u32 { 0xDCE775 }
    /// Lime\[400\] -> 0xD4E157
    /// <div style="padding: 10px; background-color: #D4E157"></div>
    pub const fn c400(&self) -> u32 { 0xD4E157 }
    /// Lime\[500\] -> 0xCDDC39
    /// <div style="padding: 10px; background-color: #CDDC39"></div>
    pub const fn c500(&self) -> u32 { 0xCDDC39 }
    /// Lime\[600\] -> 0xC0CA33
    /// <div style="padding: 10px; background-color: #C0CA33"></div>
    pub const fn c600(&self) -> u32 { 0xC0CA33 }
    /// Lime\[700\] -> 0xAFB42B
    /// <div style="padding: 10px; background-color: #AFB42B"></div>
    pub const fn c700(&self) -> u32 { 0xAFB42B }
    /// Lime\[800\] -> 0x9E9D24
    /// <div style="padding: 10px; background-color: #9E9D24"></div>
    pub const fn c800(&self) -> u32 { 0x9E9D24 }
    /// Lime\[900\] -> 0x827717
    /// <div style="padding: 10px; background-color: #827717"></div>
    pub const fn c900(&self) -> u32 { 0x827717 }
    /// Lime\[A100\] -> 0xF4FF81
    /// <div style="padding: 10px; background-color: #F4FF81"></div>
    pub const fn a100(&self) -> u32 { 0xF4FF81 }
    /// Lime\[A200\] -> 0xEEFF41
    /// <div style="padding: 10px; background-color: #EEFF41"></div>
    pub const fn a200(&self) -> u32 { 0xEEFF41 }
    /// Lime\[A400\] -> 0xC6FF00
    /// <div style="padding: 10px; background-color: #C6FF00"></div>
    pub const fn a400(&self) -> u32 { 0xC6FF00 }
    /// Lime\[A700\] -> 0xAEEA00
    /// <div style="padding: 10px; background-color: #AEEA00"></div>
    pub const fn a700(&self) -> u32 { 0xAEEA00 }
}

/// Yellow color and its accents
pub struct MatColorYellow;

#[rustfmt::skip]
impl MatColorYellow {
    /// Yellow\[50\] -> 0xFFFDE7
    /// <div style="padding: 10px; background-color: #FFFDE7"></div>
    pub const fn c50(&self) -> u32 { 0xFFFDE7 }
    /// Yellow\[100\] -> 0xFFF9C4
    /// <div style="padding: 10px; background-color: #FFF9C4"></div>
    pub const fn c100(&self) -> u32 { 0xFFF9C4 }
    /// Yellow\[200\] -> 0xFFF59D
    /// <div style="padding: 10px; background-color: #FFF59D"></div>
    pub const fn c200(&self) -> u32 { 0xFFF59D }
    /// Yellow\[300\] -> 0xFFF176
    /// <div style="padding: 10px; background-color: #FFF176"></div>
    pub const fn c300(&self) -> u32 { 0xFFF176 }
    /// Yellow\[400\] -> 0xFFEE58
    /// <div style="padding: 10px; background-color: #FFEE58"></div>
    pub const fn c400(&self) -> u32 { 0xFFEE58 }
    /// Yellow\[500\] -> 0xFFEB3B
    /// <div style="padding: 10px; background-color: #FFEB3B"></div>
    pub const fn c500(&self) -> u32 { 0xFFEB3B }
    /// Yellow\[600\] -> 0xFDD835
    /// <div style="padding: 10px; background-color: #FDD835"></div>
    pub const fn c600(&self) -> u32 { 0xFDD835 }
    /// Yellow\[700\] -> 0xFBC02D
    /// <div style="padding: 10px; background-color: #FBC02D"></div>
    pub const fn c700(&self) -> u32 { 0xFBC02D }
    /// Yellow\[800\] -> 0xF9A825
    /// <div style="padding: 10px; background-color: #F9A825"></div>
    pub const fn c800(&self) -> u32 { 0xF9A825 }
    /// Yellow\[900\] -> 0xF57F17
    /// <div style="padding: 10px; background-color: #F57F17"></div>
    pub const fn c900(&self) -> u32 { 0xF57F17 }
    /// Yellow\[A100\] -> 0xFFFF8D
    /// <div style="padding: 10px; background-color: #FFFF8D"></div>
    pub const fn a100(&self) -> u32 { 0xFFFF8D }
    /// Yellow\[A200\] -> 0xFFFF00
    /// <div style="padding: 10px; background-color: #FFFF00"></div>
    pub const fn a200(&self) -> u32 { 0xFFFF00 }
    /// Yellow\[A400\] -> 0xFFEA00
    /// <div style="padding: 10px; background-color: #FFEA00"></div>
    pub const fn a400(&self) -> u32 { 0xFFEA00 }
    /// Yellow\[A700\] -> 0xFFD600
    /// <div style="padding: 10px; background-color: #FFD600"></div>
    pub const fn a700(&self) -> u32 { 0xFFD600 }
}

/// Amber color and its accents
pub struct MatColorAmber;

#[rustfmt::skip]
impl MatColorAmber {
    /// Amber\[50\] -> 0xFFF8E1
    /// <div style="padding: 10px; background-color: #FFF8E1"></div>
    pub const fn c50(&self) -> u32 { 0xFFF8E1 }
    /// Amber\[100\] -> 0xFFECB3
    /// <div style="padding: 10px; background-color: #FFECB3"></div>
    pub const fn c100(&self) -> u32 { 0xFFECB3 }
    /// Amber\[200\] -> 0xFFE082
    /// <div style="padding: 10px; background-color: #FFE082"></div>
    pub const fn c200(&self) -> u32 { 0xFFE082 }
    /// Amber\[300\] -> 0xFFD54F
    /// <div style="padding: 10px; background-color: #FFD54F"></div>
    pub const fn c300(&self) -> u32 { 0xFFD54F }
    /// Amber\[400\] -> 0xFFCA28
    /// <div style="padding: 10px; background-color: #FFCA28"></div>
    pub const fn c400(&self) -> u32 { 0xFFCA28 }
    /// Amber\[500\] -> 0xFFC107
    /// <div style="padding: 10px; background-color: #FFC107"></div>
    pub const fn c500(&self) -> u32 { 0xFFC107 }
    /// Amber\[600\] -> 0xFFB300
    /// <div style="padding: 10px; background-color: #FFB300"></div>
    pub const fn c600(&self) -> u32 { 0xFFB300 }
    /// Amber\[700\] -> 0xFFA000
    /// <div style="padding: 10px; background-color: #FFA000"></div>
    pub const fn c700(&self) -> u32 { 0xFFA000 }
    /// Amber\[800\] -> 0xFF8F00
    /// <div style="padding: 10px; background-color: #FF8F00"></div>
    pub const fn c800(&self) -> u32 { 0xFF8F00 }
    /// Amber\[900\] -> 0xFF6F00
    /// <div style="padding: 10px; background-color: #FF6F00"></div>
    pub const fn c900(&self) -> u32 { 0xFF6F00 }
    /// Amber\[A100\] -> 0xFFE57F
    /// <div style="padding: 10px; background-color: #FFE57F"></div>
    pub const fn a100(&self) -> u32 { 0xFFE57F }
    /// Amber\[A200\] -> 0xFFD740
    /// <div style="padding: 10px; background-color: #FFD740"></div>
    pub const fn a200(&self) -> u32 { 0xFFD740 }
    /// Amber\[A400\] -> 0xFFC400
    /// <div style="padding: 10px; background-color: #FFC400"></div>
    pub const fn a400(&self) -> u32 { 0xFFC400 }
    /// Amber\[A700\] -> 0xFFAB00
    /// <div style="padding: 10px; background-color: #FFAB00"></div>
    pub const fn a700(&self) -> u32 { 0xFFAB00 }
}

/// Orange color and its accents
pub struct MatColorOrange;

#[rustfmt::skip]
impl MatColorOrange {
    /// Orange\[50\] -> 0xFFF3E0
    /// <div style="padding: 10px; background-color: #FFF3E0"></div>
    pub const fn c50(&self) -> u32 { 0xFFF3E0 }
    /// Orange\[100\] -> 0xFFE0B2
    /// <div style="padding: 10px; background-color: #FFE0B2"></div>
    pub const fn c100(&self) -> u32 { 0xFFE0B2 }
    /// Orange\[200\] -> 0xFFCC80
    /// <div style="padding: 10px; background-color: #FFCC80"></div>
    pub const fn c200(&self) -> u32 { 0xFFCC80 }
    /// Orange\[300\] -> 0xFFB74D
    /// <div style="padding: 10px; background-color: #FFB74D"></div>
    pub const fn c300(&self) -> u32 { 0xFFB74D }
    /// Orange\[400\] -> 0xFFA726
    /// <div style="padding: 10px; background-color: #FFA726"></div>
    pub const fn c400(&self) -> u32 { 0xFFA726 }
    /// Orange\[500\] -> 0xFF9800
    /// <div style="padding: 10px; background-color: #FF9800"></div>
    pub const fn c500(&self) -> u32 { 0xFF9800 }
    /// Orange\[600\] -> 0xFB8C00
    /// <div style="padding: 10px; background-color: #FB8C00"></div>
    pub const fn c600(&self) -> u32 { 0xFB8C00 }
    /// Orange\[700\] -> 0xF57C00
    /// <div style="padding: 10px; background-color: #F57C00"></div>
    pub const fn c700(&self) -> u32 { 0xF57C00 }
    /// Orange\[800\] -> 0xEF6C00
    /// <div style="padding: 10px; background-color: #EF6C00"></div>
    pub const fn c800(&self) -> u32 { 0xEF6C00 }
    /// Orange\[900\] -> 0xE65100
    /// <div style="padding: 10px; background-color: #E65100"></div>
    pub const fn c900(&self) -> u32 { 0xE65100 }
    /// Orange\[A100\] -> 0xFFD180
    /// <div style="padding: 10px; background-color: #FFD180"></div>
    pub const fn a100(&self) -> u32 { 0xFFD180 }
    /// Orange\[A200\] -> 0xFFAB40
    /// <div style="padding: 10px; background-color: #FFAB40"></div>
    pub const fn a200(&self) -> u32 { 0xFFAB40 }
    /// Orange\[A400\] -> 0xFF9100
    /// <div style="padding: 10px; background-color: #FF9100"></div>
    pub const fn a400(&self) -> u32 { 0xFF9100 }
    /// Orange\[A700\] -> 0xFF6D00
    /// <div style="padding: 10px; background-color: #FF6D00"></div>
    pub const fn a700(&self) -> u32 { 0xFF6D00 }
}

/// Deep Orange color and its accents
pub struct MatColorDeepOrange;

#[rustfmt::skip]
impl MatColorDeepOrange {
    /// DeepOrange\[50\] -> 0xFBE9E7
    /// <div style="padding: 10px; background-color: #FBE9E7"></div>
    pub const fn c50(&self) -> u32 { 0xFBE9E7 }
    /// DeepOrange\[100\] -> 0xFFCCBC
    /// <div style="padding: 10px; background-color: #FFCCBC"></div>
    pub const fn c100(&self) -> u32 { 0xFFCCBC }
    /// DeepOrange\[200\] -> 0xFFAB91
    /// <div style="padding: 10px; background-color: #FFAB91"></div>
    pub const fn c200(&self) -> u32 { 0xFFAB91 }
    /// DeepOrange\[300\] -> 0xFF8A65
    /// <div style="padding: 10px; background-color: #FF8A65"></div>
    pub const fn c300(&self) -> u32 { 0xFF8A65 }
    /// DeepOrange\[400\] -> 0xFF7043
    /// <div style="padding: 10px; background-color: #FF7043"></div>
    pub const fn c400(&self) -> u32 { 0xFF7043 }
    /// DeepOrange\[500\] -> 0xFF5722
    /// <div style="padding: 10px; background-color: #FF5722"></div>
    pub const fn c500(&self) -> u32 { 0xFF5722 }
    /// DeepOrange\[600\] -> 0xF4511E
    /// <div style="padding: 10px; background-color: #F4511E"></div>
    pub const fn c600(&self) -> u32 { 0xF4511E }
    /// DeepOrange\[700\] -> 0xE64A19
    /// <div style="padding: 10px; background-color: #E64A19"></div>
    pub const fn c700(&self) -> u32 { 0xE64A19 }
    /// DeepOrange\[800\] -> 0xD84315
    /// <div style="padding: 10px; background-color: #D84315"></div>
    pub const fn c800(&self) -> u32 { 0xD84315 }
    /// DeepOrange\[900\] -> 0xBF360C
    /// <div style="padding: 10px; background-color: #BF360C"></div>
    pub const fn c900(&self) -> u32 { 0xBF360C }
    /// DeepOrange\[A100\] -> 0xFF9E80
    /// <div style="padding: 10px; background-color: #FF9E80"></div>
    pub const fn a100(&self) -> u32 { 0xFF9E80 }
    /// DeepOrange\[A200\] -> 0xFF6E40
    /// <div style="padding: 10px; background-color: #FF6E40"></div>
    pub const fn a200(&self) -> u32 { 0xFF6E40 }
    /// DeepOrange\[A400\] -> 0xFF3D00
    /// <div style="padding: 10px; background-color: #FF3D00"></div>
    pub const fn a400(&self) -> u32 { 0xFF3D00 }
    /// DeepOrange\[A700\] -> 0xDD2C00
    /// <div style="padding: 10px; background-color: #DD2C00"></div>
    pub const fn a700(&self) -> u32 { 0xDD2C00 }
}

/// Brown color and its accents
pub struct MatColorBrown;

#[rustfmt::skip]
impl MatColorBrown {
    /// Brown\[50\] -> 0xEFEBE9
    /// <div style="padding: 10px; background-color: #EFEBE9"></div>
    pub const fn c50(&self) -> u32 { 0xEFEBE9 }
    /// Brown\[100\] -> 0xD7CCC8
    /// <div style="padding: 10px; background-color: #D7CCC8"></div>
    pub const fn c100(&self) -> u32 { 0xD7CCC8 }
    /// Brown\[200\] -> 0xBCAAA4
    /// <div style="padding: 10px; background-color: #BCAAA4"></div>
    pub const fn c200(&self) -> u32 { 0xBCAAA4 }
    /// Brown\[300\] -> 0xA1887F
    /// <div style="padding: 10px; background-color: #A1887F"></div>
    pub const fn c300(&self) -> u32 { 0xA1887F }
    /// Brown\[400\] -> 0x8D6E63
    /// <div style="padding: 10px; background-color: #8D6E63"></div>
    pub const fn c400(&self) -> u32 { 0x8D6E63 }
    /// Brown\[500\] -> 0x795548
    /// <div style="padding: 10px; background-color: #795548"></div>
    pub const fn c500(&self) -> u32 { 0x795548 }
    /// Brown\[600\] -> 0x6D4C41
    /// <div style="padding: 10px; background-color: #6D4C41"></div>
    pub const fn c600(&self) -> u32 { 0x6D4C41 }
    /// Brown\[700\] -> 0x5D4037
    /// <div style="padding: 10px; background-color: #5D4037"></div>
    pub const fn c700(&self) -> u32 { 0x5D4037 }
    /// Brown\[800\] -> 0x4E342E
    /// <div style="padding: 10px; background-color: #4E342E"></div>
    pub const fn c800(&self) -> u32 { 0x4E342E }
    /// Brown\[900\] -> 0x3E2723
    /// <div style="padding: 10px; background-color: #3E2723"></div>
    pub const fn c900(&self) -> u32 { 0x3E2723 }
}

/// Gray color and its accents
pub struct MatColorGray;

#[rustfmt::skip]
impl MatColorGray {
    /// Gray\[50\] -> 0xFAFAFA
    /// <div style="padding: 10px; background-color: #FAFAFA"></div>
    pub const fn c50(&self) -> u32 { 0xFAFAFA }
    /// Gray\[100\] -> 0xF5F5F5
    /// <div style="padding: 10px; background-color: #F5F5F5"></div>
    pub const fn c100(&self) -> u32 { 0xF5F5F5 }
    /// Gray\[200\] -> 0xEEEEEE
    /// <div style="padding: 10px; background-color: #EEEEEE"></div>
    pub const fn c200(&self) -> u32 { 0xEEEEEE }
    /// Gray\[300\] -> 0xE0E0E0
    /// <div style="padding: 10px; background-color: #E0E0E0"></div>
    pub const fn c300(&self) -> u32 { 0xE0E0E0 }
    /// Gray\[400\] -> 0xBDBDBD
    /// <div style="padding: 10px; background-color: #BDBDBD"></div>
    pub const fn c400(&self) -> u32 { 0xBDBDBD }
    /// Gray\[500\] -> 0x9E9E9E
    /// <div style="padding: 10px; background-color: #9E9E9E"></div>
    pub const fn c500(&self) -> u32 { 0x9E9E9E }
    /// Gray\[600\] -> 0x757575
    /// <div style="padding: 10px; background-color: #757575"></div>
    pub const fn c600(&self) -> u32 { 0x757575 }
    /// Gray\[700\] -> 0x616161
    /// <div style="padding: 10px; background-color: #616161"></div>
    pub const fn c700(&self) -> u32 { 0x616161 }
    /// Gray\[800\] -> 0x424242
    /// <div style="padding: 10px; background-color: #424242"></div>
    pub const fn c800(&self) -> u32 { 0x424242 }
    /// Gray\[900\] -> 0x212121
    /// <div style="padding: 10px; background-color: #212121"></div>
    pub const fn c900(&self) -> u32 { 0x212121 }
}

/// Blue Gray color and its accents
pub struct MatColorBlueGray;

#[rustfmt::skip]
impl MatColorBlueGray {
    /// BlueGray\[50\] -> 0xECEFF1
    /// <div style="padding: 10px; background-color: #ECEFF1"></div>
    pub const fn c50(&self) -> u32 { 0xECEFF1 }
    /// BlueGray\[100\] -> 0xCFD8DC
    /// <div style="padding: 10px; background-color: #CFD8DC"></div>
    pub const fn c100(&self) -> u32 { 0xCFD8DC }
    /// BlueGray\[200\] -> 0xB0BEC5
    /// <div style="padding: 10px; background-color: #B0BEC5"></div>
    pub const fn c200(&self) -> u32 { 0xB0BEC5 }
    /// BlueGray\[300\] -> 0x90A4AE
    /// <div style="padding: 10px; background-color: #90A4AE"></div>
    pub const fn c300(&self) -> u32 { 0x90A4AE }
    /// BlueGray\[400\] -> 0x78909C
    /// <div style="padding: 10px; background-color: #78909C"></div>
    pub const fn c400(&self) -> u32 { 0x78909C }
    /// BlueGray\[500\] -> 0x607D8B
    /// <div style="padding: 10px; background-color: #607D8B"></div>
    pub const fn c500(&self) -> u32 { 0x607D8B }
    /// BlueGray\[600\] -> 0x546E7A
    /// <div style="padding: 10px; background-color: #546E7A"></div>
    pub const fn c600(&self) -> u32 { 0x546E7A }
    /// BlueGray\[700\] -> 0x455A64
    /// <div style="padding: 10px; background-color: #455A64"></div>
    pub const fn c700(&self) -> u32 { 0x455A64 }
    /// BlueGray\[800\] -> 0x37474F
    /// <div style="padding: 10px; background-color: #37474F"></div>
    pub const fn c800(&self) -> u32 { 0x37474F }
    /// BlueGray\[900\] -> 0x263238
    /// <div style="padding: 10px; background-color: #263238"></div>
    pub const fn c900(&self) -> u32 { 0x263238 }
}

impl MatColor {
    /// Creates a color from a [`MatColorVariant`] and an accent [`MatColorAccent`]
    /// checks if its combination is valid. For example, not every color has
    /// every type of accent. Prefer using the `const fn` variants if you can.
    /// In case of [`MatColorVariant::Black`] or [`MatColorVariant::White`], the
    /// accent value is ignored.
    pub fn new(color: MatColorVariant, accent: MatColorAccent) -> Option<u32> {
        match color {
            MatColorVariant::Black => Some(Self.black()),
            MatColorVariant::White => Some(Self.white()),
            MatColorVariant::Brown | MatColorVariant::Gray | MatColorVariant::BlueGray => {
                if accent as usize > MatColorAccent::C900 as usize {
                    None
                } else {
                    Some(MAT_COLORS_NO_ACCENT[color as usize - 16][accent as usize])
                }
            }
            _ => Some(MAT_COLORS_ACCENT[color as usize][accent as usize]),
        }
    }

    /// Access to [`MatColorRed`]
    pub const fn red(&self) -> MatColorRed {
        MatColorRed
    }
    /// Access to [`MatColorPink`]
    pub const fn pink(&self) -> MatColorPink {
        MatColorPink
    }
    /// Access to [`MatColorPurple`]
    pub const fn purple(&self) -> MatColorPurple {
        MatColorPurple
    }
    /// Access to [`MatColorDeepPurple`]
    pub const fn deep_purple(&self) -> MatColorDeepPurple {
        MatColorDeepPurple
    }
    /// Access to [`MatColorIndigo`]
    pub const fn indigo(&self) -> MatColorIndigo {
        MatColorIndigo
    }
    /// Access to [`MatColorBlue`]
    pub const fn blue(&self) -> MatColorBlue {
        MatColorBlue
    }
    /// Access to [`MatColorLightBlue`]
    pub const fn light_blue(&self) -> MatColorLightBlue {
        MatColorLightBlue
    }
    /// Access to [`MatColorCyan`]
    pub const fn cyan(&self) -> MatColorCyan {
        MatColorCyan
    }
    /// Access to [`MatColorTeal`]
    pub const fn teal(&self) -> MatColorTeal {
        MatColorTeal
    }
    /// Access to [`MatColorGreen`]
    pub const fn green(&self) -> MatColorGreen {
        MatColorGreen
    }
    /// Access to [`MatColorLightGreen`]
    pub const fn light_green(&self) -> MatColorLightGreen {
        MatColorLightGreen
    }
    /// Access to [`MatColorLime`]
    pub const fn lime(&self) -> MatColorLime {
        MatColorLime
    }
    /// Access to [`MatColorYellow`]
    pub const fn yellow(&self) -> MatColorYellow {
        MatColorYellow
    }
    /// Access to [`MatColorAmber`]
    pub const fn amber(&self) -> MatColorAmber {
        MatColorAmber
    }
    /// Access to [`MatColorOrange`]
    pub const fn orange(&self) -> MatColorOrange {
        MatColorOrange
    }
    /// Access to [`MatColorDeepOrange`]
    pub const fn deep_orange(&self) -> MatColorDeepOrange {
        MatColorDeepOrange
    }
    /// Access to [`MatColorBrown`]
    pub const fn brown(&self) -> MatColorBrown {
        MatColorBrown
    }
    /// Access to [`MatColorGray`]
    pub const fn gray(&self) -> MatColorGray {
        MatColorGray
    }
    /// Access to [`MatColorBlueGray`]
    pub const fn blue_gray(&self) -> MatColorBlueGray {
        MatColorBlueGray
    }
    /// Black -> 0x000000
    /// <div style="padding: 10px; background-color: #000000"></div>
    pub const fn black(&self) -> u32 {
        0x000000
    }
    /// White -> 0xFFFFFF
    /// <div style="padding: 10px; background-color: #FFFFFF"></div>
    pub const fn white(&self) -> u32 {
        0xFFFFFF
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;
    use strum::IntoEnumIterator;

    #[test]
    fn it_works() {
        let all_colors = MatColorVariant::iter()
            .flat_map(|var| MatColorAccent::iter().map(move |acc| (MatColor::new(var, acc), var, acc)))
            .filter(|tup| tup.0.is_some())
            .filter(|tup| !((tup.1 == MatColorVariant::Black || tup.1 == MatColorVariant::White) && tup.2 != MatColorAccent::C50))
            .map(|tup| (tup.0.unwrap(), tup.1, tup.2))
            .collect::<HashSet<(u32, MatColorVariant, MatColorAccent)>>();

        assert_eq!(256, all_colors.len());

        // TODO test for each color variant
    }
}
