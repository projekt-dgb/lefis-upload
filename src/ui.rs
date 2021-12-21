//! Auto-generated UI source code

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


const STRING_9801366799028015987: AzString = AzString::from_const_str("Segoe UI");
const STRING_12078601221173113980: AzString = AzString::from_const_str("Segoe UI Bold");
const STRING_16146701490593874959: AzString = AzString::from_const_str("sans-serif");
const STYLE_BACKGROUND_CONTENT_605821317762015437_ITEMS: &[StyleBackgroundContent] = &[
    StyleBackgroundContent::Color(ColorU { r: 239, g: 239, b: 239, a: 255 })
];
const STYLE_BACKGROUND_CONTENT_2353255066510409165_ITEMS: &[StyleBackgroundContent] = &[
    StyleBackgroundContent::Color(ColorU { r: 255, g: 0, b: 0, a: 255 })
];
const STYLE_BACKGROUND_CONTENT_9425389510111643135_ITEMS: &[StyleBackgroundContent] = &[
    StyleBackgroundContent::LinearGradient(LinearGradient {
        direction: Direction::FromTo(DirectionCorners { from: DirectionCorner::Top, to: DirectionCorner::Bottom }),
        extend_mode: ExtendMode::Clamp,
        stops: NormalizedLinearColorStopVec::from_const_slice(LINEAR_COLOR_STOP_14019419773578374595_ITEMS),
    })
];
const STYLE_BACKGROUND_CONTENT_12667596745911411196_ITEMS: &[StyleBackgroundContent] = &[
    StyleBackgroundContent::LinearGradient(LinearGradient {
        direction: Direction::FromTo(DirectionCorners { from: DirectionCorner::Top, to: DirectionCorner::Bottom }),
        extend_mode: ExtendMode::Clamp,
        stops: NormalizedLinearColorStopVec::from_const_slice(LINEAR_COLOR_STOP_17879764291917296390_ITEMS),
    })
];
const STYLE_BACKGROUND_CONTENT_13046214508323908618_ITEMS: &[StyleBackgroundContent] = &[
    StyleBackgroundContent::LinearGradient(LinearGradient {
        direction: Direction::FromTo(DirectionCorners { from: DirectionCorner::Top, to: DirectionCorner::Bottom }),
        extend_mode: ExtendMode::Clamp,
        stops: NormalizedLinearColorStopVec::from_const_slice(LINEAR_COLOR_STOP_5390058917785529581_ITEMS),
    })
];
const STYLE_BACKGROUND_CONTENT_14631383723043813572_ITEMS: &[StyleBackgroundContent] = &[
    StyleBackgroundContent::LinearGradient(LinearGradient {
        direction: Direction::FromTo(DirectionCorners { from: DirectionCorner::Top, to: DirectionCorner::Bottom }),
        extend_mode: ExtendMode::Clamp,
        stops: NormalizedLinearColorStopVec::from_const_slice(LINEAR_COLOR_STOP_650287208941589230_ITEMS),
    })
];
const STYLE_BACKGROUND_CONTENT_15133467471352813994_ITEMS: &[StyleBackgroundContent] = &[
    StyleBackgroundContent::LinearGradient(LinearGradient {
        direction: Direction::FromTo(DirectionCorners { from: DirectionCorner::Top, to: DirectionCorner::Bottom }),
        extend_mode: ExtendMode::Clamp,
        stops: NormalizedLinearColorStopVec::from_const_slice(LINEAR_COLOR_STOP_9453686362801359926_ITEMS),
    })
];
const STYLE_BACKGROUND_CONTENT_16418392607769947338_ITEMS: &[StyleBackgroundContent] = &[
    StyleBackgroundContent::Color(ColorU { r: 255, g: 255, b: 255, a: 255 })
];
const STYLE_BACKGROUND_CONTENT_18282593039276133616_ITEMS: &[StyleBackgroundContent] = &[
    StyleBackgroundContent::LinearGradient(LinearGradient {
        direction: Direction::FromTo(DirectionCorners { from: DirectionCorner::Top, to: DirectionCorner::Bottom }),
        extend_mode: ExtendMode::Clamp,
        stops: NormalizedLinearColorStopVec::from_const_slice(LINEAR_COLOR_STOP_1896825754186205459_ITEMS),
    })
];
const STYLE_FONT_FAMILY_4570983119824751714_ITEMS: &[StyleFontFamily] = &[
    StyleFontFamily::System(STRING_12078601221173113980)
];
const STYLE_FONT_FAMILY_10134585962653230168_ITEMS: &[StyleFontFamily] = &[
    StyleFontFamily::System(STRING_16146701490593874959)
];
const STYLE_FONT_FAMILY_10582572158086033001_ITEMS: &[StyleFontFamily] = &[
    StyleFontFamily::System(STRING_9801366799028015987)
];
const LINEAR_COLOR_STOP_650287208941589230_ITEMS: &[NormalizedLinearColorStop] = &[
    NormalizedLinearColorStop { offset: PercentageValue::const_new(0), color: ColorU { r: 114, g: 160, b: 205, a: 255 } },
NormalizedLinearColorStop { offset: PercentageValue::const_new(100), color: ColorU { r: 78, g: 143, b: 207, a: 255 } }
];
const LINEAR_COLOR_STOP_1896825754186205459_ITEMS: &[NormalizedLinearColorStop] = &[
    NormalizedLinearColorStop { offset: PercentageValue::const_new(0), color: ColorU { r: 25, g: 189, b: 158, a: 255 } },
NormalizedLinearColorStop { offset: PercentageValue::const_new(100), color: ColorU { r: 55, g: 128, b: 110, a: 255 } }
];
const LINEAR_COLOR_STOP_5390058917785529581_ITEMS: &[NormalizedLinearColorStop] = &[
    NormalizedLinearColorStop { offset: PercentageValue::const_new(0), color: ColorU { r: 190, g: 190, b: 190, a: 255 } },
NormalizedLinearColorStop { offset: PercentageValue::const_new(100), color: ColorU { r: 216, g: 216, b: 216, a: 255 } }
];
const LINEAR_COLOR_STOP_9453686362801359926_ITEMS: &[NormalizedLinearColorStop] = &[
    NormalizedLinearColorStop { offset: PercentageValue::const_new(0), color: ColorU { r: 252, g: 252, b: 252, a: 255 } },
NormalizedLinearColorStop { offset: PercentageValue::const_new(100), color: ColorU { r: 239, g: 239, b: 239, a: 255 } }
];
const LINEAR_COLOR_STOP_14019419773578374595_ITEMS: &[NormalizedLinearColorStop] = &[
    NormalizedLinearColorStop { offset: PercentageValue::const_new(0), color: ColorU { r: 200, g: 49, b: 46, a: 255 } },
NormalizedLinearColorStop { offset: PercentageValue::const_new(100), color: ColorU { r: 200, g: 49, b: 46, a: 255 } }
];
const LINEAR_COLOR_STOP_17879764291917296390_ITEMS: &[NormalizedLinearColorStop] = &[
    NormalizedLinearColorStop { offset: PercentageValue::const_new(0), color: ColorU { r: 248, g: 248, b: 248, a: 255 } },
NormalizedLinearColorStop { offset: PercentageValue::const_new(100), color: ColorU { r: 220, g: 220, b: 220, a: 255 } }
];

const CSS_MATCH_10644212216580784356_PROPERTIES: &[NodeDataInlineCssProperty] = &[
    // .ausgewaehltes-verfahren-titel
    NodeDataInlineCssProperty::Normal(CssProperty::MaxWidth(LayoutMaxWidthValue::Exact(LayoutMaxWidth { inner: PixelValue::const_px(600) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::FontSize(StyleFontSizeValue::Exact(StyleFontSize { inner: PixelValue::const_px(25) })))
];
const CSS_MATCH_10644212216580784356: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_10644212216580784356_PROPERTIES);    

const CSS_MATCH_11080677877874143473_PROPERTIES: &[NodeDataInlineCssProperty] = &[
    // .verfahrens-info-zeile p
    NodeDataInlineCssProperty::Normal(CssProperty::FlexGrow(LayoutFlexGrowValue::Exact(LayoutFlexGrow { inner: FloatValue::const_new(1) })))
];
const CSS_MATCH_11080677877874143473: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_11080677877874143473_PROPERTIES);    

const CSS_MATCH_11387455194806019534_PROPERTIES: &[NodeDataInlineCssProperty] = &[
    // .verfahren-info
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingRight(LayoutPaddingRightValue::Exact(LayoutPaddingRight { inner: PixelValue::const_px(0) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingLeft(LayoutPaddingLeftValue::Exact(LayoutPaddingLeft { inner: PixelValue::const_px(0) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingBottom(LayoutPaddingBottomValue::Exact(LayoutPaddingBottom { inner: PixelValue::const_px(10) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingTop(LayoutPaddingTopValue::Exact(LayoutPaddingTop { inner: PixelValue::const_px(10) })))
];
const CSS_MATCH_11387455194806019534: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_11387455194806019534_PROPERTIES);    

const CSS_MATCH_11661823484470247691_PROPERTIES: &[NodeDataInlineCssProperty] = &[
    // .status-log-fehler .header
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingBottom(LayoutPaddingBottomValue::Exact(LayoutPaddingBottom { inner: PixelValue::const_px(5) })))
];
const CSS_MATCH_11661823484470247691: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_11661823484470247691_PROPERTIES);    

const CSS_MATCH_12913589432591768438_PROPERTIES: &[NodeDataInlineCssProperty] = &[
    // .status-log-fehler
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingRight(LayoutPaddingRightValue::Exact(LayoutPaddingRight { inner: PixelValue::const_px(10) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingLeft(LayoutPaddingLeftValue::Exact(LayoutPaddingLeft { inner: PixelValue::const_px(10) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingBottom(LayoutPaddingBottomValue::Exact(LayoutPaddingBottom { inner: PixelValue::const_px(10) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingTop(LayoutPaddingTopValue::Exact(LayoutPaddingTop { inner: PixelValue::const_px(10) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::TextColor(StyleTextColorValue::Exact(StyleTextColor { inner: ColorU { r: 255, g: 255, b: 255, a: 255 } }))),
    NodeDataInlineCssProperty::Normal(CssProperty::BackgroundContent(StyleBackgroundContentVecValue::Exact(StyleBackgroundContentVec::from_const_slice(STYLE_BACKGROUND_CONTENT_9425389510111643135_ITEMS))))
];
const CSS_MATCH_12913589432591768438: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_12913589432591768438_PROPERTIES);    

const CSS_MATCH_13021692908862555904_PROPERTIES: &[NodeDataInlineCssProperty] = &[
    // .projekt-ueberschrift
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingRight(LayoutPaddingRightValue::Exact(LayoutPaddingRight { inner: PixelValue::const_px(10) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingLeft(LayoutPaddingLeftValue::Exact(LayoutPaddingLeft { inner: PixelValue::const_px(10) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingBottom(LayoutPaddingBottomValue::Exact(LayoutPaddingBottom { inner: PixelValue::const_px(5) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingTop(LayoutPaddingTopValue::Exact(LayoutPaddingTop { inner: PixelValue::const_px(5) })))
];
const CSS_MATCH_13021692908862555904: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_13021692908862555904_PROPERTIES);    

const CSS_MATCH_1313287635652259770_PROPERTIES: &[NodeDataInlineCssProperty] = &[
    // .verfahrens-info-zeile
    NodeDataInlineCssProperty::Normal(CssProperty::FlexGrow(LayoutFlexGrowValue::Exact(LayoutFlexGrow { inner: FloatValue::const_new(1) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::FlexDirection(LayoutFlexDirectionValue::Exact(LayoutFlexDirection::Row)))
];
const CSS_MATCH_1313287635652259770: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_1313287635652259770_PROPERTIES);    

const CSS_MATCH_13326242258686333006_PROPERTIES: &[NodeDataInlineCssProperty] = &[
    // .projekt-name p
    NodeDataInlineCssProperty::Normal(CssProperty::FontSize(StyleFontSizeValue::Exact(StyleFontSize { inner: PixelValue::const_px(12) })))
];
const CSS_MATCH_13326242258686333006: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_13326242258686333006_PROPERTIES);    

const CSS_MATCH_13650487208571438689_PROPERTIES: &[NodeDataInlineCssProperty] = &[
    // body
    NodeDataInlineCssProperty::Normal(CssProperty::FlexDirection(LayoutFlexDirectionValue::Exact(LayoutFlexDirection::Row))),
    // *
    NodeDataInlineCssProperty::Normal(CssProperty::FontFamily(StyleFontFamilyVecValue::Exact(StyleFontFamilyVec::from_const_slice(STYLE_FONT_FAMILY_10582572158086033001_ITEMS))))
];
const CSS_MATCH_13650487208571438689: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_13650487208571438689_PROPERTIES);    

const CSS_MATCH_1550058906051548789_PROPERTIES: &[NodeDataInlineCssProperty] = &[
    // .status-log-ok
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingRight(LayoutPaddingRightValue::Exact(LayoutPaddingRight { inner: PixelValue::const_px(10) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingLeft(LayoutPaddingLeftValue::Exact(LayoutPaddingLeft { inner: PixelValue::const_px(10) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingBottom(LayoutPaddingBottomValue::Exact(LayoutPaddingBottom { inner: PixelValue::const_px(10) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingTop(LayoutPaddingTopValue::Exact(LayoutPaddingTop { inner: PixelValue::const_px(10) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::TextColor(StyleTextColorValue::Exact(StyleTextColor { inner: ColorU { r: 255, g: 255, b: 255, a: 255 } }))),
    NodeDataInlineCssProperty::Normal(CssProperty::BackgroundContent(StyleBackgroundContentVecValue::Exact(StyleBackgroundContentVec::from_const_slice(STYLE_BACKGROUND_CONTENT_18282593039276133616_ITEMS))))
];
const CSS_MATCH_1550058906051548789: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_1550058906051548789_PROPERTIES);    

const CSS_MATCH_1569823123531431981_PROPERTIES: &[NodeDataInlineCssProperty] = &[
    // .projekt-ueberschrift p
    NodeDataInlineCssProperty::Normal(CssProperty::FontSize(StyleFontSizeValue::Exact(StyleFontSize { inner: PixelValue::const_px(16) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::FontFamily(StyleFontFamilyVecValue::Exact(StyleFontFamilyVec::from_const_slice(STYLE_FONT_FAMILY_4570983119824751714_ITEMS))))
];
const CSS_MATCH_1569823123531431981: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_1569823123531431981_PROPERTIES);    

const CSS_MATCH_16439713609851736080_PROPERTIES: &[NodeDataInlineCssProperty] = &[
    // .status-log
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingRight(LayoutPaddingRightValue::Exact(LayoutPaddingRight { inner: PixelValue::const_px(10) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingLeft(LayoutPaddingLeftValue::Exact(LayoutPaddingLeft { inner: PixelValue::const_px(10) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingBottom(LayoutPaddingBottomValue::Exact(LayoutPaddingBottom { inner: PixelValue::const_px(10) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingTop(LayoutPaddingTopValue::Exact(LayoutPaddingTop { inner: PixelValue::const_px(10) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::TextColor(StyleTextColorValue::Exact(StyleTextColor { inner: ColorU { r: 255, g: 255, b: 255, a: 255 } }))),
    NodeDataInlineCssProperty::Normal(CssProperty::BackgroundContent(StyleBackgroundContentVecValue::Exact(StyleBackgroundContentVec::from_const_slice(STYLE_BACKGROUND_CONTENT_14631383723043813572_ITEMS))))
];
const CSS_MATCH_16439713609851736080: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_16439713609851736080_PROPERTIES);    

const CSS_MATCH_17359577546762513908_PROPERTIES: &[NodeDataInlineCssProperty] = &[
    // .verfahrensuebersicht
    NodeDataInlineCssProperty::Normal(CssProperty::MaxWidth(LayoutMaxWidthValue::Exact(LayoutMaxWidth { inner: PixelValue::const_px(200) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::FlexGrow(LayoutFlexGrowValue::Exact(LayoutFlexGrow { inner: FloatValue::const_new(1) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::BackgroundContent(StyleBackgroundContentVecValue::Exact(StyleBackgroundContentVec::from_const_slice(STYLE_BACKGROUND_CONTENT_605821317762015437_ITEMS))))
];
const CSS_MATCH_17359577546762513908: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_17359577546762513908_PROPERTIES);    

const CSS_MATCH_18307594868656599026_PROPERTIES: &[NodeDataInlineCssProperty] = &[
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
const CSS_MATCH_18307594868656599026: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_18307594868656599026_PROPERTIES);    

const CSS_MATCH_2027421917835617538_PROPERTIES: &[NodeDataInlineCssProperty] = &[
    // .projekt-name:hover
    NodeDataInlineCssProperty::Hover(CssProperty::Cursor(StyleCursorValue::Exact(StyleCursor::Pointer))),
    NodeDataInlineCssProperty::Hover(CssProperty::BackgroundContent(StyleBackgroundContentVecValue::Exact(StyleBackgroundContentVec::from_const_slice(STYLE_BACKGROUND_CONTENT_2353255066510409165_ITEMS)))),
    // .projekt-name
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingRight(LayoutPaddingRightValue::Exact(LayoutPaddingRight { inner: PixelValue::const_px(10) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingLeft(LayoutPaddingLeftValue::Exact(LayoutPaddingLeft { inner: PixelValue::const_px(10) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingBottom(LayoutPaddingBottomValue::Exact(LayoutPaddingBottom { inner: PixelValue::const_px(3) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingTop(LayoutPaddingTopValue::Exact(LayoutPaddingTop { inner: PixelValue::const_px(3) })))
];
const CSS_MATCH_2027421917835617538: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_2027421917835617538_PROPERTIES);    

const CSS_MATCH_2217151026558672233_PROPERTIES: &[NodeDataInlineCssProperty] = &[
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
    NodeDataInlineCssProperty::Normal(CssProperty::Cursor(StyleCursorValue::Exact(StyleCursor::Pointer))),
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
const CSS_MATCH_2217151026558672233: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_2217151026558672233_PROPERTIES);    

const CSS_MATCH_2875502314340155187_PROPERTIES: &[NodeDataInlineCssProperty] = &[
    // .ausgewaehltes-verfahren
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingRight(LayoutPaddingRightValue::Exact(LayoutPaddingRight { inner: PixelValue::const_px(25) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingLeft(LayoutPaddingLeftValue::Exact(LayoutPaddingLeft { inner: PixelValue::const_px(25) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingBottom(LayoutPaddingBottomValue::Exact(LayoutPaddingBottom { inner: PixelValue::const_px(25) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingTop(LayoutPaddingTopValue::Exact(LayoutPaddingTop { inner: PixelValue::const_px(25) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::FlexGrow(LayoutFlexGrowValue::Exact(LayoutFlexGrow { inner: FloatValue::const_new(1) })))
];
const CSS_MATCH_2875502314340155187: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_2875502314340155187_PROPERTIES);    

const CSS_MATCH_6447022794024462679_PROPERTIES: &[NodeDataInlineCssProperty] = &[
    // .test-buttons>*
    NodeDataInlineCssProperty::Normal(CssProperty::MarginRight(LayoutMarginRightValue::Exact(LayoutMarginRight { inner: PixelValue::const_px(10) }))),
    // .test-buttons
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingRight(LayoutPaddingRightValue::Exact(LayoutPaddingRight { inner: PixelValue::const_px(0) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingLeft(LayoutPaddingLeftValue::Exact(LayoutPaddingLeft { inner: PixelValue::const_px(0) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingBottom(LayoutPaddingBottomValue::Exact(LayoutPaddingBottom { inner: PixelValue::const_px(10) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::PaddingTop(LayoutPaddingTopValue::Exact(LayoutPaddingTop { inner: PixelValue::const_px(10) }))),
    NodeDataInlineCssProperty::Normal(CssProperty::FlexDirection(LayoutFlexDirectionValue::Exact(LayoutFlexDirection::Row)))
];
const CSS_MATCH_6447022794024462679: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_6447022794024462679_PROPERTIES);    

const CSS_MATCH_7059827997353143778_PROPERTIES: &[NodeDataInlineCssProperty] = &[
    // .verfahrens-info-zeile p.text-rechts
    NodeDataInlineCssProperty::Normal(CssProperty::TextAlign(StyleTextAlignValue::Exact(StyleTextAlign::Right))),
    // .verfahrens-info-zeile p
    NodeDataInlineCssProperty::Normal(CssProperty::FlexGrow(LayoutFlexGrowValue::Exact(LayoutFlexGrow { inner: FloatValue::const_new(1) })))
];
const CSS_MATCH_7059827997353143778: NodeDataInlineCssPropertyVec = NodeDataInlineCssPropertyVec::from_const_slice(CSS_MATCH_7059827997353143778_PROPERTIES);

pub fn render() -> Dom {
    Dom::body()
        .with_inline_css_props(CSS_MATCH_13650487208571438689)
    .with_children(DomVec::from_vec(vec![
        div::render()
        .with_inline_css_props(CSS_MATCH_17359577546762513908)
        .with_ids_and_classes({
            const IDS_AND_CLASSES_17892264718074089608: &[IdOrClass] = &[
                    Class(AzString::from_const_str("verfahrensuebersicht")),
    
            ];
            IdOrClassVec::from_const_slice(IDS_AND_CLASSES_17892264718074089608)
        })
        .with_children(DomVec::from_vec(vec![
            div::render()
            .with_inline_css_props(CSS_MATCH_13021692908862555904)
            .with_ids_and_classes({
                const IDS_AND_CLASSES_11102932419086932125: &[IdOrClass] = &[
                            Class(AzString::from_const_str("projekt-ueberschrift")),
    
                ];
                IdOrClassVec::from_const_slice(IDS_AND_CLASSES_11102932419086932125)
            })
            .with_children(DomVec::from_vec(vec![
                p::render(AzString::from_const_str("PROJEKTE"))
                .with_inline_css_props(CSS_MATCH_1569823123531431981)
            ])),
            div::render()
            .with_inline_css_props(CSS_MATCH_2027421917835617538)
            .with_ids_and_classes({
                const IDS_AND_CLASSES_4373706879957701343: &[IdOrClass] = &[
                            Class(AzString::from_const_str("projekt-name")),
    
                ];
                IdOrClassVec::from_const_slice(IDS_AND_CLASSES_4373706879957701343)
            })
            .with_children(DomVec::from_vec(vec![
                p::render(AzString::from_const_str("1001 Schenkenberg"))
                .with_inline_css_props(CSS_MATCH_13326242258686333006)
            ])),
            div::render()
            .with_inline_css_props(CSS_MATCH_2027421917835617538)
            .with_ids_and_classes({
                const IDS_AND_CLASSES_4373706879957701343: &[IdOrClass] = &[
                            Class(AzString::from_const_str("projekt-name")),
    
                ];
                IdOrClassVec::from_const_slice(IDS_AND_CLASSES_4373706879957701343)
            })
            .with_children(DomVec::from_vec(vec![
                p::render(AzString::from_const_str("1001 Schenkenberg"))
                .with_inline_css_props(CSS_MATCH_13326242258686333006)
            ])),
            div::render()
            .with_inline_css_props(CSS_MATCH_2027421917835617538)
            .with_ids_and_classes({
                const IDS_AND_CLASSES_4373706879957701343: &[IdOrClass] = &[
                            Class(AzString::from_const_str("projekt-name")),
    
                ];
                IdOrClassVec::from_const_slice(IDS_AND_CLASSES_4373706879957701343)
            })
            .with_children(DomVec::from_vec(vec![
                p::render(AzString::from_const_str("1001 Schenkenberg"))
                .with_inline_css_props(CSS_MATCH_13326242258686333006)
            ])),
            div::render()
            .with_inline_css_props(CSS_MATCH_2027421917835617538)
            .with_ids_and_classes({
                const IDS_AND_CLASSES_4373706879957701343: &[IdOrClass] = &[
                            Class(AzString::from_const_str("projekt-name")),
    
                ];
                IdOrClassVec::from_const_slice(IDS_AND_CLASSES_4373706879957701343)
            })
            .with_children(DomVec::from_vec(vec![
                p::render(AzString::from_const_str("1001 Schenkenberg"))
                .with_inline_css_props(CSS_MATCH_13326242258686333006)
            ])),
            div::render()
            .with_inline_css_props(CSS_MATCH_2027421917835617538)
            .with_ids_and_classes({
                const IDS_AND_CLASSES_4373706879957701343: &[IdOrClass] = &[
                            Class(AzString::from_const_str("projekt-name")),
    
                ];
                IdOrClassVec::from_const_slice(IDS_AND_CLASSES_4373706879957701343)
            })
            .with_children(DomVec::from_vec(vec![
                p::render(AzString::from_const_str("1001 Schenkenberg"))
                .with_inline_css_props(CSS_MATCH_13326242258686333006)
            ])),
            div::render()
            .with_inline_css_props(CSS_MATCH_2027421917835617538)
            .with_ids_and_classes({
                const IDS_AND_CLASSES_4373706879957701343: &[IdOrClass] = &[
                            Class(AzString::from_const_str("projekt-name")),
    
                ];
                IdOrClassVec::from_const_slice(IDS_AND_CLASSES_4373706879957701343)
            })
            .with_children(DomVec::from_vec(vec![
                p::render(AzString::from_const_str("1001 Schenkenberg"))
                .with_inline_css_props(CSS_MATCH_13326242258686333006)
            ])),
            div::render()
            .with_inline_css_props(CSS_MATCH_2027421917835617538)
            .with_ids_and_classes({
                const IDS_AND_CLASSES_4373706879957701343: &[IdOrClass] = &[
                            Class(AzString::from_const_str("projekt-name")),
    
                ];
                IdOrClassVec::from_const_slice(IDS_AND_CLASSES_4373706879957701343)
            })
            .with_children(DomVec::from_vec(vec![
                p::render(AzString::from_const_str("1001 Schenkenberg"))
                .with_inline_css_props(CSS_MATCH_13326242258686333006)
            ]))
        ])),
        div::render()
        .with_inline_css_props(CSS_MATCH_2875502314340155187)
        .with_ids_and_classes({
            const IDS_AND_CLASSES_12190625166707343638: &[IdOrClass] = &[
                    Class(AzString::from_const_str("ausgewaehltes-verfahren")),
    
            ];
            IdOrClassVec::from_const_slice(IDS_AND_CLASSES_12190625166707343638)
        })
        .with_children(DomVec::from_vec(vec![
            p::render(AzString::from_const_str("500106 Neurüdnitz-Neuküstrinchen"))
            .with_inline_css_props(CSS_MATCH_10644212216580784356)
            .with_ids_and_classes({
                const IDS_AND_CLASSES_7612915473292556625: &[IdOrClass] = &[
                            Class(AzString::from_const_str("ausgewaehltes-verfahren-titel")),
    
                ];
                IdOrClassVec::from_const_slice(IDS_AND_CLASSES_7612915473292556625)
            }),
            div::render()
            .with_inline_css_props(CSS_MATCH_6447022794024462679)
            .with_ids_and_classes({
                const IDS_AND_CLASSES_3777573455697972450: &[IdOrClass] = &[
                            Class(AzString::from_const_str("test-buttons")),
    
                ];
                IdOrClassVec::from_const_slice(IDS_AND_CLASSES_3777573455697972450)
            })
            .with_children(DomVec::from_vec(vec![
                div::render()
                .with_inline_css_props(CSS_MATCH_2217151026558672233)
                .with_ids_and_classes({
                    const IDS_AND_CLASSES_11301910805331300582: &[IdOrClass] = &[
                                    Class(AzString::from_const_str("__azul-native-button")),
    
                    ];
                    IdOrClassVec::from_const_slice(IDS_AND_CLASSES_11301910805331300582)
                })
                .with_children(DomVec::from_vec(vec![
                    p::render(AzString::from_const_str("Test Abt2. speicher unter"))
                ])),
                div::render()
                .with_inline_css_props(CSS_MATCH_2217151026558672233)
                .with_ids_and_classes({
                    const IDS_AND_CLASSES_11301910805331300582: &[IdOrClass] = &[
                                    Class(AzString::from_const_str("__azul-native-button")),
    
                    ];
                    IdOrClassVec::from_const_slice(IDS_AND_CLASSES_11301910805331300582)
                })
                .with_children(DomVec::from_vec(vec![
                    p::render(AzString::from_const_str("Test Abt3. speichern unter"))
                ]))
            ])),
            div::render()
            .with_inline_css_props(CSS_MATCH_6447022794024462679)
            .with_ids_and_classes({
                const IDS_AND_CLASSES_3777573455697972450: &[IdOrClass] = &[
                            Class(AzString::from_const_str("test-buttons")),
    
                ];
                IdOrClassVec::from_const_slice(IDS_AND_CLASSES_3777573455697972450)
            })
            .with_children(DomVec::from_vec(vec![
                div::render()
                .with_inline_css_props(CSS_MATCH_2217151026558672233)
                .with_ids_and_classes({
                    const IDS_AND_CLASSES_11301910805331300582: &[IdOrClass] = &[
                                    Class(AzString::from_const_str("__azul-native-button")),
    
                    ];
                    IdOrClassVec::from_const_slice(IDS_AND_CLASSES_11301910805331300582)
                })
                .with_children(DomVec::from_vec(vec![
                    p::render(AzString::from_const_str("Test alle Abt2. löschen"))
                ])),
                div::render()
                .with_inline_css_props(CSS_MATCH_2217151026558672233)
                .with_ids_and_classes({
                    const IDS_AND_CLASSES_11301910805331300582: &[IdOrClass] = &[
                                    Class(AzString::from_const_str("__azul-native-button")),
    
                    ];
                    IdOrClassVec::from_const_slice(IDS_AND_CLASSES_11301910805331300582)
                })
                .with_children(DomVec::from_vec(vec![
                    p::render(AzString::from_const_str("Test alle Abt3. löschen"))
                ]))
            ])),
            div::render()
            .with_inline_css_props(CSS_MATCH_6447022794024462679)
            .with_ids_and_classes({
                const IDS_AND_CLASSES_3777573455697972450: &[IdOrClass] = &[
                            Class(AzString::from_const_str("test-buttons")),
    
                ];
                IdOrClassVec::from_const_slice(IDS_AND_CLASSES_3777573455697972450)
            })
            .with_children(DomVec::from_vec(vec![
                div::render()
                .with_inline_css_props(CSS_MATCH_2217151026558672233)
                .with_ids_and_classes({
                    const IDS_AND_CLASSES_11301910805331300582: &[IdOrClass] = &[
                                    Class(AzString::from_const_str("__azul-native-button")),
    
                    ];
                    IdOrClassVec::from_const_slice(IDS_AND_CLASSES_11301910805331300582)
                })
                .with_children(DomVec::from_vec(vec![
                    p::render(AzString::from_const_str(".lefis Datei fortführen"))
                ]))
            ])),
            div::render()
            .with_inline_css_props(CSS_MATCH_11387455194806019534)
            .with_ids_and_classes({
                const IDS_AND_CLASSES_17584478058659276754: &[IdOrClass] = &[
                            Class(AzString::from_const_str("verfahren-info")),
    
                ];
                IdOrClassVec::from_const_slice(IDS_AND_CLASSES_17584478058659276754)
            })
            .with_children(DomVec::from_vec(vec![
                div::render()
                .with_inline_css_props(CSS_MATCH_1313287635652259770)
                .with_ids_and_classes({
                    const IDS_AND_CLASSES_3574993573047581730: &[IdOrClass] = &[
                                    Class(AzString::from_const_str("verfahrens-info-zeile")),
    
                    ];
                    IdOrClassVec::from_const_slice(IDS_AND_CLASSES_3574993573047581730)
                })
                .with_children(DomVec::from_vec(vec![
                    p::render(AzString::from_const_str("Verbindung:"))
                    .with_inline_css_props(CSS_MATCH_11080677877874143473),
                    p::render(AzString::from_const_str("LEFISPM (http://lefispm.vlf.pdm:80/WebSericeTS)"))
                    .with_inline_css_props(CSS_MATCH_7059827997353143778)
                    .with_ids_and_classes({
                        const IDS_AND_CLASSES_14596186059031365806: &[IdOrClass] = &[
                                            Class(AzString::from_const_str("text-rechts")),
    
                        ];
                        IdOrClassVec::from_const_slice(IDS_AND_CLASSES_14596186059031365806)
                    })
                ])),
                div::render()
                .with_inline_css_props(CSS_MATCH_1313287635652259770)
                .with_ids_and_classes({
                    const IDS_AND_CLASSES_3574993573047581730: &[IdOrClass] = &[
                                    Class(AzString::from_const_str("verfahrens-info-zeile")),
    
                    ];
                    IdOrClassVec::from_const_slice(IDS_AND_CLASSES_3574993573047581730)
                })
                .with_children(DomVec::from_vec(vec![
                    p::render(AzString::from_const_str("Ordnungsnummern:"))
                    .with_inline_css_props(CSS_MATCH_11080677877874143473),
                    p::render(AzString::from_const_str("94"))
                    .with_inline_css_props(CSS_MATCH_7059827997353143778)
                    .with_ids_and_classes({
                        const IDS_AND_CLASSES_14596186059031365806: &[IdOrClass] = &[
                                            Class(AzString::from_const_str("text-rechts")),
    
                        ];
                        IdOrClassVec::from_const_slice(IDS_AND_CLASSES_14596186059031365806)
                    })
                ])),
                div::render()
                .with_inline_css_props(CSS_MATCH_1313287635652259770)
                .with_ids_and_classes({
                    const IDS_AND_CLASSES_3574993573047581730: &[IdOrClass] = &[
                                    Class(AzString::from_const_str("verfahrens-info-zeile")),
    
                    ];
                    IdOrClassVec::from_const_slice(IDS_AND_CLASSES_3574993573047581730)
                })
                .with_children(DomVec::from_vec(vec![
                    p::render(AzString::from_const_str("Grundbuchblätter:"))
                    .with_inline_css_props(CSS_MATCH_11080677877874143473),
                    p::render(AzString::from_const_str("100"))
                    .with_inline_css_props(CSS_MATCH_7059827997353143778)
                    .with_ids_and_classes({
                        const IDS_AND_CLASSES_14596186059031365806: &[IdOrClass] = &[
                                            Class(AzString::from_const_str("text-rechts")),
    
                        ];
                        IdOrClassVec::from_const_slice(IDS_AND_CLASSES_14596186059031365806)
                    })
                ]))
            ])),
            div::render()
            .with_inline_css_props(CSS_MATCH_16439713609851736080)
            .with_ids_and_classes({
                const IDS_AND_CLASSES_16141427507998024750: &[IdOrClass] = &[
                            Class(AzString::from_const_str("status-log")),
    
                ];
                IdOrClassVec::from_const_slice(IDS_AND_CLASSES_16141427507998024750)
            })
            .with_children(DomVec::from_vec(vec![
                p::render(AzString::from_const_str("Status: Übermittle Auftrag..."))
            ])),
            div::render()
            .with_inline_css_props(CSS_MATCH_12913589432591768438)
            .with_ids_and_classes({
                const IDS_AND_CLASSES_11219677383442904130: &[IdOrClass] = &[
                            Class(AzString::from_const_str("status-log-fehler")),
    
                ];
                IdOrClassVec::from_const_slice(IDS_AND_CLASSES_11219677383442904130)
            })
            .with_children(DomVec::from_vec(vec![
                div::render()
                .with_inline_css_props(CSS_MATCH_11661823484470247691)
                .with_ids_and_classes({
                    const IDS_AND_CLASSES_5083963822852818370: &[IdOrClass] = &[
                                    Class(AzString::from_const_str("header")),
    
                    ];
                    IdOrClassVec::from_const_slice(IDS_AND_CLASSES_5083963822852818370)
                })
                .with_children(DomVec::from_vec(vec![
                    p::render(AzString::from_const_str("Fehler: Auftrag konnte nicht fortgeführt werden"))
                ])),
                div::render()
                .with_inline_css_props(CSS_MATCH_18307594868656599026)
                .with_ids_and_classes({
                    const IDS_AND_CLASSES_9850780023879010224: &[IdOrClass] = &[
                                    Class(AzString::from_const_str("detail")),
    
                    ];
                    IdOrClassVec::from_const_slice(IDS_AND_CLASSES_9850780023879010224)
                })
                .with_children(DomVec::from_vec(vec![
                    p::render(AzString::from_const_str("blah blah blah"))
                ]))
            ])),
            div::render()
            .with_inline_css_props(CSS_MATCH_1550058906051548789)
            .with_ids_and_classes({
                const IDS_AND_CLASSES_5815670520971901181: &[IdOrClass] = &[
                            Class(AzString::from_const_str("status-log-ok")),
    
                ];
                IdOrClassVec::from_const_slice(IDS_AND_CLASSES_5815670520971901181)
            })
            .with_children(DomVec::from_vec(vec![
                p::render(AzString::from_const_str("Ok: Auftrag wurde fortgeführt"))
            ]))
        ])),
    
    ]))
}
