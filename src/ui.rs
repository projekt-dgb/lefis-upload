//! Auto-generated UI source code
#![allow(unused_imports)]

use azul::widgets::{FileInput, TabHeaderState, FileInputState, CheckBoxState, TextInputState, NumberInputState};
use azul::task::{ThreadSender, ThreadReceiver};
use azul::vec::U8Vec;
use crate::{LefisUploadKonfiguration, exec_sync};
use crate::wsdl::{ProtokollMsg, AuftragsManager, RequestFailure};
use crate::GeladeneVerfahren;

const IMG_DONE: &[u8] = include_bytes!("../img/icons8-done-48.png");
const IMG_CLOSE: &[u8] = include_bytes!("../img/icons8-close-48.png");
const IMG_DONE_VEC: U8Vec = U8Vec::from_const_slice(IMG_DONE);
const IMG_CLOSE_VEC: U8Vec = U8Vec::from_const_slice(IMG_CLOSE);

pub mod components {
    #[allow(unused_imports)]
    pub mod body {
        use azul::dom::Dom;
        use azul::str::String as AzString;
        #[inline]
        pub fn render() -> Dom {
            Dom::body()
        }
    }
    
    #[allow(unused_imports)]
    pub mod div {
        use azul::dom::Dom;
        use azul::str::String as AzString;
        #[inline]
        pub fn render() -> Dom {
            Dom::div()
        }
    }
    
    #[allow(unused_imports)]
    pub mod p {
        use azul::dom::Dom;
        use azul::str::String as AzString;
        #[inline]
        pub fn render(text: AzString) -> Dom {
            Dom::text(text)
        }
    }
}

pub use crate::ui::components::*;
use crate::{VerfahrenGeladen, AppData};
use azul::css::*;
use azul::str::String as AzString;
use azul::vec::{
    DomVec, IdOrClassVec, NodeDataInlineCssPropertyVec,
    StyleBackgroundSizeVec, StyleBackgroundRepeatVec,
    StyleBackgroundContentVec, StyleTransformVec,
    StyleFontFamilyVec, StyleBackgroundPositionVec,
    NormalizedLinearColorStopVec, NormalizedRadialColorStopVec,
};
use azul::dom::{
    Dom, IdOrClass, TabIndex,
    IdOrClass::{Id, Class},
    NodeDataInlineCssProperty,
};
use self::styles::*;
use azul::callbacks::{RefAny, CallbackInfo, Update, Callback};
use azul::dom::{EventFilter, HoverEventFilter, CallbackData};

#[allow(unused_imports)]
mod styles {

    use azul::css::*;
    use azul::str::String as AzString;
    use azul::vec::{
        DomVec, IdOrClassVec, NodeDataInlineCssPropertyVec,
        StyleBackgroundSizeVec, StyleBackgroundRepeatVec,
        StyleBackgroundContentVec, StyleTransformVec,
        StyleFontFamilyVec, StyleBackgroundPositionVec,
        NormalizedLinearColorStopVec, NormalizedRadialColorStopVec,
    };
    use azul::dom::{
        Dom, IdOrClass, TabIndex,
        IdOrClass::{Id, Class},
        NodeDataInlineCssProperty,
    };

    pub(in super) const STRING_9801366799028015987: AzString = AzString::from_const_str("Segoe UI");
    pub(in super) const STRING_12078601221173113980: AzString = AzString::from_const_str("Segoe UI Bold");
    pub(in super) const STRING_16146701490593874959: AzString = AzString::from_const_str("sans-serif");
    pub(in super) const STYLE_BACKGROUND_CONTENT_605821317762015437_ITEMS: &[StyleBackgroundContent] = &[
        StyleBackgroundContent::Color(ColorU { r: 239, g: 239, b: 239, a: 255 })
    ];
    pub(in super) const STYLE_BACKGROUND_CONTENT_2353255066510409165_ITEMS: &[StyleBackgroundContent] = &[
        StyleBackgroundContent::Color(ColorU { r: 255, g: 0, b: 0, a: 255 })
    ];
    pub(in super) const STYLE_BACKGROUND_CONTENT_9425389510111643135_ITEMS: &[StyleBackgroundContent] = &[
        StyleBackgroundContent::LinearGradient(LinearGradient {
            direction: Direction::FromTo(DirectionCorners { from: DirectionCorner::Top, to: DirectionCorner::Bottom }),
            extend_mode: ExtendMode::Clamp,
            stops: NormalizedLinearColorStopVec::from_const_slice(LINEAR_COLOR_STOP_14019419773578374595_ITEMS),
        })
    ];
    pub(in super) const STYLE_BACKGROUND_CONTENT_12667596745911411196_ITEMS: &[StyleBackgroundContent] = &[
        StyleBackgroundContent::LinearGradient(LinearGradient {
            direction: Direction::FromTo(DirectionCorners { from: DirectionCorner::Top, to: DirectionCorner::Bottom }),
            extend_mode: ExtendMode::Clamp,
            stops: NormalizedLinearColorStopVec::from_const_slice(LINEAR_COLOR_STOP_17879764291917296390_ITEMS),
        })
    ];
    pub(in super) const STYLE_BACKGROUND_CONTENT_13046214508323908618_ITEMS: &[StyleBackgroundContent] = &[
        StyleBackgroundContent::LinearGradient(LinearGradient {
            direction: Direction::FromTo(DirectionCorners { from: DirectionCorner::Top, to: DirectionCorner::Bottom }),
            extend_mode: ExtendMode::Clamp,
            stops: NormalizedLinearColorStopVec::from_const_slice(LINEAR_COLOR_STOP_5390058917785529581_ITEMS),
        })
    ];
    pub(in super) const STYLE_BACKGROUND_CONTENT_14631383723043813572_ITEMS: &[StyleBackgroundContent] = &[
        StyleBackgroundContent::LinearGradient(LinearGradient {
            direction: Direction::FromTo(DirectionCorners { from: DirectionCorner::Top, to: DirectionCorner::Bottom }),
            extend_mode: ExtendMode::Clamp,
            stops: NormalizedLinearColorStopVec::from_const_slice(LINEAR_COLOR_STOP_650287208941589230_ITEMS),
        })
    ];
    pub(in super) const STYLE_BACKGROUND_CONTENT_15133467471352813994_ITEMS: &[StyleBackgroundContent] = &[
        StyleBackgroundContent::LinearGradient(LinearGradient {
            direction: Direction::FromTo(DirectionCorners { from: DirectionCorner::Top, to: DirectionCorner::Bottom }),
            extend_mode: ExtendMode::Clamp,
            stops: NormalizedLinearColorStopVec::from_const_slice(LINEAR_COLOR_STOP_9453686362801359926_ITEMS),
        })
    ];
    pub(in super) const STYLE_BACKGROUND_CONTENT_16418392607769947338_ITEMS: &[StyleBackgroundContent] = &[
        StyleBackgroundContent::Color(ColorU { r: 255, g: 255, b: 255, a: 255 })
    ];
    pub(in super) const STYLE_BACKGROUND_CONTENT_18282593039276133616_ITEMS: &[StyleBackgroundContent] = &[
        StyleBackgroundContent::LinearGradient(LinearGradient {
            direction: Direction::FromTo(DirectionCorners { from: DirectionCorner::Top, to: DirectionCorner::Bottom }),
            extend_mode: ExtendMode::Clamp,
            stops: NormalizedLinearColorStopVec::from_const_slice(LINEAR_COLOR_STOP_1896825754186205459_ITEMS),
        })
    ];
    pub(in super) const STYLE_FONT_FAMILY_4570983119824751714_ITEMS: &[StyleFontFamily] = &[
        StyleFontFamily::System(STRING_12078601221173113980)
    ];
    pub(in super) const STYLE_FONT_FAMILY_10134585962653230168_ITEMS: &[StyleFontFamily] = &[
        StyleFontFamily::System(STRING_16146701490593874959)
    ];
    pub(in super) const STYLE_FONT_FAMILY_10582572158086033001_ITEMS: &[StyleFontFamily] = &[
        StyleFontFamily::System(STRING_9801366799028015987)
    ];
    pub(in super) const LINEAR_COLOR_STOP_650287208941589230_ITEMS: &[NormalizedLinearColorStop] = &[
        NormalizedLinearColorStop { offset: PercentageValue::const_new(0), color: ColorU { r: 114, g: 160, b: 205, a: 255 } },
    NormalizedLinearColorStop { offset: PercentageValue::const_new(100), color: ColorU { r: 78, g: 143, b: 207, a: 255 } }
    ];
    pub(in super) const LINEAR_COLOR_STOP_1896825754186205459_ITEMS: &[NormalizedLinearColorStop] = &[
        NormalizedLinearColorStop { offset: PercentageValue::const_new(0), color: ColorU { r: 25, g: 189, b: 158, a: 255 } },
    NormalizedLinearColorStop { offset: PercentageValue::const_new(100), color: ColorU { r: 55, g: 128, b: 110, a: 255 } }
    ];
    pub(in super) const LINEAR_COLOR_STOP_5390058917785529581_ITEMS: &[NormalizedLinearColorStop] = &[
        NormalizedLinearColorStop { offset: PercentageValue::const_new(0), color: ColorU { r: 190, g: 190, b: 190, a: 255 } },
    NormalizedLinearColorStop { offset: PercentageValue::const_new(100), color: ColorU { r: 216, g: 216, b: 216, a: 255 } }
    ];
    pub(in super) const LINEAR_COLOR_STOP_9453686362801359926_ITEMS: &[NormalizedLinearColorStop] = &[
        NormalizedLinearColorStop { offset: PercentageValue::const_new(0), color: ColorU { r: 252, g: 252, b: 252, a: 255 } },
    NormalizedLinearColorStop { offset: PercentageValue::const_new(100), color: ColorU { r: 239, g: 239, b: 239, a: 255 } }
    ];
    pub(in super) const LINEAR_COLOR_STOP_14019419773578374595_ITEMS: &[NormalizedLinearColorStop] = &[
        NormalizedLinearColorStop { offset: PercentageValue::const_new(0), color: ColorU { r: 200, g: 49, b: 46, a: 255 } },
    NormalizedLinearColorStop { offset: PercentageValue::const_new(100), color: ColorU { r: 200, g: 49, b: 46, a: 255 } }
    ];
    pub(in super) const LINEAR_COLOR_STOP_17879764291917296390_ITEMS: &[NormalizedLinearColorStop] = &[
        NormalizedLinearColorStop { offset: PercentageValue::const_new(0), color: ColorU { r: 248, g: 248, b: 248, a: 255 } },
    NormalizedLinearColorStop { offset: PercentageValue::const_new(100), color: ColorU { r: 220, g: 220, b: 220, a: 255 } }
    ];

    pub(in super) const CSS_MATCH_10644212216580784356_PROPERTIES: &[NodeDataInlineCssProperty] = &[
        // .ausgewaehltes-verfahren-titel
        NodeDataInlineCssProperty::Normal(CssProperty::MaxWidth(LayoutMaxWidthValue::Exact(LayoutMaxWidth { inner: PixelValue::const_px(600) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::FontSize(StyleFontSizeValue::Exact(StyleFontSize { inner: PixelValue::const_px(25) })))
    ];
    pub(in super) const CSS_MATCH_10644212216580784356: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_10644212216580784356_PROPERTIES);    

    pub(in super) const CSS_MATCH_11080677877874143473_PROPERTIES: &[NodeDataInlineCssProperty] = &[
        // .verfahrens-info-zeile p
        NodeDataInlineCssProperty::Normal(CssProperty::FlexGrow(LayoutFlexGrowValue::Exact(LayoutFlexGrow { inner: FloatValue::const_new(1) })))
    ];
    pub(in super) const CSS_MATCH_11080677877874143473: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_11080677877874143473_PROPERTIES);    

    pub(in super) const CSS_MATCH_11387455194806019534_PROPERTIES: &[NodeDataInlineCssProperty] = &[
        // .verfahren-info
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingRight(LayoutPaddingRightValue::Exact(LayoutPaddingRight { inner: PixelValue::const_px(0) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingLeft(LayoutPaddingLeftValue::Exact(LayoutPaddingLeft { inner: PixelValue::const_px(0) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingBottom(LayoutPaddingBottomValue::Exact(LayoutPaddingBottom { inner: PixelValue::const_px(10) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingTop(LayoutPaddingTopValue::Exact(LayoutPaddingTop { inner: PixelValue::const_px(10) })))
    ];
    pub(in super) const CSS_MATCH_11387455194806019534: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_11387455194806019534_PROPERTIES);    

    pub(in super) const CSS_MATCH_11661823484470247691_PROPERTIES: &[NodeDataInlineCssProperty] = &[
        // .status-log-fehler .header
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingBottom(LayoutPaddingBottomValue::Exact(LayoutPaddingBottom { inner: PixelValue::const_px(5) })))
    ];
    pub(in super) const CSS_MATCH_11661823484470247691: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_11661823484470247691_PROPERTIES);    

    pub(in super) const CSS_MATCH_12913589432591768438_PROPERTIES: &[NodeDataInlineCssProperty] = &[
        // .status-log-fehler
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingRight(LayoutPaddingRightValue::Exact(LayoutPaddingRight { inner: PixelValue::const_px(10) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingLeft(LayoutPaddingLeftValue::Exact(LayoutPaddingLeft { inner: PixelValue::const_px(10) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingBottom(LayoutPaddingBottomValue::Exact(LayoutPaddingBottom { inner: PixelValue::const_px(10) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingTop(LayoutPaddingTopValue::Exact(LayoutPaddingTop { inner: PixelValue::const_px(10) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::TextColor(StyleTextColorValue::Exact(StyleTextColor { inner: ColorU { r: 255, g: 255, b: 255, a: 255 } }))),
        NodeDataInlineCssProperty::Normal(CssProperty::BackgroundContent(StyleBackgroundContentVecValue::Exact(StyleBackgroundContentVec::from_const_slice(STYLE_BACKGROUND_CONTENT_9425389510111643135_ITEMS))))
    ];
    pub(in super) const CSS_MATCH_12913589432591768438: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_12913589432591768438_PROPERTIES);    

    pub(in super) const CSS_MATCH_13021692908862555904_PROPERTIES: &[NodeDataInlineCssProperty] = &[
        // .projekt-ueberschrift
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingRight(LayoutPaddingRightValue::Exact(LayoutPaddingRight { inner: PixelValue::const_px(10) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingLeft(LayoutPaddingLeftValue::Exact(LayoutPaddingLeft { inner: PixelValue::const_px(10) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingBottom(LayoutPaddingBottomValue::Exact(LayoutPaddingBottom { inner: PixelValue::const_px(5) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingTop(LayoutPaddingTopValue::Exact(LayoutPaddingTop { inner: PixelValue::const_px(5) })))
    ];
    pub(in super) const CSS_MATCH_13021692908862555904: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_13021692908862555904_PROPERTIES);    

    pub(in super) const CSS_MATCH_1313287635652259770_PROPERTIES: &[NodeDataInlineCssProperty] = &[
        // .verfahrens-info-zeile
        NodeDataInlineCssProperty::Normal(CssProperty::FlexGrow(LayoutFlexGrowValue::Exact(LayoutFlexGrow { inner: FloatValue::const_new(1) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::FlexDirection(LayoutFlexDirectionValue::Exact(LayoutFlexDirection::Row)))
    ];
    pub(in super) const CSS_MATCH_1313287635652259770: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_1313287635652259770_PROPERTIES);    

    pub(in super) const CSS_MATCH_13326242258686333006_PROPERTIES: &[NodeDataInlineCssProperty] = &[
        // .projekt-name p
        NodeDataInlineCssProperty::Normal(CssProperty::FontSize(StyleFontSizeValue::Exact(StyleFontSize { inner: PixelValue::const_px(12) })))
    ];
    pub(in super) const CSS_MATCH_13326242258686333006: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_13326242258686333006_PROPERTIES);    

    pub(in super) const CSS_MATCH_13650487208571438689_PROPERTIES: &[NodeDataInlineCssProperty] = &[
        // body
        NodeDataInlineCssProperty::Normal(CssProperty::FlexDirection(LayoutFlexDirectionValue::Exact(LayoutFlexDirection::Row))),
        // *
        NodeDataInlineCssProperty::Normal(CssProperty::FontFamily(StyleFontFamilyVecValue::Exact(StyleFontFamilyVec::from_const_slice(STYLE_FONT_FAMILY_10582572158086033001_ITEMS))))
    ];
    pub(in super) const CSS_MATCH_13650487208571438689: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_13650487208571438689_PROPERTIES);    

    pub(in super) const CSS_MATCH_1550058906051548789_PROPERTIES: &[NodeDataInlineCssProperty] = &[
        // .status-log-ok
        NodeDataInlineCssProperty::Normal(CssProperty::FlexDirection(LayoutFlexDirectionValue::Exact(LayoutFlexDirection::Row))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingRight(LayoutPaddingRightValue::Exact(LayoutPaddingRight { inner: PixelValue::const_px(10) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingLeft(LayoutPaddingLeftValue::Exact(LayoutPaddingLeft { inner: PixelValue::const_px(10) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingBottom(LayoutPaddingBottomValue::Exact(LayoutPaddingBottom { inner: PixelValue::const_px(10) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingTop(LayoutPaddingTopValue::Exact(LayoutPaddingTop { inner: PixelValue::const_px(10) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::TextColor(StyleTextColorValue::Exact(StyleTextColor { inner: ColorU { r: 255, g: 255, b: 255, a: 255 } }))),
        NodeDataInlineCssProperty::Normal(CssProperty::BackgroundContent(StyleBackgroundContentVecValue::Exact(StyleBackgroundContentVec::from_const_slice(STYLE_BACKGROUND_CONTENT_18282593039276133616_ITEMS))))
    ];
    pub(in super) const CSS_MATCH_1550058906051548789: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_1550058906051548789_PROPERTIES);    

    pub(in super) const CSS_MATCH_1569823123531431981_PROPERTIES: &[NodeDataInlineCssProperty] = &[
        // .projekt-ueberschrift p
        NodeDataInlineCssProperty::Normal(CssProperty::FontSize(StyleFontSizeValue::Exact(StyleFontSize { inner: PixelValue::const_px(16) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::FontFamily(StyleFontFamilyVecValue::Exact(StyleFontFamilyVec::from_const_slice(STYLE_FONT_FAMILY_4570983119824751714_ITEMS))))
    ];
    pub(in super) const CSS_MATCH_1569823123531431981: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_1569823123531431981_PROPERTIES);    

    pub(in super) const CSS_MATCH_16439713609851736080_PROPERTIES: &[NodeDataInlineCssProperty] = &[
        // .status-log
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingRight(LayoutPaddingRightValue::Exact(LayoutPaddingRight { inner: PixelValue::const_px(10) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingLeft(LayoutPaddingLeftValue::Exact(LayoutPaddingLeft { inner: PixelValue::const_px(10) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingBottom(LayoutPaddingBottomValue::Exact(LayoutPaddingBottom { inner: PixelValue::const_px(10) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingTop(LayoutPaddingTopValue::Exact(LayoutPaddingTop { inner: PixelValue::const_px(10) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::TextColor(StyleTextColorValue::Exact(StyleTextColor { inner: ColorU { r: 255, g: 255, b: 255, a: 255 } }))),
        NodeDataInlineCssProperty::Normal(CssProperty::BackgroundContent(StyleBackgroundContentVecValue::Exact(StyleBackgroundContentVec::from_const_slice(STYLE_BACKGROUND_CONTENT_14631383723043813572_ITEMS))))
    ];
    pub(in super) const CSS_MATCH_16439713609851736080: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_16439713609851736080_PROPERTIES);    

    pub(in super) const CSS_MATCH_17359577546762513908_PROPERTIES: &[NodeDataInlineCssProperty] = &[
        // .verfahrensuebersicht
        NodeDataInlineCssProperty::Normal(CssProperty::MaxWidth(LayoutMaxWidthValue::Exact(LayoutMaxWidth { inner: PixelValue::const_px(200) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::FlexGrow(LayoutFlexGrowValue::Exact(LayoutFlexGrow { inner: FloatValue::const_new(1) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::BackgroundContent(StyleBackgroundContentVecValue::Exact(StyleBackgroundContentVec::from_const_slice(STYLE_BACKGROUND_CONTENT_605821317762015437_ITEMS))))
    ];
    pub(in super) const CSS_MATCH_17359577546762513908: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_17359577546762513908_PROPERTIES);    

    pub(in super) const CSS_MATCH_18307594868656599026_PROPERTIES: &[NodeDataInlineCssProperty] = &[
        // .status-log-fehler .detail
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingRight(LayoutPaddingRightValue::Exact(LayoutPaddingRight { inner: PixelValue::const_px(5) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingLeft(LayoutPaddingLeftValue::Exact(LayoutPaddingLeft { inner: PixelValue::const_px(5) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingBottom(LayoutPaddingBottomValue::Exact(LayoutPaddingBottom { inner: PixelValue::const_px(5) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingTop(LayoutPaddingTopValue::Exact(LayoutPaddingTop { inner: PixelValue::const_px(5) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::TextColor(StyleTextColorValue::Exact(StyleTextColor { inner: ColorU { r: 0, g: 0, b: 0, a: 255 } }))),
        NodeDataInlineCssProperty::Normal(CssProperty::BoxShadowBottom(StyleBoxShadowValue::Exact(StyleBoxShadow {
                offset: [PixelValueNoPercent { inner: PixelValue::const_px(0) }, PixelValueNoPercent { inner: PixelValue::const_px(0) }],
                color: ColorU { r: 0, g: 0, b: 0, a: 17 },
                blur_radius: PixelValueNoPercent { inner: PixelValue::const_px(10) },
                spread_radius: PixelValueNoPercent { inner: PixelValue::const_px(0) },
                clip_mode: BoxShadowClipMode::Outset,
            }))),
        NodeDataInlineCssProperty::Normal(CssProperty::BoxShadowTop(StyleBoxShadowValue::Exact(StyleBoxShadow {
                offset: [PixelValueNoPercent { inner: PixelValue::const_px(0) }, PixelValueNoPercent { inner: PixelValue::const_px(0) }],
                color: ColorU { r: 0, g: 0, b: 0, a: 17 },
                blur_radius: PixelValueNoPercent { inner: PixelValue::const_px(10) },
                spread_radius: PixelValueNoPercent { inner: PixelValue::const_px(0) },
                clip_mode: BoxShadowClipMode::Outset,
            }))),
        NodeDataInlineCssProperty::Normal(CssProperty::BoxShadowRight(StyleBoxShadowValue::Exact(StyleBoxShadow {
                offset: [PixelValueNoPercent { inner: PixelValue::const_px(0) }, PixelValueNoPercent { inner: PixelValue::const_px(0) }],
                color: ColorU { r: 0, g: 0, b: 0, a: 17 },
                blur_radius: PixelValueNoPercent { inner: PixelValue::const_px(10) },
                spread_radius: PixelValueNoPercent { inner: PixelValue::const_px(0) },
                clip_mode: BoxShadowClipMode::Outset,
            }))),
        NodeDataInlineCssProperty::Normal(CssProperty::BoxShadowLeft(StyleBoxShadowValue::Exact(StyleBoxShadow {
                offset: [PixelValueNoPercent { inner: PixelValue::const_px(0) }, PixelValueNoPercent { inner: PixelValue::const_px(0) }],
                color: ColorU { r: 0, g: 0, b: 0, a: 17 },
                blur_radius: PixelValueNoPercent { inner: PixelValue::const_px(10) },
                spread_radius: PixelValueNoPercent { inner: PixelValue::const_px(0) },
                clip_mode: BoxShadowClipMode::Outset,
            }))),
        NodeDataInlineCssProperty::Normal(CssProperty::BorderBottomRightRadius(StyleBorderBottomRightRadiusValue::Exact(StyleBorderBottomRightRadius { inner: PixelValue::const_px(4) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::BorderBottomLeftRadius(StyleBorderBottomLeftRadiusValue::Exact(StyleBorderBottomLeftRadius { inner: PixelValue::const_px(4) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::BorderTopRightRadius(StyleBorderTopRightRadiusValue::Exact(StyleBorderTopRightRadius { inner: PixelValue::const_px(4) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::BorderTopLeftRadius(StyleBorderTopLeftRadiusValue::Exact(StyleBorderTopLeftRadius { inner: PixelValue::const_px(4) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::BackgroundContent(StyleBackgroundContentVecValue::Exact(StyleBackgroundContentVec::from_const_slice(STYLE_BACKGROUND_CONTENT_16418392607769947338_ITEMS))))
    ];
    pub(in super) const CSS_MATCH_18307594868656599026: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_18307594868656599026_PROPERTIES);    

    pub(in super) const CSS_MATCH_2027421917835617538_PROPERTIES: &[NodeDataInlineCssProperty] = &[
        // .projekt-name
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingRight(LayoutPaddingRightValue::Exact(LayoutPaddingRight { inner: PixelValue::const_px(10) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingLeft(LayoutPaddingLeftValue::Exact(LayoutPaddingLeft { inner: PixelValue::const_px(10) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingBottom(LayoutPaddingBottomValue::Exact(LayoutPaddingBottom { inner: PixelValue::const_px(3) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingTop(LayoutPaddingTopValue::Exact(LayoutPaddingTop { inner: PixelValue::const_px(3) })))
    ];
    pub(in super) const CSS_MATCH_2027421917835617538: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_2027421917835617538_PROPERTIES);    

    pub(in super) const CSS_MATCH_2217151026558672233_PROPERTIES: &[NodeDataInlineCssProperty] = &[
        // .__azul-native-button:active
        NodeDataInlineCssProperty::Active(CssProperty::BackgroundContent(StyleBackgroundContentVecValue::Exact(StyleBackgroundContentVec::from_const_slice(STYLE_BACKGROUND_CONTENT_13046214508323908618_ITEMS)))),
        // .__azul-native-button:hover
        NodeDataInlineCssProperty::Hover(CssProperty::BackgroundContent(StyleBackgroundContentVecValue::Exact(StyleBackgroundContentVec::from_const_slice(STYLE_BACKGROUND_CONTENT_15133467471352813994_ITEMS)))),
        // .__azul-native-button
        NodeDataInlineCssProperty::Normal(CssProperty::TextAlign(StyleTextAlignValue::Exact(StyleTextAlign::Center))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingRight(LayoutPaddingRightValue::Exact(LayoutPaddingRight { inner: PixelValue::const_px(5) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingLeft(LayoutPaddingLeftValue::Exact(LayoutPaddingLeft { inner: PixelValue::const_px(5) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingBottom(LayoutPaddingBottomValue::Exact(LayoutPaddingBottom { inner: PixelValue::const_px(5) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingTop(LayoutPaddingTopValue::Exact(LayoutPaddingTop { inner: PixelValue::const_px(5) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::JustifyContent(LayoutJustifyContentValue::Exact(LayoutJustifyContent::Center))),
        NodeDataInlineCssProperty::Normal(CssProperty::FontSize(StyleFontSizeValue::Exact(StyleFontSize { inner: PixelValue::const_px(13) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::FontFamily(StyleFontFamilyVecValue::Exact(StyleFontFamilyVec::from_const_slice(STYLE_FONT_FAMILY_10134585962653230168_ITEMS)))),
        NodeDataInlineCssProperty::Normal(CssProperty::FlexGrow(LayoutFlexGrowValue::Exact(LayoutFlexGrow { inner: FloatValue::const_new(0) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::FlexDirection(LayoutFlexDirectionValue::Exact(LayoutFlexDirection::Column))),
        NodeDataInlineCssProperty::Normal(CssProperty::Display(LayoutDisplayValue::Exact(LayoutDisplay::Block))),
        NodeDataInlineCssProperty::Normal(CssProperty::TextColor(StyleTextColorValue::Exact(StyleTextColor { inner: ColorU { r: 76, g: 76, b: 76, a: 255 } }))),
        NodeDataInlineCssProperty::Normal(CssProperty::BoxShadowBottom(StyleBoxShadowValue::Exact(StyleBoxShadow {
                offset: [PixelValueNoPercent { inner: PixelValue::const_px(0) }, PixelValueNoPercent { inner: PixelValue::const_px(0) }],
                color: ColorU { r: 197, g: 197, b: 197, a: 173 },
                blur_radius: PixelValueNoPercent { inner: PixelValue::const_px(3) },
                spread_radius: PixelValueNoPercent { inner: PixelValue::const_px(0) },
                clip_mode: BoxShadowClipMode::Outset,
            }))),
        NodeDataInlineCssProperty::Normal(CssProperty::BoxShadowTop(StyleBoxShadowValue::Exact(StyleBoxShadow {
                offset: [PixelValueNoPercent { inner: PixelValue::const_px(0) }, PixelValueNoPercent { inner: PixelValue::const_px(0) }],
                color: ColorU { r: 197, g: 197, b: 197, a: 173 },
                blur_radius: PixelValueNoPercent { inner: PixelValue::const_px(3) },
                spread_radius: PixelValueNoPercent { inner: PixelValue::const_px(0) },
                clip_mode: BoxShadowClipMode::Outset,
            }))),
        NodeDataInlineCssProperty::Normal(CssProperty::BoxShadowRight(StyleBoxShadowValue::Exact(StyleBoxShadow {
                offset: [PixelValueNoPercent { inner: PixelValue::const_px(0) }, PixelValueNoPercent { inner: PixelValue::const_px(0) }],
                color: ColorU { r: 197, g: 197, b: 197, a: 173 },
                blur_radius: PixelValueNoPercent { inner: PixelValue::const_px(3) },
                spread_radius: PixelValueNoPercent { inner: PixelValue::const_px(0) },
                clip_mode: BoxShadowClipMode::Outset,
            }))),
        NodeDataInlineCssProperty::Normal(CssProperty::BoxShadowLeft(StyleBoxShadowValue::Exact(StyleBoxShadow {
                offset: [PixelValueNoPercent { inner: PixelValue::const_px(0) }, PixelValueNoPercent { inner: PixelValue::const_px(0) }],
                color: ColorU { r: 197, g: 197, b: 197, a: 173 },
                blur_radius: PixelValueNoPercent { inner: PixelValue::const_px(3) },
                spread_radius: PixelValueNoPercent { inner: PixelValue::const_px(0) },
                clip_mode: BoxShadowClipMode::Outset,
            }))),
        NodeDataInlineCssProperty::Normal(CssProperty::BorderBottomWidth(LayoutBorderBottomWidthValue::Exact(LayoutBorderBottomWidth { inner: PixelValue::const_px(1) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::BorderLeftWidth(LayoutBorderLeftWidthValue::Exact(LayoutBorderLeftWidth { inner: PixelValue::const_px(1) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::BorderRightWidth(LayoutBorderRightWidthValue::Exact(LayoutBorderRightWidth { inner: PixelValue::const_px(1) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::BorderTopWidth(LayoutBorderTopWidthValue::Exact(LayoutBorderTopWidth { inner: PixelValue::const_px(1) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::BorderBottomStyle(StyleBorderBottomStyleValue::Exact(StyleBorderBottomStyle { inner: BorderStyle::Solid }))),
        NodeDataInlineCssProperty::Normal(CssProperty::BorderLeftStyle(StyleBorderLeftStyleValue::Exact(StyleBorderLeftStyle { inner: BorderStyle::Solid }))),
        NodeDataInlineCssProperty::Normal(CssProperty::BorderRightStyle(StyleBorderRightStyleValue::Exact(StyleBorderRightStyle { inner: BorderStyle::Solid }))),
        NodeDataInlineCssProperty::Normal(CssProperty::BorderTopStyle(StyleBorderTopStyleValue::Exact(StyleBorderTopStyle { inner: BorderStyle::Solid }))),
        NodeDataInlineCssProperty::Normal(CssProperty::BorderBottomColor(StyleBorderBottomColorValue::Exact(StyleBorderBottomColor { inner: ColorU { r: 172, g: 172, b: 172, a: 255 } }))),
        NodeDataInlineCssProperty::Normal(CssProperty::BorderLeftColor(StyleBorderLeftColorValue::Exact(StyleBorderLeftColor { inner: ColorU { r: 172, g: 172, b: 172, a: 255 } }))),
        NodeDataInlineCssProperty::Normal(CssProperty::BorderRightColor(StyleBorderRightColorValue::Exact(StyleBorderRightColor { inner: ColorU { r: 172, g: 172, b: 172, a: 255 } }))),
        NodeDataInlineCssProperty::Normal(CssProperty::BorderTopColor(StyleBorderTopColorValue::Exact(StyleBorderTopColor { inner: ColorU { r: 172, g: 172, b: 172, a: 255 } }))),
        NodeDataInlineCssProperty::Normal(CssProperty::BackgroundContent(StyleBackgroundContentVecValue::Exact(StyleBackgroundContentVec::from_const_slice(STYLE_BACKGROUND_CONTENT_12667596745911411196_ITEMS))))
    ];
    pub(in super) const CSS_MATCH_2217151026558672233: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_2217151026558672233_PROPERTIES);    

    pub(in super) const CSS_MATCH_2875502314340155187_PROPERTIES: &[NodeDataInlineCssProperty] = &[
        // .ausgewaehltes-verfahren
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingRight(LayoutPaddingRightValue::Exact(LayoutPaddingRight { inner: PixelValue::const_px(25) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingLeft(LayoutPaddingLeftValue::Exact(LayoutPaddingLeft { inner: PixelValue::const_px(25) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingBottom(LayoutPaddingBottomValue::Exact(LayoutPaddingBottom { inner: PixelValue::const_px(25) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingTop(LayoutPaddingTopValue::Exact(LayoutPaddingTop { inner: PixelValue::const_px(25) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::FlexGrow(LayoutFlexGrowValue::Exact(LayoutFlexGrow { inner: FloatValue::const_new(1) })))
    ];
    pub(in super) const CSS_MATCH_2875502314340155187: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_2875502314340155187_PROPERTIES);    

    pub(in super) const CSS_MATCH_IMAGE_PROPERTIES: &[NodeDataInlineCssProperty] = &[
        // .ausgewaehltes-verfahren
        NodeDataInlineCssProperty::Normal(CssProperty::Width(LayoutWidthValue::Exact(LayoutWidth { inner: PixelValue::const_px(16) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::MinWidth(LayoutMinWidthValue::Exact(LayoutMinWidth { inner: PixelValue::const_px(16) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::MaxWidth(LayoutMaxWidthValue::Exact(LayoutMaxWidth { inner: PixelValue::const_px(16) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::Height(LayoutHeightValue::Exact(LayoutHeight { inner: PixelValue::const_px(16) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::MinHeight(LayoutMinHeightValue::Exact(LayoutMinHeight { inner: PixelValue::const_px(16) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::MaxHeight(LayoutMaxHeightValue::Exact(LayoutMaxHeight { inner: PixelValue::const_px(16) }))),
    ];
    pub(in super) const CSS_MATCH_IMAGE: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_IMAGE_PROPERTIES);    

    pub(in super) const CSS_MATCH_6447022794024462679_PROPERTIES: &[NodeDataInlineCssProperty] = &[
        // .test-buttons>*
        NodeDataInlineCssProperty::Normal(CssProperty::MarginRight(LayoutMarginRightValue::Exact(LayoutMarginRight { inner: PixelValue::const_px(10) }))),
        // .test-buttons
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingRight(LayoutPaddingRightValue::Exact(LayoutPaddingRight { inner: PixelValue::const_px(0) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingLeft(LayoutPaddingLeftValue::Exact(LayoutPaddingLeft { inner: PixelValue::const_px(0) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingBottom(LayoutPaddingBottomValue::Exact(LayoutPaddingBottom { inner: PixelValue::const_px(10) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::PaddingTop(LayoutPaddingTopValue::Exact(LayoutPaddingTop { inner: PixelValue::const_px(10) }))),
        NodeDataInlineCssProperty::Normal(CssProperty::FlexDirection(LayoutFlexDirectionValue::Exact(LayoutFlexDirection::Row)))
    ];
    pub(in super) const CSS_MATCH_6447022794024462679: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_6447022794024462679_PROPERTIES);    

    pub(in super) const CSS_MATCH_7059827997353143778_PROPERTIES: &[NodeDataInlineCssProperty] = &[
        // .verfahrens-info-zeile p.text-rechts
        NodeDataInlineCssProperty::Normal(CssProperty::TextAlign(StyleTextAlignValue::Exact(StyleTextAlign::Right))),
        // .verfahrens-info-zeile p
        NodeDataInlineCssProperty::Normal(CssProperty::FlexGrow(LayoutFlexGrowValue::Exact(LayoutFlexGrow { inner: FloatValue::const_new(1) })))
    ];
    pub(in super) const CSS_MATCH_7059827997353143778: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_7059827997353143778_PROPERTIES);
}


fn render_verfahrensuebersicht(
    app_data: RefAny, 
    verfahren_filter: &Option<String>, 
    verfahren: &[VerfahrenGeladen], 
    ausgewaehltes_verfahren: &Option<String>
) -> Dom {
    use azul::widgets::{TextInput, Button};

    div::render()
    .with_inline_css_props(CSS_MATCH_17359577546762513908)
    .with_ids_and_classes({
        pub(in super) const IDS_AND_CLASSES_17892264718074089608: &[IdOrClass] = &[
                Class(AzString::from_const_str("verfahrensuebersicht")),

        ];
        IdOrClassVec::from_const_slice(IDS_AND_CLASSES_17892264718074089608)
    })
    .with_children(vec![

            div::render()
            .with_inline_css_props(CSS_MATCH_13021692908862555904)
            .with_ids_and_classes({
                pub(in super) const IDS_AND_CLASSES_11102932419086932125: &[IdOrClass] = &[
                    Class(AzString::from_const_str("projekt-ueberschrift")),
                ];
                IdOrClassVec::from_const_slice(IDS_AND_CLASSES_11102932419086932125)
            })
            .with_children(DomVec::from_vec(vec![
                p::render(AzString::from_const_str("PROJEKTE"))
                .with_inline_css_props(CSS_MATCH_1569823123531431981)
            ])),

            div::render()
            .with_inline_style("padding:5px;flex-direction:row;".into())
            .with_children(vec![
                {
                    let mut ti = TextInput::new()
                    .with_text(verfahren_filter.clone().unwrap_or_default().into())
                    .with_on_focus_lost(app_data.clone(), filter_verfahren);

                    if verfahren_filter.is_none() {
                        ti.set_placeholder("Projekte durchsuchen...".into());
                    }

                    ti.dom()
                },
                Dom::div()
                .with_inline_style("padding-left:5px;".into())
                .with_child(
                    Button::new("Suche".into())
                    .dom()
                )
            ].into()),
            
            verfahren
            .iter()
            .filter(|vg| {
                let search_text = format!("{:06} - {}", vg.nummer, vg.name);
                match verfahren_filter.as_ref() {
                    None => true,
                    Some(s) => search_text.contains(s) || {
                        vg.buchungsblatt_bodenordnung.iter().any(|bb| {
                            match bb.ax_buchungsblatt.bbb_name.as_ref() {
                                Some(b) => b.contains(s),
                                None => false,
                            }
                        })
                    },
                }
            })
            .map(|vg| {

                let ist_ausgewaehlt = *ausgewaehltes_verfahren == Some(vg.uuid.clone());

                let mut d = div::render()
                .with_children(DomVec::from_vec(vec![
                    p::render(format!("{:06} - {}", vg.nummer, vg.name).into())
                    .with_inline_css_props(CSS_MATCH_13326242258686333006)
                ]))
                .with_inline_css_props(CSS_MATCH_2027421917835617538)
                .with_inline_style(if ist_ausgewaehlt {
                    AzString::from_const_str("background-color: white")
                } else {
                    AzString::from_const_str("")
                })
                .with_callbacks(vec![CallbackData {
                    event: EventFilter::Hover(HoverEventFilter::MouseUp),
                    callback: Callback { cb: verfahren_auswaehlen },
                    data: RefAny::new(VerfahrenAuswaehlenDataset {
                        backref: app_data.clone(),
                        verfahren_uuid: vg.uuid.clone(),
                    }),
                }].into());

                if !ist_ausgewaehlt {
                    d.with_inline_hover_style(AzString::from_const_str("background-color: #c1c1c1"))
                } else {
                    d
                }

        }).collect::<Dom>()

    ].into())
}

struct VerfahrenAuswaehlenDataset {
    backref: RefAny,
    verfahren_uuid: String,
}

extern "C"
fn filter_verfahren(data: &mut RefAny, info: &mut CallbackInfo, ts: &TextInputState) -> Update {

    let mut data = match data.downcast_mut::<AppData>() {
        Some(s) => s,
        None => return Update::DoNothing,
    };

    let text = ts.get_text();
    data.verfahren_filter = if text.as_str().trim().is_empty() { None } else { Some(text.as_str().to_string()) };
    Update::RefreshDom
}

extern "C"
fn verfahren_auswaehlen(data: &mut RefAny, info: &mut CallbackInfo) -> Update {

    let mut data = match data.downcast_mut::<VerfahrenAuswaehlenDataset>() {
        Some(s) => s,
        None => return Update::DoNothing,
    };

    let verfahren_uuid = data.verfahren_uuid.clone();

    let mut data = match data.backref.downcast_mut::<AppData>() {
        Some(s) => s,
        None => return Update::DoNothing,
    };

    data.ausgewaehltes_verfahren = Some(verfahren_uuid);

    Update::RefreshDom
}

fn render_verfahren_info(app_data: RefAny, data: &AppData) -> Dom {

    use crate::Auftragsstatus;
    use azul::widgets::*;
    use azul::image::{ImageRef, RawImage};
    use azul::option::OptionImageRef;
    use azul::error::ResultRawImageDecodeImageError;

    let aw = match data.ausgewaehltes_verfahren.clone() {
        Some(s) => s,
        None => { return Dom::div(); },
    };

    let ausgewaehltes_verfahren = match data.geladene_verfahren.verfahren.iter().find(|v| v.uuid == aw) {
        Some(s) => s,
        None => { return Dom::div(); },
    };

    let mut insert = 0;
    let mut replace = 0;
    let mut delete = 0;

    let img_done = match RawImage::decode_image_bytes_any(IMG_DONE_VEC.as_ref_vec()) {
        ResultRawImageDecodeImageError::Ok(s) => s,
        ResultRawImageDecodeImageError::Err(_) => return Dom::div(),
    };

    let img_close = match RawImage::decode_image_bytes_any(IMG_CLOSE_VEC.as_ref_vec()) {
        ResultRawImageDecodeImageError::Ok(s) => s,
        ResultRawImageDecodeImageError::Err(_) => return Dom::div(),
    };

    let img_done = match ImageRef::raw_image(img_done) {
        OptionImageRef::Some(i) => i,
        OptionImageRef::None => return Dom::div(),
    };

    let img_close = match ImageRef::raw_image(img_close) {
        OptionImageRef::Some(i) => i,
        OptionImageRef::None => return Dom::div(),
    };

    div::render()
    .with_inline_css_props(CSS_MATCH_2875502314340155187)
    .with_ids_and_classes({
        pub(in super) const IDS_AND_CLASSES_12190625166707343638: &[IdOrClass] = &[
                Class(AzString::from_const_str("ausgewaehltes-verfahren")),
        ];
        IdOrClassVec::from_const_slice(IDS_AND_CLASSES_12190625166707343638)
    })
    .with_children(DomVec::from_vec(vec![
        p::render(format!("{:06} - {} ({})", 
            ausgewaehltes_verfahren.nummer, 
            ausgewaehltes_verfahren.name, 
            ausgewaehltes_verfahren.dhk_verbindung
        ).into())
        .with_inline_css_props(CSS_MATCH_10644212216580784356)
        .with_ids_and_classes({
            pub(in super) const IDS_AND_CLASSES_7612915473292556625: &[IdOrClass] = &[
                        Class(AzString::from_const_str("ausgewaehltes-verfahren-titel")),

            ];
            IdOrClassVec::from_const_slice(IDS_AND_CLASSES_7612915473292556625)
        }),
        div::render()
        .with_inline_css_props(CSS_MATCH_6447022794024462679)
        .with_ids_and_classes({
            pub(in super) const IDS_AND_CLASSES_3777573455697972450: &[IdOrClass] = &[
                        Class(AzString::from_const_str("test-buttons")),

            ];
            IdOrClassVec::from_const_slice(IDS_AND_CLASSES_3777573455697972450)
        })
        .with_children(DomVec::from_vec(vec![
            div::render().with_children(vec![
                p::render(AzString::from_const_str(".lefis Datei fortführen")),       
            ].into())
            .with_inline_style("display: flex; flex-grow: 1;".into()),
            div::render().with_children(vec![
                FileInput::new(None.into())
                .with_default_text("Datei öffnen".into())
                .with_on_path_change(app_data.clone(), lefis_datei_laden)
                .dom()
            ].into())
            .with_inline_style("display: flex; flex-grow: 0;".into()),
        ])),

        div::render()
        .with_children(vec![
            TabHeader::new(vec![
                format!("Grundbuchblätter ({})", ausgewaehltes_verfahren.buchungsblatt_bodenordnung
                        .iter()
                        .filter(|i| !i.nebenbeteiligten_blatt)
                        .count()
                    ),
                format!("Flurstücke ({})", ausgewaehltes_verfahren.flurstuecke.len()),
                format!("Abt. 2 Rechte ({})", ausgewaehltes_verfahren.abt2_rechte.len()),
                format!("Abt. 3 Rechte ({})", ausgewaehltes_verfahren.abt3_rechte.len()),
            ].into())
            .with_active_tab(ausgewaehltes_verfahren.ui.tab)
            .with_on_click(app_data.clone(), switch_active_tab)
            .dom()
        ].into()),

        match ausgewaehltes_verfahren.ui.tab {
            0 => {
                div::render()
                .with_children(vec![
                    TabHeader::new(vec![
                        format!("Alle ({})", ausgewaehltes_verfahren.buchungsblatt_bodenordnung
                            .iter()
                            .filter(|i| !i.nebenbeteiligten_blatt)
                            .count()
                        ),
                        format!("Abgeglichen ({})", ausgewaehltes_verfahren.buchungsblatt_bodenordnung
                            .iter()
                            .filter(|i| !i.nebenbeteiligten_blatt)
                            .filter(|i| i.grundbuchvergleich_durchgefuehrt)
                            .count()
                        ),
                        format!("Nicht abgeglichen ({})", ausgewaehltes_verfahren.buchungsblatt_bodenordnung
                            .iter()
                            .filter(|i| !i.nebenbeteiligten_blatt)
                            .filter(|i| !i.grundbuchvergleich_durchgefuehrt)
                            .count()
                        ),
                    ].into())
                    .with_active_tab(ausgewaehltes_verfahren.ui.sub_tab)
                    .with_on_click(app_data.clone(), switch_active_sub_tab)
                    .dom()
                ].into())
            }
            _ => div::render()
        },

        match ausgewaehltes_verfahren.ui.tab {
            0 => ListView::new(vec![
                format!("Abgeglichen"),
                format!("Blatt"),
                format!("Nr."),
            ].into())
            .with_rows(ausgewaehltes_verfahren.buchungsblatt_bodenordnung
                .iter()
                .filter(|gb| !gb.nebenbeteiligten_blatt)
                .filter(|gb| {
                    match ausgewaehltes_verfahren.ui.sub_tab {
                        1 => gb.grundbuchvergleich_durchgefuehrt,
                        2 => !gb.grundbuchvergleich_durchgefuehrt,
                        _ => true,
                    }
                })
                .take(20)
                .map(|gb| ListViewRow {
                    cells: vec![
                        if gb.grundbuchvergleich_durchgefuehrt {
                            Dom::image(img_done.clone())
                        } else {
                            Dom::image(img_close.clone())
                        }
                        .with_inline_css_props(CSS_MATCH_IMAGE),
                        p::render(gb.ax_buchungsblatt.bbb_name.clone().unwrap_or_default().into()),
                        p::render(format!("{}", gb.ax_buchungsblatt.bbn).into()),
                    ].into(),
                    height: None.into(),
                }).collect::<Vec<_>>().into())
            .dom(),
            1 => ListView::new(vec![
                format!("Gemarkung"),
                format!("Flur"),
                format!("Flurstück"),
                format!("belastet"),
            ].into())
            .with_rows(ausgewaehltes_verfahren.flurstuecke.iter().take(20).map(|flst| {
               ListViewRow {
                   cells: vec![
                       p::render(flst.uuid.clone().into()),
                       p::render(flst.kennzeichen.clone().into()),
                       p::render(flst.ax21008.clone().into()),
                       CheckBox::new(false).dom(),
                   ].into(),
                   height: None.into(),
               }     
            }).collect::<Vec<_>>().into()).dom(),
            2 => ListView::new(vec![
                format!("Blatt"),
                format!("lfd. Nr."),
                format!("Art"),
                format!("Text"),
                format!("Flurstücke"),
                format!("Personen"),
            ].into())
            .with_rows(ausgewaehltes_verfahren.abt2_rechte.iter().take(20).map(|abt2| {
                ListViewRow {
                    cells: vec![
                        p::render(abt2.uuid.clone().into()),
                        p::render(format!("{}", abt2.lfd_nr).into()),
                        p::render(format!("{}", abt2.lfd_nr).into()),
                        p::render(format!("{}", abt2.lfd_nr).into()),
                        p::render(format!("{}", abt2.lfd_nr).into()),
                        p::render(format!("{}", abt2.lfd_nr).into()),
                    ].into(),
                    height: None.into(),
                }
            }).collect::<Vec<_>>().into()).dom(),
            3 => ListView::new(vec![
                format!("Blatt"),
                format!("lfd. Nr."),
                format!("Art"),
                format!("Betrag"),
                format!("Text"),
                format!("Flurstücke"),
                format!("Personen"),
            ].into())
            .with_rows(ausgewaehltes_verfahren.abt3_rechte.iter().take(20).map(|abt3| {
                ListViewRow {
                    cells: vec![
                        p::render(abt3.uuid.clone().into()),
                        p::render(format!("{}", abt3.lfd_nr).into()),
                        p::render(format!("{}", abt3.lfd_nr).into()),
                        p::render(format!("{}", abt3.lfd_nr).into()),
                        p::render(format!("{}", abt3.lfd_nr).into()),
                        p::render(format!("{}", abt3.lfd_nr).into()),
                        p::render(format!("{}", abt3.lfd_nr).into()),
                    ].into(),
                    height: None.into(),
                }
            }).collect::<Vec<_>>().into()).dom(),
            _ => Dom::div(),
        },

        match &ausgewaehltes_verfahren.lefis_geladen {
            None => div::render(),
            Some(v) => {                
                let mut warnungen = Vec::new();
                let ffa = generiere_ffa(
                    &data.geladene_verfahren, 
                    &aw, 
                    v,
                    &mut warnungen
                );

                insert = ffa.insert.len();
                replace = ffa.replace.len();
                delete = ffa.delete.len();

                match warnungen.first() {
                    Some(s) => {
                        div::render()
                        .with_inline_style("padding: 5px;".into())
                        .with_child(
                            div::render()
                            .with_inline_style("flex-grow: 1;flex-direction:row;".into())
                            .with_children(if warnungen.len() > 1 {
                                    vec![
                                    p::render(format!("Warnung: {} - und {} weitere Fehler", s, warnungen.len() - 1).into())
                                    .with_inline_style("flex-grow:1;font-size: 12px; color: #999900; background: #FFFFE0;".into()),
                                    div::render()
                                    .with_inline_style("display:flex;flex-grow:0;align-items:flex-end;justify-content:flex-end;".into())
                                    .with_child(
                                        Button::new("Anzeigen".into())
                                        .with_on_click(RefAny::new(WarnungDialog {
                                            warnungen: warnungen.clone(),
                                        }), zeige_warnungen)
                                        .dom()
                                    )
                                ]
                            } else {
                                vec![
                                    p::render(format!("Warnung: {}", s).into())
                                    .with_inline_style("font-size: 12px; color: #999900; background: #FFFFE0;".into())
                                ]
                            }.into())
                            .with_inline_style("padding: 2.5px;".into())
                        )
                    },
                    None => {
                        div::render()
                    },
                }
            }
        },

        div::render()
        .with_children(vec![
            match &ausgewaehltes_verfahren.lefis_geladen {
                None => match &ausgewaehltes_verfahren.auftragsstatus {
                    None => div::render(),
                    Some(Auftragsstatus::AuftragWirdFortgefuehrt { prozent }) =>  {
                        div::render()
                        .with_inline_style("padding-top:5px;".into())
                        .with_child(
                            div::render()
                            .with_inline_css_props(CSS_MATCH_16439713609851736080)
                            .with_ids_and_classes({
                                pub(in super) const IDS_AND_CLASSES_16141427507998024750: &[IdOrClass] = &[
                                            Class(AzString::from_const_str("status-log")),

                                ];
                                IdOrClassVec::from_const_slice(IDS_AND_CLASSES_16141427507998024750)
                            })
                            .with_children(DomVec::from_vec(vec![
                                p::render(format!("Auftrag wird fortgeführt... ({}%)", prozent).into())
                            ]))
                        )
                    },
                    Some(Auftragsstatus::Fehler { log }) => {
                        div::render()
                        .with_inline_style("padding-top:5px;".into())
                        .with_child(
                            div::render()
                            .with_inline_css_props(CSS_MATCH_12913589432591768438)
                            .with_ids_and_classes({
                                pub(in super) const IDS_AND_CLASSES_11219677383442904130: &[IdOrClass] = &[
                                            Class(AzString::from_const_str("status-log-fehler")),

                                ];
                                IdOrClassVec::from_const_slice(IDS_AND_CLASSES_11219677383442904130)
                            })
                            .with_children(DomVec::from_vec(vec![
                                div::render()
                                .with_inline_css_props(CSS_MATCH_11661823484470247691)
                                .with_children(DomVec::from_vec(vec![
                                    p::render(AzString::from_const_str("Fehler: Auftrag konnte nicht fortgeführt werden")),
                                    div::render().with_inline_style("flex-grow:1;".into()),
                                    FileInput::new(None.into())
                                    .with_default_text("Protokoll speichern unter".into())
                                    .with_on_path_change(RefAny::new(ProtokollDataset {
                                        log: log.clone(),
                                    }), protokoll_speichern_unter)
                                    .dom()
                                ])),
                                div::render()
                                .with_inline_css_props(CSS_MATCH_18307594868656599026)
                                .with_child(
                                    div::render()
                                    .with_inline_style("width:700px;font-size:12px;".into())
                                    .with_child({
                                        let text = log.iter().filter_map(|msg| match msg {
                                            ProtokollMsg::Error(f) => Some(format!("Fehler: {}", f)),
                                            _ => None,
                                        }).collect::<Vec<_>>().join("\r\n\r\n");
                                        
                                        p::render(text.into())
                                        .with_inline_style("width:700px;font-size:12px;".into())
                                    })
                                )
                            ]))
                        )
                    },
                    Some(Auftragsstatus::ErfolgreichFortgefuehrt) => {

                        div::render()
                        .with_inline_style("padding-top:5px;".into())
                        .with_child(
                            div::render()
                            .with_inline_css_props(CSS_MATCH_1550058906051548789)
                            .with_ids_and_classes({
                                pub(in super) const IDS_AND_CLASSES_5815670520971901181: &[IdOrClass] = &[
                                            Class(AzString::from_const_str("status-log-ok")),

                                ];
                                IdOrClassVec::from_const_slice(IDS_AND_CLASSES_5815670520971901181)
                            })
                            .with_children(DomVec::from_vec(vec![
                                p::render(AzString::from_const_str("Ok: Auftrag wurde fortgeführt"))
                            ]))
                        )
                    }
                },
                Some(v) => {
                    div::render()
                    .with_inline_style("padding-top:5px;".into())
                    .with_child(
                        div::render()
                        .with_inline_css_props(CSS_MATCH_1550058906051548789)
                        .with_children(DomVec::from_vec(vec![
                            div::render().with_children(vec![
                                p::render(
                                    format!("Ok: {} Grundbuchblätter bereit für Fortführung in DHK ({} Insert, {} Replace, {} Delete)", 
                                    v.len(), insert, replace, delete)
                                .into()),
                            ].into())
                            .with_inline_style("display: flex; flex-grow: 1;".into()),
                            div::render().with_children(vec![
                                Button::new("Fortführen".into())
                                .with_on_click(app_data.clone(), lefis_datei_fortfuehren)
                                .dom(),
                            ].into())
                            .with_inline_style("display: flex; flex-grow: 0;".into()),
                        ]))
                    )
                }
            },
        ].into())
    ]))
}

struct ProtokollDataset {
    log: Vec<ProtokollMsg>,
}

extern "C"
fn protokoll_speichern_unter(data: &mut RefAny, _info: &mut CallbackInfo, fi: &FileInputState) -> Update {
    
    let data = match data.downcast_ref::<ProtokollDataset>() {
        Some(s) => s,
        None => return Update::DoNothing,
    };
    let data = &*data;

    let datei_pfad = match fi.path.as_ref() {
        Some(s) => s.as_str().to_string(),
        None => return Update::DoNothing,
    };

    let s = data.log.iter()
        .map(|v| match v {
            ProtokollMsg::Info(i) => format!("INFO: {}", i),
            ProtokollMsg::Error(e) => format!("ERROR: {}", e),
        })
        .collect::<Vec<_>>()
        .join("\r\n");

    let _ = std::fs::write(&datei_pfad, s.as_bytes());

    Update::RefreshDom
}

struct WarnungDialog {
    warnungen: Vec<String>,
}

extern "C"
fn zeige_warnungen(data: &mut RefAny, _info: &mut CallbackInfo) -> Update {
    
    use azul::dialog::MsgBox;

    let data = match data.downcast_ref::<WarnungDialog>() {
        Some(s) => s,
        None => { return Update::DoNothing; },
    };

    let _ = MsgBox::info(data.warnungen.clone().join("\r\n\r\n").into());

    Update::DoNothing
}

// switch_active_tab
extern "C"
fn switch_active_tab(data: &mut RefAny, _info: &mut CallbackInfo, th: &TabHeaderState) -> Update {

    use crate::{LefisDatei, Auftragsstatus};
    use azul::dialog::MsgBox;

    let mut data_mut = match data.downcast_mut::<AppData>() {
        Some(s) => s,
        None => return Update::DoNothing,
    };

    let data_mut = &mut *data_mut;

    let aktives_verfahren = match data_mut.ausgewaehltes_verfahren.clone().and_then(|d| data_mut.geladene_verfahren.verfahren.iter_mut().find(|v| v.uuid == d)) {
        Some(s) => s,
        None => return Update::DoNothing,
    };

    aktives_verfahren.ui.tab = th.active_tab;

    Update::RefreshDom
}

extern "C"
fn switch_active_sub_tab(data: &mut RefAny, _info: &mut CallbackInfo, th: &TabHeaderState) -> Update {

    use crate::{LefisDatei, Auftragsstatus};
    use azul::dialog::MsgBox;

    let mut data_mut = match data.downcast_mut::<AppData>() {
        Some(s) => s,
        None => return Update::DoNothing,
    };

    let data_mut = &mut *data_mut;

    let aktives_verfahren = match data_mut.ausgewaehltes_verfahren.clone().and_then(|d| data_mut.geladene_verfahren.verfahren.iter_mut().find(|v| v.uuid == d)) {
        Some(s) => s,
        None => return Update::DoNothing,
    };

    aktives_verfahren.ui.sub_tab = th.active_tab;

    Update::RefreshDom
}

extern "C"
fn lefis_datei_laden(data: &mut RefAny, _info: &mut CallbackInfo, fi: &FileInputState) -> Update {
    
    use crate::{LefisDatei, Auftragsstatus};
    use azul::dialog::MsgBox;

    let mut data_mut = match data.downcast_mut::<AppData>() {
        Some(s) => s,
        None => return Update::DoNothing,
    };

    let data_mut = &mut *data_mut;

    let aktives_verfahren = match data_mut.ausgewaehltes_verfahren.clone().and_then(|d| data_mut.geladene_verfahren.verfahren.iter_mut().find(|v| v.uuid == d)) {
        Some(s) => s,
        None => return Update::DoNothing,
    };

    if let Some(Auftragsstatus::AuftragWirdFortgefuehrt { .. }) = aktives_verfahren.auftragsstatus {
        MsgBox::error(format!("Es wird gerade ein Verfahren fortgeführt. Bitte warten Sie, bis der Vorgang abgeschlossen ist.").into());
        return Update::DoNothing;
    }

    let datei_pfad = match fi.path.as_ref() {
        Some(s) => s.as_str().to_string(),
        None => return Update::DoNothing,
    };

    let f_read = match std::fs::read_to_string(&datei_pfad) {
        Ok(s) => s,
        Err(e) => {
            MsgBox::error(format!("Fehler beim Laden: {} - {}", datei_pfad, e).into());
            return Update::DoNothing;
        }
    };

    let deserialized: Vec<LefisDatei> = match serde_json::from_str(&f_read) {
        Ok(s) => s,
        Err(e) => {
            MsgBox::error(format!("Fehler beim Laden: {} - {}", datei_pfad, e).into());
            return Update::DoNothing;
        }
    };

    aktives_verfahren.lefis_geladen = Some(deserialized);

    Update::DoNothing
}


extern "C"
fn verfahren_exportieren(data: &mut RefAny, _: &mut CallbackInfo) -> Update {

    use crate::{LefisDatei, Auftragsstatus};
    use azul::dialog::FileDialog;

    let mut data_mut = match data.downcast_mut::<AppData>() {
        Some(s) => s,
        None => return Update::DoNothing,
    };
    let data_mut = &mut *data_mut;

    let aktives_verfahren = match data_mut.ausgewaehltes_verfahren.clone().and_then(|d| data_mut.geladene_verfahren.verfahren.iter().find(|v| v.uuid == d)) {
        Some(s) => s,
        None => return Update::DoNothing,
    };

    if let Some(dateipfad) = FileDialog::save_file("Verfahren speichern unter".into(), None.into()).into_option() {
        let mut dateipfad = dateipfad.as_str().to_string();
        if !dateipfad.ends_with(".verfahren") {
            dateipfad = format!("{}.verfahren", dateipfad);
        }
        let _ = std::fs::write(&dateipfad, aktives_verfahren.to_json().as_bytes());
    }
    
    Update::RefreshDom
}

extern "C"
fn lefis_ffa_exportieren(data: &mut RefAny, _: &mut CallbackInfo) -> Update {

    use crate::{LefisDatei, Auftragsstatus};
    use azul::dialog::FileDialog;

    let mut data_mut = match data.downcast_mut::<AppData>() {
        Some(s) => s,
        None => return Update::DoNothing,
    };
    let data_mut = &mut *data_mut;

    let aw = match data_mut.ausgewaehltes_verfahren.clone() {
        Some(s) => s,
        None => return Update::DoNothing,
    };

    let aktives_verfahren = match data_mut.geladene_verfahren.verfahren.iter_mut().find(|v| v.uuid == aw) {
        Some(s) => s,
        None => return Update::DoNothing,
    };

    let lefis_geladen = match aktives_verfahren.lefis_geladen.clone() {
        Some(s) => s,
        None => return Update::DoNothing,
    };

    let mut warnungen = Vec::new();
    let ffa = generiere_ffa(
        &data_mut.geladene_verfahren,
        &aw,
        &lefis_geladen, 
        &mut warnungen
    );

    if let Some(dateipfad) = FileDialog::save_file("Fortführungsauftrag speichern unter".into(), None.into()).into_option() {
        let mut dateipfad = dateipfad.as_str().to_string();
        if !dateipfad.ends_with(".ffa.xml") {
            dateipfad = format!("{}.ffa.xml", dateipfad);
        }
        let ffa_xml = ffa.get_xml().trim().lines().collect::<Vec<_>>().join("\r\n");
        let _ = std::fs::write(&dateipfad, ffa_xml.as_bytes());
    }
    
    Update::RefreshDom
}

extern "C"
fn lefis_datei_fortfuehren(data: &mut RefAny, info: &mut CallbackInfo) -> Update {

    use crate::{LefisDatei, Auftragsstatus};
    use azul::dialog::MsgBox;

    let data_clone = data.clone();
    let mut data_mut = match data.downcast_mut::<AppData>() {
        Some(s) => s,
        None => return Update::DoNothing,
    };

    let konfiguration = data_mut.konfiguration.clone();
    let data_mut = &mut *data_mut;

    let geladene_verfahren = data_mut.geladene_verfahren.clone();
    let ausgewaehltes_verfahren = data_mut.ausgewaehltes_verfahren.clone();
    let aktives_verfahren = match data_mut.ausgewaehltes_verfahren.clone().and_then(|d| data_mut.geladene_verfahren.verfahren.iter_mut().find(|v| v.uuid == d)) {
        Some(s) => s,
        None => return Update::DoNothing,
    };

    if let Some(Auftragsstatus::AuftragWirdFortgefuehrt { .. }) = aktives_verfahren.auftragsstatus {
        MsgBox::error(format!("Es wird gerade ein Verfahren fortgeführt. Bitte warten Sie, bis der Vorgang abgeschlossen ist.").into());
        return Update::DoNothing;
    }

    if aktives_verfahren.lefis_geladen.is_none() {
        return Update::DoNothing;
    }

    let init_data = RefAny::new(BackgroundThreadInit {
        konfiguration,
        geladene_verfahren,
        ausgewaehltes_verfahren: ausgewaehltes_verfahren.unwrap(),
    });

    let thread_id = match info.start_thread(init_data, data_clone, ffa_background_thread).into_option() {
        Some(s) => s,
        None => return Update::DoNothing, // thread creation failed
    };

    aktives_verfahren.lefis_geladen = None;

    Update::DoNothing
}

#[derive(Debug)]
struct XmlBackgroundThreadInit {
    konfiguration: LefisUploadKonfiguration,
    verfahren_uuid: String,
    xml: String,
}

extern "C" fn xml_ffa_background_thread(
    mut initial_data: RefAny,
    mut sender: ThreadSender,
    _recv: ThreadReceiver,
) {
    use crate::wsdl::{
        AuftragsManager, 
        RequestFailure,
        ProtokollMsg,
    };
    use azul::task::*;
    use azul::callbacks::WriteBackCallback;

    let initial_data = match initial_data.downcast_ref::<XmlBackgroundThreadInit>() {
        Some(s) => s,
        None => return, // error
    };
    let initial_data = &*initial_data;

    let konfiguration = &initial_data.konfiguration.lefis;
    let verfahren_uuid = initial_data.verfahren_uuid.clone();

    let am = AuftragsManager::new(
        &konfiguration.get_webservice_url(), 
        konfiguration.benutzer.clone(), 
        konfiguration.passwort.clone()
    );
    
    sender.send(ThreadReceiveMsg::WriteBack(ThreadWriteBackMsg {
        data: RefAny::new(BackgroundThreadReturn::FortschrittUpdate { 
            verfahrens_uuid: verfahren_uuid.clone(), 
            prozent: 0,
        }),
        callback: WriteBackCallback { cb: writeback_callback }
    }));

    let mut protokoll_ext = None;
    let e: Result<(), RequestFailure> = exec_sync(async {
        let login = am.login().await?;
        let auftragsnummer = match am.register_gzip(login.session_id, &initial_data.xml).await {
            Ok(o) => Ok(o.auftragsnummer),
            Err(e) => {
                am.logout(login.session_id).await?;
                Err(e)
            }
        }?;

        let result = warte_auf_auftrag_fortfuehrung(&am, login.session_id, &auftragsnummer).await;

        if let Err(e) = &result {
            let protocol = am.get_protocol_gzip(login.session_id, &auftragsnummer).await?;
            protokoll_ext = Some(protocol.protokoll_msg);
        }

        am.logout(login.session_id).await?;

        result
    });

    let data = match e {
        Err(e) => {
            BackgroundThreadReturn::Fehler { 
                verfahrens_uuid: verfahren_uuid.clone(),
                log: protokoll_ext.unwrap_or_default(),
            }
        },
        Ok(_) => {
            BackgroundThreadReturn::Fortgefuehrt { 
                verfahrens_uuid: verfahren_uuid.clone(), 
            }  
        }
    };

    sender.send(ThreadReceiveMsg::WriteBack(ThreadWriteBackMsg {
        data: RefAny::new(data),
        callback: WriteBackCallback { cb: writeback_callback }
    }));
}

#[derive(Debug)]
struct BackgroundThreadInit {
    konfiguration: LefisUploadKonfiguration,
    geladene_verfahren: GeladeneVerfahren,
    ausgewaehltes_verfahren: String,
}

#[derive(Debug, Clone)]
enum BackgroundThreadReturn {
    Fortgefuehrt { verfahrens_uuid: String },
    FortschrittUpdate { verfahrens_uuid: String, prozent: usize },
    Fehler { verfahrens_uuid: String, log: Vec<ProtokollMsg>, },
}

use crate::wsdl::FortfuehrungsAuftrag;
use crate::LefisDatei;

extern "C" fn ffa_background_thread(
    mut initial_data: RefAny,
    mut sender: ThreadSender,
    _recv: ThreadReceiver,
) {
    use crate::wsdl::{
        AuftragsManager, 
        RequestFailure,
        ProtokollMsg,
    };
    use azul::task::*;
    use azul::callbacks::WriteBackCallback;

    let initial_data = match initial_data.downcast_ref::<BackgroundThreadInit>() {
        Some(s) => s,
        None => return, // error
    };
    let initial_data = &*initial_data;

    let geladene_verfahren = &initial_data.geladene_verfahren;
    let ausgewaehltes_verfahren = initial_data.ausgewaehltes_verfahren.as_str();
    let verfahren = match geladene_verfahren.verfahren.iter().find(|v| v.uuid.as_str() == ausgewaehltes_verfahren) {
        Some(s) => s,
        None => return, // error
    };
    let konfiguration = &initial_data.konfiguration.lefis;

    let lefis_geladen = match verfahren.lefis_geladen.as_ref() {
        Some(s) => s.clone(),
        None => return, // error
    };

    let am = AuftragsManager::new(
        &konfiguration.get_webservice_url(), 
        konfiguration.benutzer.clone(), 
        konfiguration.passwort.clone()
    );
    
    sender.send(ThreadReceiveMsg::WriteBack(ThreadWriteBackMsg {
        data: RefAny::new(BackgroundThreadReturn::FortschrittUpdate { 
            verfahrens_uuid: verfahren.uuid.clone(), 
            prozent: 0,
        }),
        callback: WriteBackCallback { cb: writeback_callback }
    }));

    let mut warnungen = Vec::new();
    let ffa_xml = generiere_ffa(
        &geladene_verfahren,
        &ausgewaehltes_verfahren,
        &lefis_geladen, 
        &mut warnungen
    ).get_xml();

    let mut protokoll_ext = None;
    let e: Result<(), RequestFailure> = exec_sync(async {
        let login = am.login().await?;
        let auftragsnummer = match am.register_gzip(login.session_id, &ffa_xml).await {
            Ok(o) => Ok(o.auftragsnummer),
            Err(e) => {
                am.logout(login.session_id).await?;
                Err(e)
            }
        }?;

        let result = warte_auf_auftrag_fortfuehrung(&am, login.session_id, &auftragsnummer).await;

        if let Err(e) = &result {
            let protocol = am.get_protocol_gzip(login.session_id, &auftragsnummer).await?;
            protokoll_ext = Some(protocol.protokoll_msg);
        }

        am.logout(login.session_id).await?;

        result
    });

    let data = match e {
        Err(e) => {
            BackgroundThreadReturn::Fehler { 
                verfahrens_uuid: verfahren.uuid.clone(),
                log: protokoll_ext.unwrap_or_default(),
            }
        },
        Ok(_) => {
            BackgroundThreadReturn::Fortgefuehrt { 
                verfahrens_uuid: verfahren.uuid.clone(), 
            }  
        }
    };

    sender.send(ThreadReceiveMsg::WriteBack(ThreadWriteBackMsg {
        data: RefAny::new(data),
        callback: WriteBackCallback { cb: writeback_callback }
    }));
}

fn generiere_ffa(
    geladene_verfahren: &GeladeneVerfahren,
    aktives_verfahren: &str,
    lefis_geladen: &[LefisDatei], 
    warnungen: &mut Vec<String>
) -> FortfuehrungsAuftrag {

    use crate::wsdl::{
        FfaInsert, FfaDelete, FfaReplace,
        FfaLxAbteilung2, FfaLxAbteilung3,
        FfaLxBuchungsstelleBelastetAbt2,
        FfaLxBuchungsstelleBelastetAbt3,
        FfaLxOrdnungsNummer, Anrede,
        FfaLxPersonRolle, FfaLxPerson,
        FfaAxPerson,

        KennzeichnungAlterNeuerBestand
    };
    use chrono::Utc;
    use crate::BvEintrag;
    use std::collections::{BTreeMap, BTreeSet};

    let jetzt = Utc::now();
    let kan = KennzeichnungAlterNeuerBestand::AlterBestand; // TODO: konfigurieren!
    let kurztexte_benutzen = true;
    let verfahren = match geladene_verfahren.verfahren.iter().find(|v| v.uuid.as_str() == aktives_verfahren) {
        Some(s) => s,
        None => return FortfuehrungsAuftrag::default(),
    };

    // konfiguration: LefisUploadKonfiguration,
    // verfahren: VerfahrenGeladen,

    let mut uuid_counter = 0;

    // generiert eine neue UUID im Format "DE_000000000000q"
    let mut generiere_neue_uuid = || {
        uuid_counter += 1;
        let uuid_name = column_name_from_number(uuid_counter).to_lowercase();
        let uuid_len = uuid_name.len();
        let max_len = 13_usize;
        let pad_0 = (0..max_len.saturating_sub(uuid_len)).map(|_| '0').collect::<String>();
        format!("DE_{}{}", pad_0, uuid_name)
    };

    let mut fsk_nicht_gefunden = BTreeMap::new();

    let mut global_delete = Vec::new();
    let mut global_insert = Vec::new();
    let mut global_replace = Vec::new();
    let mut alle_ordnungsnummern = BTreeMap::new();
    let mut ordnungsnummern_zu_rechte_map = BTreeMap::new();

    let mut bereits_gelöscht_abt3 = BTreeSet::new();
    let mut bereits_gelöscht_abt2 = BTreeSet::new();

    for grundbuchblatt in lefis_geladen.iter() {

        let grundbuch_name = format!("{} Blatt {}", grundbuchblatt.titelblatt.grundbuch_von, grundbuchblatt.titelblatt.blatt);

        let mut delete = Vec::new();
        let mut insert = Vec::new();
        let mut replace = Vec::new();

        let buchungsblattbezirke = match verfahren.buchungsblattbezirke.get(&grundbuchblatt.titelblatt.grundbuch_von) {
            Some(s) => s.clone(),
            None => {
                warnungen.push(format!("Konnte Grundbuchbezirks-ID für {:?} nicht finden", grundbuchblatt.titelblatt.grundbuch_von));
                continue;
            }
        };

        let mut gbb_vorhanden = None;
        for bb in buchungsblattbezirke {        
            match verfahren.buchungsblatt_bodenordnung.iter().find(|lx_buchungsblatt_bodenordnung| {
                lx_buchungsblatt_bodenordnung.ax_buchungsblatt.lan16 == bb.lan16 &&
                lx_buchungsblatt_bodenordnung.ax_buchungsblatt.bbb == bb.bbb &&
                lx_buchungsblatt_bodenordnung.ax_buchungsblatt.bbn == grundbuchblatt.titelblatt.blatt
            }) {
                Some(s) => { gbb_vorhanden = Some(s.clone()); break; },
                None => { },
            };
        }

        let gbb_vorhanden = match gbb_vorhanden {
            Some(s) => s,
            None => {
                warnungen.push(format!(
                    "{} Blatt {} gehört nicht zu Verfahren", 
                    grundbuchblatt.titelblatt.grundbuch_von,
                    grundbuchblatt.titelblatt.blatt,
                ));
                continue;
            }
        };

        for a2 in grundbuchblatt.rechte.abt2.iter() {
            let onr = match a2.nebenbeteiligter.ordnungsnummer.clone() {
                Some(s) => s,
                None => continue,
            };

            alle_ordnungsnummern
            .entry(onr)
            .or_insert_with(|| BTreeMap::new())
            .entry(gbb_vorhanden.uuid.clone())
            .or_insert_with(|| Vec::new())
            .push((a2.nebenbeteiligter.clone(), (gbb_vorhanden.ax_buchungsblatt.lan16, gbb_vorhanden.ax_buchungsblatt.bbb)));
        }

        for a3 in grundbuchblatt.rechte.abt3.iter() {

            let onr = match a3.nebenbeteiligter.ordnungsnummer.clone() {
                Some(s) => s,
                None => continue,
            };

            alle_ordnungsnummern
            .entry(onr)
            .or_insert_with(|| BTreeMap::new())
            .entry(gbb_vorhanden.uuid.clone())
            .or_insert_with(|| Vec::new())
            .push((a3.nebenbeteiligter.clone(), (gbb_vorhanden.ax_buchungsblatt.lan16, gbb_vorhanden.ax_buchungsblatt.bbb)));
        }

        'a2_inner: for a2_neu in grundbuchblatt.rechte.abt2.iter() {

            let neue_uuid = generiere_neue_uuid();
            let neue_buchungsstelle_uuid = generiere_neue_uuid();

            let onr = match a2_neu.nebenbeteiligter.ordnungsnummer.clone() {
                Some(s) => s,
                None => continue,
            };

            ordnungsnummern_zu_rechte_map.entry(onr)
            .or_insert_with(|| Vec::new())
            .push(neue_uuid.clone());

            let mut grundstuecke_belastet = Vec::new();
            for belastet in a2_neu.belastete_flurstuecke.iter() {

                let belastet_zahler = match belastet.flurstueck.split("/").nth(0).and_then(|p| p.parse::<usize>().ok()) {
                    Some(s) => format!("{:05}", s),
                    None => {
                        warnungen.push(format!("{} Blatt {}: Unlesbares Flurstückskennzeichen: {:?}", 
                            grundbuchblatt.titelblatt.grundbuch_von,
                            grundbuchblatt.titelblatt.blatt,
                            belastet.flurstueck, 
                        ));
                        continue;
                    }
                };

                let belastet_nenner = match belastet.flurstueck.split("/").nth(1) {
                    Some(s) => match s.parse::<usize>().ok() {
                        Some(s) => format!("{:04}", s),
                        None => {
                            warnungen.push(format!("{} Blatt {}: Unlesbarer Nenner: {:?}", 
                                grundbuchblatt.titelblatt.grundbuch_von,
                                grundbuchblatt.titelblatt.blatt,
                                belastet.flurstueck, 
                            ));
                            continue;
                        }
                    },
                    None => format!("____"),
                };

                // Suche in jetzigem Verfahren
                let mut gefunden = None;

                // Eine Gemarkung kann mehrere IDs haben
                let belastet_gemarkungsbezirk = belastet.gemarkung.as_ref()
                    .and_then(|s| if s.trim().is_empty() { None } else { Some(s.trim())})
                    .unwrap_or(&grundbuchblatt.titelblatt.grundbuch_von);
                
                let gemarkung_ids = match verfahren.gemarkungen.get(belastet_gemarkungsbezirk) {
                    Some(s) => s.clone(),
                    None => {
                        warnungen.push(format!("Konnte Gemarkungs-ID für {:?} nicht finden", belastet_gemarkungsbezirk));
                        continue;
                    }
                };
                
                'gemarkung_loop: for bb in gemarkung_ids.iter() {

                    if gefunden.is_some() { break 'gemarkung_loop; }
                    
                    let suche_fsk = format!("{:02}{:04}{:03}{}{}__", 
                        bb.lan19, bb.gmn19, 
                        belastet.flur, belastet_zahler, 
                        belastet_nenner
                    );

                    gefunden = verfahren.flurstuecke
                    .iter()
                    .find_map(|ax| {
                        if ax.kennzeichen == suche_fsk { Some(ax.clone()) } else { None }
                    });

                    if gefunden.is_some() { break 'gemarkung_loop; }

                    // Suche in anderen Verfahren
                    let gefunden_in_anderen_verfahren = geladene_verfahren.verfahren.iter().find_map(|v| {
                        if v.uuid == verfahren.uuid { return None; }
                        v.flurstuecke
                        .iter()
                        .find_map(|ax| if ax.kennzeichen == suche_fsk { Some(()) } else { None })
                    }).is_some();

                    if gefunden_in_anderen_verfahren {
                        // Flurstück gefunden, aber nicht Teil von Verfahren
                        continue 'a2_inner;
                    }

                    // Suche in Flurstücken außerhalb von allen Verfahren
                    let gefunden_außerhalb_aller_verfahren = geladene_verfahren.flurstuecke_ohne_verfahren
                        .iter()
                        .any(|ax| ax.kennzeichen == suche_fsk);
                    
                    if gefunden_außerhalb_aller_verfahren {
                        // Flurstück gefunden, aber nicht Teil von Verfahren
                        continue 'a2_inner;
                    }
                }

                match gefunden {
                    Some(ax) => {
                        // Unter welcher Nr. ist das Flurstück im momentanen Grundbuchblatt geführt
                        let gefuehrt_unter_lfd_nr = verfahren.abt3_rechte.iter()
                        .filter_map(|a| a.buchungsstellen.iter().find_map(|b| {
                            if b.ax21008 == ax.ax21008 && 
                               b.ax21007 == gbb_vorhanden.ax_buchungsblatt.uuid { 
                                Some(b.lfd_nr_grundbuch) 
                            } else {
                                None
                            }
                        }))
                        .collect::<Vec<_>>();

                        if gefuehrt_unter_lfd_nr.contains(&belastet.lfd_nr) {
                            grundstuecke_belastet.push(ax.lx21008); 
                        } else if !gefuehrt_unter_lfd_nr.is_empty() {
                            /*
                            warnungen.push(format!("{} Blatt {} Abt. 2 Recht {}: Flurstück {} unter lfd. Nr. {:?} gefunden, erwartete lfd. Nr. {}", 
                                grundbuchblatt.titelblatt.grundbuch_von,
                                grundbuchblatt.titelblatt.blatt,
                                a2_neu.lfd_nr,
                                ax.lx21008,
                                gefuehrt_unter_lfd_nr,
                                belastet.lfd_nr,
                            ));
                            */
                            grundstuecke_belastet.push(ax.lx21008); 
                        } else {
                            fsk_nicht_gefunden.entry((belastet_gemarkungsbezirk, belastet.flur, belastet.flurstueck.clone()))
                            .or_insert_with(|| Vec::new())
                            .push(format!("{} Blatt {} Abt. 2 Recht {}", 
                                grundbuchblatt.titelblatt.grundbuch_von,
                                grundbuchblatt.titelblatt.blatt,
                                a2_neu.lfd_nr 
                            ));
                            continue 'a2_inner;
                        }
                    },
                    None => {
                        fsk_nicht_gefunden.entry((belastet_gemarkungsbezirk, belastet.flur, belastet.flurstueck.clone()))
                        .or_insert_with(|| Vec::new())
                        .push(format!("{} Blatt {} Abt. 2 Recht {}", 
                            grundbuchblatt.titelblatt.grundbuch_von,
                            grundbuchblatt.titelblatt.blatt,
                            a2_neu.lfd_nr 
                        ));
                        continue 'a2_inner;
                    },
                }
            }

            if grundstuecke_belastet.is_empty() {
                warnungen.push(format!("{} Blatt {} Abt. 2 Recht {}: Keine belastetbaren Flurstücke gefunden, kann kein Recht erzeugen", 
                    grundbuchblatt.titelblatt.grundbuch_von,
                    grundbuchblatt.titelblatt.blatt,
                    a2_neu.lfd_nr,
                ));
                continue;
            }

            insert.push(FfaInsert::Abteilung2(FfaLxAbteilung2 {
                grundbuch_name: grundbuch_name.clone(),
                neue_uuid: neue_uuid,
                beginnt_datum: jetzt.clone(),
                kan: kan,
                verfahren_uuid: verfahren.uuid.clone(),
                rechtsinhaber: Vec::new(), // wird in späterem Schritt ausgefüllt
                buchungsstelle_uuid: neue_buchungsstelle_uuid.clone(),
                lfd_nr: a2_neu.lfd_nr,
                textlicher_teil: if kurztexte_benutzen {
                    a2_neu.text_kurz.clone()
                } else {
                    a2_neu.text_original.clone()
                },
                rechteart: a2_neu.rechteart,
                rangvermerk: a2_neu.rangvermerk.clone().unwrap_or_default(),
            }));
            insert.push(FfaInsert::BuchungsstelleBelastetAbt2(FfaLxBuchungsstelleBelastetAbt2 { 
                neue_buchungsstelle_uuid,
                beginnt_datum: jetzt.clone(),
                kan: kan,
                verfahren_uuid: verfahren.uuid.clone(),
                grundstuecke_belastet,
                anteil_belastet_zaehler: 1,
                anteil_belastet_nenner: 1,
            })); 
        }

        'a3_outer: for a3_neu in grundbuchblatt.rechte.abt3.iter() {
            
            let neue_uuid = generiere_neue_uuid();
            let neue_buchungsstelle_uuid = generiere_neue_uuid();

            let onr = match a3_neu.nebenbeteiligter.ordnungsnummer.clone() {
                Some(s) => s,
                None => continue,
            };

            ordnungsnummern_zu_rechte_map.entry(onr)
            .or_insert_with(|| Vec::new())
            .push(neue_uuid.clone());

            let mut grundstuecke_belastet = Vec::new();
            'a3_inner: for belastet in a3_neu.belastete_flurstuecke.iter() {

                let belastet_zahler = match belastet.flurstueck.split("/").nth(0).and_then(|p| p.parse::<usize>().ok()) {
                    Some(s) => format!("{:05}", s),
                    None => {
                        warnungen.push(format!("{} Blatt {}: Unlesbares Flurstückskennzeichen: {:?}", 
                            grundbuchblatt.titelblatt.grundbuch_von,
                            grundbuchblatt.titelblatt.blatt,
                            belastet.flurstueck, 
                        ));
                        continue;
                    }
                };

                let belastet_nenner = match belastet.flurstueck.split("/").nth(1) {
                    Some(s) => match s.parse::<usize>().ok() {
                        Some(s) => format!("{:04}", s),
                        None => {
                            warnungen.push(format!("{} Blatt {}: Unlesbarer Nenner: {:?}", 
                                grundbuchblatt.titelblatt.grundbuch_von,
                                grundbuchblatt.titelblatt.blatt,
                                belastet.flurstueck, 
                            ));
                            continue;
                        }
                    },
                    None => format!("____"),
                };

                let mut gefunden = None;

                let belastet_gemarkungsbezirk = belastet.gemarkung.as_ref()
                    .and_then(|s| if s.trim().is_empty() { None } else { Some(s.trim())})
                    .unwrap_or(&grundbuchblatt.titelblatt.grundbuch_von);
                
                let gemarkung_ids = match verfahren.gemarkungen.get(belastet_gemarkungsbezirk) {
                    Some(s) => s.clone(),
                    None => {
                        warnungen.push(format!("Konnte Gemarkungs-ID für {:?} nicht finden", belastet_gemarkungsbezirk));
                        continue;
                    }
                };

                'gemarkung_loop: for bb in gemarkung_ids.iter() {

                    if gefunden.is_some() { break 'gemarkung_loop; }
                    
                    let suche_fsk = format!("{:02}{:04}{:03}{}{}__", 
                        bb.lan19, bb.gmn19, 
                        belastet.flur, belastet_zahler, 
                        belastet_nenner
                    );

                    gefunden = verfahren.flurstuecke
                    .iter()
                    .find_map(|ax| {
                        if ax.kennzeichen == suche_fsk { Some(ax.clone()) } else { None }
                    });

                    if gefunden.is_some() { break 'gemarkung_loop; }

                    // Suche in anderen Verfahren
                    let gefunden_in_anderen_verfahren = geladene_verfahren.verfahren.iter().find_map(|v| {
                        if v.uuid == verfahren.uuid { return None; }
                        v.flurstuecke
                        .iter()
                        .find_map(|ax| if ax.kennzeichen == suche_fsk { Some(ax.clone()) } else { None })
                    }).is_some();

                    if gefunden_in_anderen_verfahren {
                        // Flurstück vorhanden, aber nicht Teil von Verfahren
                        continue 'a3_inner;
                    }

                    // Suche in Flurstücken außerhalb von allen Verfahren
                    let gefunden_außerhalb_aller_verfahren = geladene_verfahren.flurstuecke_ohne_verfahren
                        .iter()
                        .any(|ax| ax.kennzeichen == suche_fsk);

                    if gefunden_außerhalb_aller_verfahren {
                        // Flurstück vorhanden, aber nicht Teil von Verfahren
                        continue 'a3_inner;
                    }
                }

                match gefunden {
                    Some(ax) => {                         
                        // Unter welcher Nr. ist das Flurstück im momentanen Grundbuchblatt geführt
                        let gefuehrt_unter_lfd_nr = verfahren.abt3_rechte.iter()
                        .filter_map(|a| a.buchungsstellen.iter().find_map(|b| {
                            if b.ax21008 == ax.ax21008 && 
                               b.ax21007 == gbb_vorhanden.ax_buchungsblatt.uuid { 
                                Some(b.lfd_nr_grundbuch) 
                            } else {
                                None
                            }
                        }))
                        .collect::<Vec<_>>();

                        if gefuehrt_unter_lfd_nr.contains(&belastet.lfd_nr) {
                            grundstuecke_belastet.push(ax.lx21008); 
                        } else if !gefuehrt_unter_lfd_nr.is_empty() {
                            /*
                            warnungen.push(format!("{} Blatt {} Abt. 3 Recht {}: Flurstück {} unter lfd. Nr. {:?} gefunden, erwartete lfd. Nr. {}", 
                                grundbuchblatt.titelblatt.grundbuch_von,
                                grundbuchblatt.titelblatt.blatt,
                                a3_neu.lfd_nr,
                                ax.lx21008,
                                gefuehrt_unter_lfd_nr,
                                belastet.lfd_nr,
                            ));
                            */
                            grundstuecke_belastet.push(ax.lx21008);
                        } else {
                            fsk_nicht_gefunden.entry((belastet_gemarkungsbezirk, belastet.flur, belastet.flurstueck.clone()))
                            .or_insert_with(|| Vec::new())
                            .push(format!("{} Blatt {} Abt. 3 Recht {}", 
                                grundbuchblatt.titelblatt.grundbuch_von,
                                grundbuchblatt.titelblatt.blatt,
                                a3_neu.lfd_nr 
                            ));
                            continue 'a3_inner;
                        }
                    },
                    None => {
                        fsk_nicht_gefunden.entry((belastet_gemarkungsbezirk, belastet.flur, belastet.flurstueck.clone()))
                        .or_insert_with(|| Vec::new())
                        .push(format!("{} Blatt {} Abt. 3 Recht {}", 
                            grundbuchblatt.titelblatt.grundbuch_von,
                            grundbuchblatt.titelblatt.blatt,
                            a3_neu.lfd_nr 
                        ));
                        continue 'a3_inner;
                    },
                }
            }

            if grundstuecke_belastet.is_empty() {
                warnungen.push(format!("{} Blatt {} Abt. 3 Recht {}: Keine belastbaren Flurstücke gefunden, kann kein Recht erzeugen", 
                    grundbuchblatt.titelblatt.grundbuch_von,
                    grundbuchblatt.titelblatt.blatt,
                    a3_neu.lfd_nr,
                ));
                continue;
            }

            insert.push(FfaInsert::Abteilung3(FfaLxAbteilung3 {
                grundbuch_name: grundbuch_name.clone(),
                neue_uuid,
                beginnt_datum: jetzt.clone(),
                kan: kan, 
                verfahren_uuid: verfahren.uuid.clone(),
                rechtsinhaber: Vec::new(), // wird in späterem Schritt ausgefüllt
                buchungsstelle_uuid: neue_buchungsstelle_uuid.clone(),
                lfd_nr: a3_neu.lfd_nr,
                textlicher_teil:  if kurztexte_benutzen {
                    a3_neu.text_kurz.clone()
                } else {
                    a3_neu.text_original.clone()
                },
                waehrung: a3_neu.betrag.waehrung,
                betrag: format!("{}.{:02}", a3_neu.betrag.wert, a3_neu.betrag.nachkomma),
                schuldenart: a3_neu.schuldenart,
            }));
            insert.push(FfaInsert::BuchungsstelleBelastetAbt3(FfaLxBuchungsstelleBelastetAbt3 { 
                neue_buchungsstelle_uuid: neue_buchungsstelle_uuid,
                beginnt_datum: jetzt.clone(),
                kan: kan,
                verfahren_uuid: verfahren.uuid.clone(),
                grundstuecke_belastet: grundstuecke_belastet,
            }));
        }

        for abt2 in verfahren.abt2_rechte.iter() {
            
            if abt2.ende.is_some() {
                continue;
            }

            if bereits_gelöscht_abt2.contains(&abt2.uuid) {
                continue;
            }

            let buchungsstelle = match abt2.buchungsstellen.len() {
                0 => { continue; },
                1 => {
                    abt2.buchungsstellen[0].clone()
                },
                _ => {
                    warnungen.push(format!("{} Abteilung 2 lfd. Nr. {} hat mehr als eine Grundbuchblatt-Buchungsstelle?", grundbuch_name, abt2.lfd_nr));
                    continue;
                }
            };

            if buchungsstelle.ax21007 != gbb_vorhanden.ax_buchungsblatt.uuid {
                continue;
            }

            bereits_gelöscht_abt2.insert(abt2.uuid.clone());

            // Wenn bereits ein insert existiert, Recht nicht 
            // löschen sondern ersetzen
            if let Some(insert_idx) = insert.iter().position(|i| match i {
                FfaInsert::Abteilung2(FfaLxAbteilung2 { lfd_nr, .. }) => *lfd_nr == abt2.lfd_nr,
                _ => false,
            }) {
                let insert_buchungsstelle = match insert.remove(insert_idx + 1) {
                     FfaInsert::BuchungsstelleBelastetAbt2(a) => a,
                     _ => continue,
                };
                let insert = match insert.remove(insert_idx) {
                     FfaInsert::Abteilung2(a) => a,
                     _ => continue,
                };

                replace.push(FfaReplace::Abteilung2 {
                    grundbuch_name: grundbuch_name.clone(),
                    lfd_nr: abt2.lfd_nr,
                    uuid: abt2.uuid.clone(),
                    erstellt_am: abt2.erstellt_am.clone(), 
                    insert,
                }); 
                replace.push(FfaReplace::BuchungsstelleBelastetAbt2 {
                    uuid: buchungsstelle.lx21004.clone(),
                    erstellt_am: buchungsstelle.lx21004_erstellt_am.clone(), 
                    insert: insert_buchungsstelle,
                }); 
            } else {
                delete.push(FfaDelete::Abteilung2 { 
                    grundbuch_name: grundbuch_name.clone(),
                    lfd_nr: abt2.lfd_nr,
                    uuid: abt2.uuid.clone(),
                    erstellt_am: abt2.erstellt_am.clone(), 
                }); 
                delete.push(FfaDelete::BuchungsstelleBelastet { 
                    uuid: buchungsstelle.lx21004.clone(),
                    erstellt_am: buchungsstelle.lx21004_erstellt_am.clone(), 
                });
            }
        }

        for abt3 in verfahren.abt3_rechte.iter() {
            
            if abt3.ende.is_some() {
                continue;
            }

            if bereits_gelöscht_abt3.contains(&abt3.uuid) {
                continue;
            }

            let buchungsstelle = match abt3.buchungsstellen.len() {
                0 => { continue; },
                1 => {
                    abt3.buchungsstellen[0].clone()
                },
                _ => {
                    warnungen.push(format!("{} Abteilung 3 lfd. Nr. {} hat mehr als eine Grundbuchblatt-Buchungsstelle?", grundbuch_name, abt3.lfd_nr));
                    continue;
                }
            };

            if buchungsstelle.ax21007 != gbb_vorhanden.ax_buchungsblatt.uuid {
                continue;
            }

            let position = insert.iter().position(|i| match i {
                FfaInsert::Abteilung3(FfaLxAbteilung3 { lfd_nr, .. }) => *lfd_nr == abt3.lfd_nr,
                _ => false,
            });
            
            bereits_gelöscht_abt3.insert(abt3.uuid.clone());

            if let Some(insert_idx) = position {
                let insert_buchungsstelle = match insert.remove(insert_idx + 1) {
                    FfaInsert::BuchungsstelleBelastetAbt3(a) => a,
                    _ => continue,
                };
                let insert = match insert.remove(insert_idx) {
                    FfaInsert::Abteilung3(a) => a,
                    _ => continue,
                };

                replace.push(FfaReplace::Abteilung3 { 
                    grundbuch_name: grundbuch_name.clone(),
                    lfd_nr: abt3.lfd_nr,
                    uuid: abt3.uuid.clone(),
                    erstellt_am: abt3.erstellt_am.clone(), 
                    insert,
                }); 
                replace.push(FfaReplace::BuchungsstelleBelastetAbt3 {
                    uuid: buchungsstelle.lx21004.clone(),
                    erstellt_am: buchungsstelle.lx21004_erstellt_am.clone(), 
                    insert: insert_buchungsstelle,
                }); 
            } else {
                delete.push(FfaDelete::Abteilung3 {
                    grundbuch_name: grundbuch_name.clone(),
                    lfd_nr: abt3.lfd_nr,
                    uuid: abt3.uuid.clone(),
                    erstellt_am: abt3.erstellt_am.clone(), 
                });
                delete.push(FfaDelete::BuchungsstelleBelastet { 
                    uuid: buchungsstelle.lx21004.clone(),
                    erstellt_am: buchungsstelle.lx21004_erstellt_am.clone(), 
                });
            }
        }

        global_insert.append(&mut insert);
        global_replace.append(&mut replace);
        global_delete.append(&mut delete);
    }

    // Jeder NB sollte exakt eine Ordnungsnummer haben
    let mut ordnungsnummer_nb_map = BTreeMap::new();
    for (onr, onr_rechte) in alle_ordnungsnummern {

        let mut nb_pro_onr = onr_rechte
            .iter()
            .flat_map(|(k, v)| v.clone().into_iter())
            .collect::<Vec<_>>();

        nb_pro_onr.sort();
        dedup_by::dedup_by(&mut nb_pro_onr, |a, b| {
            a.0.name == b.0.name
        });

        if nb_pro_onr.len() > 1 {
            warnungen.push(format!("Kann Ordnungsnummer {}/00 nicht ersetzen: Mehr als ein Nebenbeteiliger für Ordnungsnummer: {:?}", onr, 
                nb_pro_onr
            ));
            continue;
        }

        let (nb, (lan16, bbb)) = match nb_pro_onr.first() {
            Some((nb, gmk)) => (nb.clone(), gmk.clone()),
            None => {
                warnungen.push(format!("Kann Ordnungsnummer {}/00 nicht ersetzen: Kein Nebenbeteiliger in Grundbuch?", onr));
                continue;
            }
        };

        ordnungsnummer_nb_map.insert(onr, (nb, lan16, bbb));
    }

    // Ordnungsnummer ("817000") zu LxOrdnungsnummerBodenOrdnung UUID ("DE....")
    // Nötig, um Verbindung zw. Recht -> Rechtsinhaber (Ordnungsnummer) zu erstellen
    let mut onr_zu_lx_ordnungsnummer_bodenordnung_map = BTreeMap::new();

    for (onr, (nb, lan16, bbb)) in ordnungsnummer_nb_map {

        let onr_existiert = verfahren.ordnungsnummern.iter().find(|o| {
            onr == o.stammnummer &&
            o.unternummer == 0
        }).cloned();

        match onr_existiert {
            Some(s) => {

                let namensnummern_fuer_ordnungsnummer = s.get_lx_namensnummern();
                if namensnummern_fuer_ordnungsnummer.len() > 1 {
                    // Irgendwas falsch: NB sollten nur eine Namensnummer haben
                    warnungen.push(format!("Kann Ordnungsnummer {}/00 nicht ersetzen: Mehr als eine Namensnummer in LX_BuchungsblattBodenordnung", onr));
                }

                let person_rolle = match namensnummern_fuer_ordnungsnummer.first().and_then(|f| {
                    verfahren.personen_rollen.iter().find(|pr| pr.lx_namensnummer_uuid == *f)
                }) {
                    Some(s) => s.clone(),
                    None => {
                        warnungen.push(format!("Kann Ordnungsnummer {}/00 nicht ersetzen: Keine LX_PersonRolle für LX_Namensnummer {:?}", 
                            onr, namensnummern_fuer_ordnungsnummer
                        ));
                        continue;
                    }
                };

                let lx_person = match verfahren.personen.iter().find(|p| p.uuid == person_rolle.person_uuid) {
                    Some(s) => s.clone(),
                    None => {
                        warnungen.push(format!("Kann Ordnungsnummer {}/00 nicht ersetzen: Keine LX_Person für LX_PersonRolle {}", 
                            onr, person_rolle.uuid
                        ));
                        continue;
                    }
                };

                onr_zu_lx_ordnungsnummer_bodenordnung_map.insert(onr, s.uuid);

                global_replace.push(FfaReplace::NebenbeteiligterReplace {
                    nebenbeteiligter_stammnr: onr,
                    lx_person_rolle_erstellt_am: person_rolle.beg.clone(),
                    lx_person_rolle: FfaLxPersonRolle {
                        personenrolle_uuid: person_rolle.uuid.clone(),
                        beginnt_datum: jetzt.clone(),
                        kan: KennzeichnungAlterNeuerBestand::AlterBestand,
                        verfahren_uuid: verfahren.uuid.clone(),
                    },

                    lx_person_erstellt_am: lx_person.beg.clone(),
                    lx_person: FfaLxPerson {
                        lx_person_uuid: lx_person.uuid.clone(),
                        beginnt_datum: jetzt.clone(),
                        verfahren_uuid: verfahren.uuid.clone(),
                        personenrolle_uuid: person_rolle.uuid.clone(),
                        ax_person_uuid: lx_person.ax_person.uuid.clone(),
                    },

                    ax_person_erstellt_am: lx_person.ax_person.beg.clone(),
                    ax_person: FfaAxPerson {
                        ax_person_uuid: lx_person.ax_person.uuid.clone(),
                        beginnt_datum: jetzt.clone(),
                        
                        anrede: nb.extra.anrede.clone(),
                        titel: nb.extra.titel.clone(),
                        vorname: nb.extra.vorname.clone(),
                        nachname_oder_firma: nb.extra.nachname_oder_firma.unwrap_or(nb.name.trim().to_string()),
                        geburtsname: nb.extra.geburtsname.clone(),
                        geburtsdatum: nb.extra.geburtsdatum.clone(),
                        wohnort: nb.extra.wohnort.clone(),
                    },
                });
            },
            None => {

                let ordnungsnummer_bodenordnung_uuid = generiere_neue_uuid();
                onr_zu_lx_ordnungsnummer_bodenordnung_map.insert(onr, ordnungsnummer_bodenordnung_uuid.clone());

                global_insert.push(FfaInsert::NebenbeteiligterNeu(FfaLxOrdnungsNummer {
                    ordnungsnummer_bodenordnung_uuid,
                    beginnt_datum: jetzt.clone(),
                    kan: KennzeichnungAlterNeuerBestand::AlterBestand,
                    verfahren_uuid: verfahren.uuid.clone(),
                    stammnummer: onr,
                    unternummer: 0,
                    personenrolle_uuid: generiere_neue_uuid(),
                    lx_person_uuid: generiere_neue_uuid(),
                    ax_person_uuid: generiere_neue_uuid(),
                    
                    anrede: nb.extra.anrede.clone(),
                    titel: nb.extra.titel.clone(),
                    vorname: nb.extra.vorname.clone(),
                    nachname_oder_firma: nb.extra.nachname_oder_firma.unwrap_or(nb.name.trim().to_string()),
                    geburtsname: nb.extra.geburtsname.clone(),
                    geburtsdatum: nb.extra.geburtsdatum.clone(),
                    wohnort: nb.extra.wohnort.clone(),

                    buchungsblatt_uuid: generiere_neue_uuid(),
                    bb_land: lan16.to_string(),
                    bb_bezirk: bbb.to_string(),
                    bbn_mit_erweiterung: format!("v00001"),
                    buchungsblatt_bodenordnung_uuid: generiere_neue_uuid(),
                    ax_namensnummer_uuid: generiere_neue_uuid(),
                    lx_namensnummer_uuid: generiere_neue_uuid(),
                }));
            }
        }
    }

    // ordnungsnummern_zu_rechte_map
    for (ordnungsnummer, rechte) in ordnungsnummern_zu_rechte_map {
        
        let lx_ordnungsnummer_bodenordnung = match onr_zu_lx_ordnungsnummer_bodenordnung_map.get(&ordnungsnummer) {
            Some(s) => s,
            None => {
                warnungen.push(format!("Konnte keine LxOrdnungsnummerBodenordnung für {}/00 finden.", ordnungsnummer));
                continue; 
            },
        };

        for recht_uuid in rechte {
            for insert in global_insert.iter_mut() {
                match insert {
                    FfaInsert::Abteilung2(i) => {
                        if i.neue_uuid != recht_uuid { continue; }
                        i.rechtsinhaber.push(lx_ordnungsnummer_bodenordnung.clone());
                        i.rechtsinhaber.sort();
                        i.rechtsinhaber.dedup();
                    },
                    FfaInsert::Abteilung3(i) => {
                        if i.neue_uuid != recht_uuid { continue; }
                        i.rechtsinhaber.push(lx_ordnungsnummer_bodenordnung.clone());
                        i.rechtsinhaber.sort();
                        i.rechtsinhaber.dedup();
                    },
                    _ => { },
                }
            }
            for replace in global_replace.iter_mut() {
                match replace {
                    FfaReplace::Abteilung2 { insert, .. } => {
                        if insert.neue_uuid != recht_uuid { continue; }
                        insert.rechtsinhaber.push(lx_ordnungsnummer_bodenordnung.clone());
                        insert.rechtsinhaber.sort();
                        insert.rechtsinhaber.dedup();
                    },
                    FfaReplace::Abteilung3 { insert, .. } => {
                        if insert.neue_uuid != recht_uuid { continue; }
                        insert.rechtsinhaber.push(lx_ordnungsnummer_bodenordnung.clone());
                        insert.rechtsinhaber.sort();
                        insert.rechtsinhaber.dedup();
                    },
                    _ => { },
                }
            }
        }
    }

    for i in &global_insert {
        let (ri, desc) = match i {
            FfaInsert::Abteilung2(i) => (&i.rechtsinhaber, &i.grundbuch_name),
            FfaInsert::Abteilung3(i) => (&i.rechtsinhaber, &i.grundbuch_name),
            _ => { continue; },
        };
        if ri.is_empty() {
            warnungen.push(format!("Recht {} hat keine Rechtsinhaber", desc));
        }
    }
    for r in &global_replace {
        let (ri, desc) = match r {
            FfaReplace::Abteilung2 { insert, .. } => (&insert.rechtsinhaber, &insert.grundbuch_name),
            FfaReplace::Abteilung3 { insert, .. } => (&insert.rechtsinhaber, &insert.grundbuch_name),
            _ => { continue; },
        };
        if ri.is_empty() {
            warnungen.push(format!("Recht {} hat keine Rechtsinhaber", desc));
        }
    }

    for ((gmk, flur, flurstueck), rechte) in fsk_nicht_gefunden {
        warnungen.push(format!("Gemarkung {} Fl. {} Flst. {} nicht gefunden, benötigt von:\r\n{}",
            gmk, flur, flurstueck, rechte.into_iter().map(|r| format!("    {}", r)).collect::<Vec<_>>().join("\r\n")
        ));
    }

    warnungen.sort();
    warnungen.dedup();
    warnungen.reverse();

    FortfuehrungsAuftrag {
        verfahren_name: verfahren.name.clone(),
        verfahren_id: verfahren.nummer,
        insert: global_insert,
        replace: global_replace,
        delete: global_delete,
    }
}

async fn warte_auf_auftrag_fortfuehrung(am: &AuftragsManager, session_id: usize, auftragsnummer: &str) -> Result<(), RequestFailure> {
    
    use std::thread;
    use std::time::Duration;

    loop {
        
        let status = am.list_auftrag(session_id, auftragsnummer).await?;

        match status.status {
            18 => break,
            1 | 3 => {
                thread::sleep(Duration::from_secs(1));
                continue;
            },
            n => {
                return Err(RequestFailure::FailedToDeserializeResponse(format!("ListAuftrag: Unbekannter Status: {}", n)));
            },
        }
    }

    let state = am.get_state(session_id, auftragsnummer).await?;
    if state.state != 18 {
        return Err(RequestFailure::FailedToDeserializeResponse(format!("GetState: Unbekannter Status: {}", state.state)));
    }

    let result_count = am.get_result_count(session_id, auftragsnummer).await?;
    for i in 0..result_count.result_count {
        let result_gzip = am.get_n_result_gzip(session_id, auftragsnummer, i).await?;
        if !result_gzip.erfolgreich {
            return Err(RequestFailure::FailedToDeserializeResponse(format!("GetNResultGzip: Auftrag {} wurde nicht erfolgreich fortgeführt. Bitte Protokoll beachten", auftragsnummer)));
        }
    }

    Ok(())
}

/// Maps an index number to a value, necessary for creating the column name:
///
/// ```no_run,ignore
/// 0   -> A
/// 25  -> Z
/// 26  -> AA
/// 27  -> AB
/// ```
fn column_name_from_number(num: usize) -> String {
    
    #[inline(always)]
    fn u8_to_char(input: u8) -> u8 {
        'A' as u8 + input
    }

    const ALPHABET_LEN: usize = 26;
    // usize::MAX is "GKGWBYLWRXTLPP" with a length of 15 characters
    const MAX_LEN: usize = 15;

    let mut result = [0;MAX_LEN + 1];
    let mut multiple_of_alphabet = num / ALPHABET_LEN;
    let mut character_count = 0;

    while multiple_of_alphabet != 0 && character_count < MAX_LEN {
        let remainder = (multiple_of_alphabet - 1) % ALPHABET_LEN;
        result[(MAX_LEN - 1) - character_count] = u8_to_char(remainder as u8);
        character_count += 1;
        multiple_of_alphabet = (multiple_of_alphabet - 1) / ALPHABET_LEN;
    }

    result[MAX_LEN] = u8_to_char((num % ALPHABET_LEN) as u8);
    let zeroed_characters = MAX_LEN.saturating_sub(character_count);
    let slice = &result[zeroed_characters..];
    unsafe { ::std::str::from_utf8_unchecked(slice) }.to_string()
}

extern "C" fn writeback_callback(
    app_data: &mut RefAny, 
    incoming_data: &mut RefAny, 
    _: &mut CallbackInfo
) -> Update {

    use self::BackgroundThreadReturn::*;
    use crate::Auftragsstatus;

    let mut data_mut = match app_data.downcast_mut::<AppData>() {
        Some(s) => s,
        None => return Update::DoNothing,
    };

    let mut incoming_data = match incoming_data.downcast_mut::<BackgroundThreadReturn>() {
        Some(s) => s,
        None => return Update::DoNothing,
    };

    let verfahrens_uuid = match &*incoming_data {
        Fortgefuehrt { verfahrens_uuid } |
        FortschrittUpdate { verfahrens_uuid, .. } |
        Fehler { verfahrens_uuid, .. } => verfahrens_uuid.clone(),
    };

    let mut aktives_verfahren = match data_mut.geladene_verfahren.verfahren.iter_mut().find(|d| d.uuid == verfahrens_uuid) {
        Some(s) => s,
        None => return Update::DoNothing,
    };

    aktives_verfahren.auftragsstatus = match &mut *incoming_data {
        Fortgefuehrt { .. } => Some(Auftragsstatus::ErfolgreichFortgefuehrt),
        FortschrittUpdate { prozent, .. } => Some(Auftragsstatus::AuftragWirdFortgefuehrt { prozent: *prozent }),
        Fehler { log, .. } => Some(Auftragsstatus::Fehler { log: log.clone() }),
    };

    Update::RefreshDom
}


pub fn render_konfigurations_dialog(app_data: RefAny, data: &LefisUploadKonfiguration) -> Dom {
    
    use azul::widgets::{Button, TextInput, NumberInput, CheckBox};

    fn render_row(s: &'static str, data: Dom) -> Dom {
        div::render()
        .with_inline_style("flex-grow:0;flex-direction:row;margin-bottom:5px;".into())
        .with_children(vec![
            p::render(s.into())
            .with_inline_style("width:100px;font-family:sans-serif;font-size:11px;".into()),
            div::render()
            .with_inline_style("flex-grow:1;".into()),
            Dom::div()
            .with_inline_style("width:200px;".into())
            .with_child(data),
        ].into())
    }

    Dom::body()
    .with_inline_style("padding:10px;".into())
    .with_children(DomVec::from_vec(vec![
        render_row("HTTPS benutzen", 
            CheckBox::new(data.lefis.https)
            .with_on_toggle(app_data.clone(), konfigurations_https_bearbeiten)
            .dom()
        ),
        render_row("Host", 
            TextInput::new()
            .with_text(data.lefis.host.clone().into())
            .with_on_focus_lost(app_data.clone(), konfiguration_host_bearbeiten)
            .dom()
        ),
        render_row("Port", 
            NumberInput::new(data.lefis.port as f32)
            .with_on_focus_lost(app_data.clone(), konfiguration_port_bearbeiten)
            .dom()
        ),
        render_row("Service", 
            TextInput::new()
            .with_text(data.lefis.service.clone().into())
            .with_on_focus_lost(app_data.clone(), konfiguration_service_bearbeiten)
            .dom()
        ),
        render_row("Benutzer", 
            TextInput::new()
            .with_text(data.lefis.benutzer.clone().unwrap_or_default().into())
            .with_on_focus_lost(app_data.clone(), konfiguration_benutzer_bearbeiten)
            .dom()
        ),
        render_row("Passwort", 
            TextInput::new()
            .with_text(data.lefis.passwort.clone().unwrap_or_default().clone().into())
            .with_on_focus_lost(app_data.clone(), konfiguration_passwort_bearbeiten)
            .dom()
        ),
        div::render()
        .with_inline_style("flex-grow:1;".into()),
        div::render()
        .with_inline_style("flex-grow:0;flex-direction:row;".into())
        .with_children(vec![
            Button::new("Verbindung testen".into())
            .with_on_click(app_data.clone(), konfiguration_verbindung_testen)
            .dom()
            .with_inline_style("margin-right:50px;".into()),
            div::render()
            .with_inline_style("flex-grow:1;".into()),
            Button::new("Speichern".into())
            .with_on_click(app_data.clone(), konfiguration_speichern)
            .dom(),
        ].into()),
    ]))
}

use azul::callbacks::LayoutCallbackInfo;
use azul::style::StyledDom;
use azul::window::WindowCreateOptions;

extern "C" 
fn konfigurations_https_bearbeiten(data: &mut RefAny, _: &mut CallbackInfo, c: &CheckBoxState) -> Update {
    
    let mut data = match data.downcast_mut::<AppData>() {
        Some(s) => s,
        None => { return Update::DoNothing; },
    };

    data.konfiguration.lefis.https = c.checked;

    Update::DoNothing
}

extern "C" 
fn konfiguration_host_bearbeiten(data: &mut RefAny, _: &mut CallbackInfo, ts: &TextInputState) -> Update {
    
    let mut data = match data.downcast_mut::<AppData>() {
        Some(s) => s,
        None => { return Update::DoNothing; },
    };

    let t = ts.get_text().as_str().trim().to_string();
    data.konfiguration.lefis.host = t;

    Update::DoNothing
}


extern "C" 
fn konfiguration_port_bearbeiten(data: &mut RefAny, _: &mut CallbackInfo, n: &NumberInputState) -> Update {
    
    let mut data = match data.downcast_mut::<AppData>() {
        Some(s) => s,
        None => { return Update::DoNothing; },
    };

    data.konfiguration.lefis.port = n.number.abs().round() as usize;

    Update::DoNothing
}

extern "C" 
fn konfiguration_service_bearbeiten(data: &mut RefAny, _: &mut CallbackInfo, ts: &TextInputState) -> Update {
    
    let mut data = match data.downcast_mut::<AppData>() {
        Some(s) => s,
        None => { return Update::DoNothing; },
    };

    let t = ts.get_text().as_str().trim().to_string();
    data.konfiguration.lefis.service = t;

    Update::DoNothing
}

extern "C" 
fn konfiguration_benutzer_bearbeiten(data: &mut RefAny, _: &mut CallbackInfo, ts: &TextInputState) -> Update {
    
    let mut data = match data.downcast_mut::<AppData>() {
        Some(s) => s,
        None => { return Update::DoNothing; },
    };

    let t = ts.get_text().as_str().trim().to_string();
    data.konfiguration.lefis.benutzer = if t.is_empty() { None } else { Some(t) };

    Update::DoNothing
}

extern "C" 
fn konfiguration_passwort_bearbeiten(data: &mut RefAny, _: &mut CallbackInfo, ts: &TextInputState) -> Update {
    
    let mut data = match data.downcast_mut::<AppData>() {
        Some(s) => s,
        None => { return Update::DoNothing; },
    };

    let t = ts.get_text().as_str().trim().to_string();
    data.konfiguration.lefis.passwort = if t.is_empty() { None } else { Some(t) };

    Update::DoNothing
}

extern "C" 
fn konfiguration_verbindung_testen(data: &mut RefAny, info: &mut CallbackInfo) -> Update {
    
    use azul::dialog::MsgBox;
    use crate::LefisInfo;

    let data = match data.downcast_ref::<AppData>() {
        Some(s) => s,
        None => { return Update::DoNothing; },
    };

    match LefisInfo::new(&data.konfiguration.lefis) {
        Ok(o) => {
            MsgBox::info(format!("Verbindung erfolgreich!").into());
        },
        Err(e) => {
            MsgBox::error(format!("Fehler beim Auslesen der Oracle-Zugangsdaten: {}", e).into());
        },
    }

    Update::DoNothing
}

extern "C" 
fn konfiguration_speichern(data: &mut RefAny, info: &mut CallbackInfo) -> Update {

    {
        let data = match data.downcast_ref::<AppData>() {
            Some(s) => s,
            None => { return Update::DoNothing; },
        };
        
        data.konfiguration.speichern();
    }

    verfahren_neu_laden(data, info)
}

extern "C" fn konfigurations_dialog(data: &mut RefAny, _: &mut LayoutCallbackInfo) -> StyledDom {
    let data_clone = data.clone();
    let data = match data.downcast_ref::<AppData>() {
        Some(s) => s,
        None => return StyledDom::default(),
    };

    crate::ui::render_konfigurations_dialog(data_clone, &data.konfiguration)
    .style(Css::empty())
}

extern "C"
fn oeffne_konfiguration_editor(_: &mut RefAny, info: &mut CallbackInfo) -> Update {
    info.create_window(konfiguration_fenster());
    Update::RefreshDom
}

pub fn konfiguration_fenster() -> WindowCreateOptions {
    let mut window = WindowCreateOptions::new(konfigurations_dialog);
    window.state.title = "DHK-Verbindung".into();
    window.state.size.dimensions.width = 400.0;
    window.state.size.dimensions.height = 300.0;
    window.state.flags.is_always_on_top = true;
    window.state.flags.is_resizable = false;
    window
}

pub fn render(app_data: RefAny, data: &AppData) -> Dom {
    use azul::menu::*;

    Dom::body()
    .with_menu_bar(Menu::new({

        let mut menu = vec![
            MenuItem::String(StringMenuItem::new("Programm".into()).with_children(vec![
                MenuItem::String(
                    StringMenuItem::new("Konfiguration".into())
                    .with_callback(app_data.clone(), oeffne_konfiguration_editor)
                ),
                MenuItem::String(
                    StringMenuItem::new("Verfahren neu laden".into())
                    .with_callback(app_data.clone(), verfahren_neu_laden)
                )
            ].into()))
        ];

        if let Some(v) = data.ausgewaehltes_verfahren.clone().and_then(|n| data.geladene_verfahren.verfahren.iter().find(|v| v.uuid == n)) {
            menu.push(MenuItem::String(StringMenuItem::new("Ausgewähltes Verfahren".into()).with_children(vec![
                MenuItem::String(StringMenuItem::new("Exportieren...".into())
                    .with_callback(app_data.clone(), verfahren_exportieren)),
                MenuItem::String({
                    let mut menu_item = StringMenuItem::new("Fortführungsauftrag speichern unter...".into())
                    .with_callback(app_data.clone(), lefis_ffa_exportieren);
                    
                    if v.lefis_geladen.is_none() {
                        menu_item.state = MenuItemState::Disabled;
                    }
                    
                    menu_item
                }),
                MenuItem::Separator,
                MenuItem::String(StringMenuItem::new("Werkzeuge".into())
                .with_children(vec![
                    MenuItem::String(
                        StringMenuItem::new("Grundbuchvergleich".into())
                        .with_children(vec![
                            MenuItem::String(
                                StringMenuItem::new("Alle -> nicht durchgeführt".into())
                                .with_callback(app_data.clone(), gbve_alle_haken_loeschen)
                            ),
                            MenuItem::String(
                                StringMenuItem::new("Alle -> durchgeführt".into())
                                .with_callback(app_data.clone(), gbve_alle_haken_setzen)
                            ),
                        ].into())
                    ),
                    MenuItem::String(
                        StringMenuItem::new("Anteile".into())
                        .with_children(vec![
                            MenuItem::String(StringMenuItem::new("Alle -> 1/1".into()))
                        ].into())
                    ),
                    MenuItem::String(
                        StringMenuItem::new("Adressen".into())
                        .with_children(vec![
                            MenuItem::String(StringMenuItem::new("Feld \"Land\" bereinigen".into()))
                        ].into())
                    ),
                    MenuItem::String(
                        StringMenuItem::new("Fortführungsauftrag aus Datei ausführen...".into())
                        .with_callback(app_data.clone(), ffa_aus_datei_ausfuehren)
                    ),
                ].into()))

            ].into())))
        }

        menu
    }.into()))
    .with_inline_css_props(CSS_MATCH_13650487208571438689)
    .with_children(DomVec::from_vec(vec![
        render_verfahrensuebersicht(
            app_data.clone(), 
            &data.verfahren_filter,
            &data.geladene_verfahren.verfahren, 
            &data.ausgewaehltes_verfahren
        ),
        match data.ausgewaehltes_verfahren.as_ref().and_then(|s| data.geladene_verfahren.verfahren.iter().find(|v| v.uuid.as_str() == s.as_str())) {
            Some(s) => render_verfahren_info(app_data.clone(), data),
            None => div::render()
        }    
    ]))
}

extern "C"
fn ffa_aus_datei_ausfuehren(app_data: &mut RefAny, info: &mut CallbackInfo) -> Update {
    
    use azul::dialog::{MsgBox, FileDialog};

    let data_clone = app_data.clone();
    let data = match app_data.downcast_ref::<AppData>() {
        Some(s) => s,
        None => return Update::DoNothing,
    };
    
    let data = &*data;

    let verfahren = match data.ausgewaehltes_verfahren
        .as_ref()
        .and_then(|s| data.geladene_verfahren.verfahren.iter().find(|v| v.uuid.as_str() == s.as_str())) {
        Some(s) => s,
        None => { return Update::DoNothing; },
    };
    
    let dateipfad = match FileDialog::select_file("Fortführungsauftrag laden".into(), None.into(), None.into()).into_option() {
        Some(s) => s,
        None => return Update::DoNothing,    
    };
    let mut dateipfad = dateipfad.as_str().to_string();

    let datei = match std::fs::read_to_string(&dateipfad) {
        Ok(o) => o,
        Err(e) => {
            MsgBox::error(format!("Fehler beim Laden der Datei: {}", e).into());
            return Update::DoNothing;
        }
    };

    let init_data = RefAny::new(XmlBackgroundThreadInit {
        konfiguration: data.konfiguration.clone(),
        verfahren_uuid: verfahren.uuid.clone(),
        xml: datei.trim().to_string(),
    });

    let _ = match info.start_thread(init_data, data_clone, xml_ffa_background_thread).into_option() {
        Some(s) => s,
        None => return Update::DoNothing, // thread creation failed
    };

    Update::RefreshDom
}

extern "C"
fn gbve_alle_haken_loeschen(app_data: &mut RefAny, info: &mut CallbackInfo) -> Update {
    
    use azul::dialog::FileDialog;

    let data_clone = app_data.clone();
    let data = match app_data.downcast_ref::<AppData>() {
        Some(s) => s,
        None => return Update::DoNothing,
    };
    
    let data = &*data;
    
    let verfahren = match data.ausgewaehltes_verfahren
        .as_ref()
        .and_then(|s| data.geladene_verfahren.verfahren.iter().find(|v| v.uuid.as_str() == s.as_str())) {
        Some(s) => s,
        None => { return Update::DoNothing; },
    };

    let xml = get_verfahren_grundbuchhaken(&verfahren, false).trim().to_string();

    let datei_pfad = match FileDialog::save_file("Fortführungsauftrag speichern unter".into(), None.into()).into_option() {
        Some(s) => s,
        None => { return Update::DoNothing; },
    };
    let datei_pfad = datei_pfad.as_str().to_string();

    let _ = std::fs::write(&datei_pfad, &xml.as_bytes());

    Update::RefreshDom
}

fn get_verfahren_grundbuchhaken(verfahren: &VerfahrenGeladen, haken_setzen: bool) -> String {
    
    use crate::wsdl::KennzeichnungAlterNeuerBestand;
    use chrono::SecondsFormat;

    let mut xml = String::new();

    for buchungsblatt in verfahren.buchungsblatt_bodenordnung.iter()
        .filter(|gb| !gb.nebenbeteiligten_blatt)
        .filter(|gb| gb.ax_buchungsblatt.blt == 1000)
        .filter(|gb| if haken_setzen {
            !gb.grundbuchvergleich_durchgefuehrt
        } else {
            gb.grundbuchvergleich_durchgefuehrt
        })
        .filter(|gb| gb.kan == KennzeichnungAlterNeuerBestand::AlterBestand) {

        xml.push_str(&format!("
            <wfsext:Replace vendorId=\"AdV\" safeToIgnore=\"false\">
              <lefis:LX_BuchungsblattBodenordnung gml:id=\"{uuid}\">
                <gml:identifier codeSpace=\"http://www.adv-online.de/\">urn:adv:oid:{uuid}</gml:identifier>
                <lebenszeitintervall>
                  <AA_Lebenszeitintervall>
                    <beginnt>{beginnt_datum}</beginnt>
                  </AA_Lebenszeitintervall>
                </lebenszeitintervall>
                <modellart>
                  <AA_Modellart>
                    <sonstigesModell>LEFIS</sonstigesModell>
                  </AA_Modellart>
                </modellart>
                <lefis:kan>A</lefis:kan>
                <lefis:gehoertZuVerfahren xlink:href=\"urn:adv:oid:{verfahren_uuid}\" />
                <lefis:unterliegtDemNachtrag>false</lefis:unterliegtDemNachtrag>
                <lefis:unterliegtEinerPlantextziffer>true</lefis:unterliegtEinerPlantextziffer>
                <lefis:kopierVorgangErfolgt>false</lefis:kopierVorgangErfolgt>
                <lefis:ergaenzt xlink:href=\"urn:adv:oid:{ax_buchungsblatt_uuid}\" />
                <lefis:nebenbeteiligtenBlatt>false</lefis:nebenbeteiligtenBlatt>
                <lefis:grundbuchvergleichDurchgefuehrt>{haken_setzen}</lefis:grundbuchvergleichDurchgefuehrt>
                <lefis:vollmigriertesGrundbuchblatt>false</lefis:vollmigriertesGrundbuchblatt>
              </lefis:LX_BuchungsblattBodenordnung>
              <ogc:Filter>
                <ogc:FeatureId fid=\"{uuid}{feature_beg}\" />
              </ogc:Filter>
            </wfsext:Replace>
        ",
            uuid = buchungsblatt.uuid,
            feature_beg = buchungsblatt.beg.format("%Y%m%dT%H%M%SZ"),
            beginnt_datum = buchungsblatt.beg.to_rfc3339_opts(SecondsFormat::Secs, true),
            verfahren_uuid = verfahren.uuid,
            ax_buchungsblatt_uuid = buchungsblatt.ax_buchungsblatt.uuid,
            haken_setzen = if haken_setzen { "true" } else { "false" },
        ));
    }

    let ffa_xml = format!("
        <?xml version=\"1.0\" encoding=\"utf-8\"?>
        <!--Die NAS-Datei wurde mit der FI-Version 6.4.3.19200 erstellt.-->
        <AX_Fortfuehrungsauftrag xsi:schemaLocation=\"http://www.landentwicklung.de/namespaces/lefis/1.5 NAS-LEFIS-Operationen.xsd http://www.adv-online.de/namespaces/adv/gid/6.0 aaa.xsd\" xmlns=\"http://www.adv-online.de/namespaces/adv/gid/6.0\" xmlns:gsr=\"http://www.isotc211.org/2005/gsr\" xmlns:fc=\"http://www.adv-online.de/namespaces/adv/gid/fc/6.0\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns:gml=\"http://www.opengis.net/gml/3.2\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" xmlns:lefis=\"http://www.landentwicklung.de/namespaces/lefis/1.5\" xmlns:wfsext=\"http://www.adv-online.de/namespaces/adv/gid/wfsext\" xmlns:gco=\"http://www.isotc211.org/2005/gco\" xmlns:xs=\"http://www.w3.org/2001/XMLSchema\" xmlns:gts=\"http://www.isotc211.org/2005/gts\" xmlns:ogc=\"http://www.adv-online.de/namespaces/adv/gid/ogc\" xmlns:wfs=\"http://www.adv-online.de/namespaces/adv/gid/wfs\" xmlns:gmd=\"http://www.isotc211.org/2005/gmd\" xmlns:gss=\"http://www.isotc211.org/2005/gss\">
              <empfaenger>
                <AA_Empfaenger>
                  <direkt>true</direkt>
                </AA_Empfaenger>
              </empfaenger>
              <ausgabeform>application/gzip</ausgabeform>
              <koordinatenangaben>
                <AA_Koordinatenreferenzsystemangaben>
                  <crs xlink:href=\"urn:adv:crs:ETRS89_UTM33\" />
                  <anzahlDerNachkommastellen>3</anzahlDerNachkommastellen>
                  <standard>true</standard>
                </AA_Koordinatenreferenzsystemangaben>
              </koordinatenangaben>
              <geaenderteObjekte>
                <wfs:Transaction version=\"1.0.0\" service=\"WFS\">
                  {replace}
                </wfs:Transaction>
              </geaenderteObjekte>
              <profilkennung>AED Sicad</profilkennung>
              <antragsnummer>LefisUpload-{auftragsnummer}-{antragsnummer}</antragsnummer>
              <auftragsnummer>{auftragsnummer}</auftragsnummer>
              <impliziteLoeschungDerReservierung>4000</impliziteLoeschungDerReservierung>
              <verarbeitungsart>1000</verarbeitungsart>
              <geometriebehandlung>false</geometriebehandlung>
              <mitTemporaeremArbeitsbereich>false</mitTemporaeremArbeitsbereich>
              <mitObjektenImFortfuehrungsgebiet>false</mitObjektenImFortfuehrungsgebiet>
              <mitFortfuehrungsnachweis>false</mitFortfuehrungsnachweis>
          </AX_Fortfuehrungsauftrag>
    ", 
        replace = xml,
        antragsnummer = verfahren.name, // Wilmersdorf-Weesow_...
        auftragsnummer = format!("{}_0099", verfahren.nummer)
    );

    ffa_xml
}

extern "C"
fn gbve_alle_haken_setzen(app_data: &mut RefAny, info: &mut CallbackInfo) -> Update {
    
    use azul::dialog::FileDialog;

    let data_clone = app_data.clone();
    let data = match app_data.downcast_ref::<AppData>() {
        Some(s) => s,
        None => return Update::DoNothing,
    };
    
    let data = &*data;
    
    let verfahren = match data.ausgewaehltes_verfahren
        .as_ref()
        .and_then(|s| data.geladene_verfahren.verfahren.iter().find(|v| v.uuid.as_str() == s.as_str())) {
        Some(s) => s,
        None => { return Update::DoNothing; },
    };

    let xml = get_verfahren_grundbuchhaken(&verfahren, true).trim().to_string();

    let datei_pfad = match FileDialog::save_file("Fortführungsauftrag speichern unter".into(), None.into()).into_option() {
        Some(s) => s,
        None => { return Update::DoNothing; },
    };
    let datei_pfad = datei_pfad.as_str().to_string();

    let _ = std::fs::write(&datei_pfad, &xml.as_bytes());

    Update::RefreshDom
}

extern "C"
fn verfahren_neu_laden(app_data: &mut RefAny, _: &mut CallbackInfo) -> Update {
    
    use azul::dialog::MsgBox;
    use crate::{LefisInfo, DhkVerbindung};

    let mut data_mut = match app_data.downcast_mut::<AppData>() {
        Some(s) => s,
        None => return Update::DoNothing,
    };
    
    let mut data_mut = &mut *data_mut;

    let (konfiguration, db) = match LefisUploadKonfiguration::neu_laden() {
        Ok(o) => {
            let lefis_info = match LefisInfo::new(&o.lefis) {
                Ok(o) => Some(o),
                Err(e) => {
                    MsgBox::error(format!("Fehler beim Auslesen der Oracle-Zugangsdaten: {}", e).into());
                    return Update::DoNothing;
                },
            };

            (o, lefis_info)
        },
        Err(e) => { 
            MsgBox::error(e.clone().into()); 
            return Update::DoNothing;
        },
    };

    let dhk_verbindung = match db.as_ref() {
        Some(s) => match DhkVerbindung::new(&s.oracle) {
            Ok(o) => Some(o),
            Err(e) => {
                MsgBox::error(format!("Fehler beim Verbinden mit Oracle-Datenbank: {:#?} - {}", s.oracle, e).into());
                return Update::DoNothing;
            }
        },
        None => None,
    };

    let geladene_verfahren = match dhk_verbindung.as_ref().and_then(|dhk| match dhk.lade_verfahren() {
        Ok(o) => Some(o),
        Err(e) => {
            MsgBox::error(format!("Fehler beim Laden der Verfahren: {}", e).into());
            None
        }
    }) {
        Some(s) => s,
        None => return Update::DoNothing,
    };

    data_mut.konfiguration = konfiguration;
    data_mut.lefis_info = db;
    data_mut.dhk_verbindung = dhk_verbindung;
    data_mut.geladene_verfahren = geladene_verfahren;
    
    MsgBox::info("OK - Verfahren wurden neu geladen.".into());

    return Update::RefreshDomAllWindows;
}