//! Auto-generated UI source code
#![allow(unused_imports)]

use azul::widgets::{FileInput, FileInputState};
use azul::task::{ThreadSender, ThreadReceiver};
use crate::{LefisUploadKonfiguration, exec_sync};

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


fn render_verfahrensuebersicht(app_data: RefAny, verfahren: &[VerfahrenGeladen], ausgewaehltes_verfahren: Option<usize>) -> Dom {
    
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

            verfahren.iter().enumerate().map(|(vg_num, vg)| {

                let ist_ausgewaehlt = ausgewaehltes_verfahren == Some(vg_num);

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
                    data: app_data.clone()
                }].into());

                if !ist_ausgewaehlt {
                    d.with_inline_hover_style(AzString::from_const_str("background-color: #c1c1c1"))
                } else {
                    d
                }

        }).collect::<Dom>()

    ].into())
}

extern "C"
fn verfahren_auswaehlen(data: &mut RefAny, info: &mut CallbackInfo) -> Update {

    let mut data = match data.downcast_mut::<AppData>() {
        Some(s) => s,
        None => return Update::DoNothing,
    };

    let index = info.get_index_in_parent(info.get_hit_node());

    data.ausgewaehltes_verfahren = Some(index);

    Update::RefreshDom
}

fn render_verfahren_info(app_data: RefAny, data: &AppData, ausgewaehltes_verfahren: &VerfahrenGeladen) -> Dom {

    use crate::Auftragsstatus;
    use azul::widgets::*;

    div::render()
    .with_inline_css_props(CSS_MATCH_2875502314340155187)
    .with_ids_and_classes({
        pub(in super) const IDS_AND_CLASSES_12190625166707343638: &[IdOrClass] = &[
                Class(AzString::from_const_str("ausgewaehltes-verfahren")),
        ];
        IdOrClassVec::from_const_slice(IDS_AND_CLASSES_12190625166707343638)
    })
    .with_children(DomVec::from_vec(vec![
        p::render(format!("{:06} - {}", ausgewaehltes_verfahren.nummer, ausgewaehltes_verfahren.name).into())
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
                .with_on_path_change(app_data.clone(), lefis_datei_laden)
                .dom()
            ].into())
            .with_inline_style("display: flex; flex-grow: 0;".into()),
        ])),

        div::render()
        .with_inline_css_props(CSS_MATCH_11387455194806019534)
        .with_ids_and_classes({
            pub(in super) const IDS_AND_CLASSES_17584478058659276754: &[IdOrClass] = &[
                        Class(AzString::from_const_str("verfahren-info")),

            ];
            IdOrClassVec::from_const_slice(IDS_AND_CLASSES_17584478058659276754)
        })
        .with_children(DomVec::from_vec(vec![
            div::render()
            .with_inline_css_props(CSS_MATCH_1313287635652259770)
            .with_ids_and_classes({
                pub(in super) const IDS_AND_CLASSES_3574993573047581730: &[IdOrClass] = &[
                                Class(AzString::from_const_str("verfahrens-info-zeile")),

                ];
                IdOrClassVec::from_const_slice(IDS_AND_CLASSES_3574993573047581730)
            })
            .with_children(DomVec::from_vec(vec![
                p::render(AzString::from_const_str("Verbindung:"))
                .with_inline_css_props(CSS_MATCH_11080677877874143473),
                p::render(format!("{} ({})", ausgewaehltes_verfahren.dhk_verbindung, data.konfiguration.lefis.webservice_url).into())
                .with_inline_css_props(CSS_MATCH_7059827997353143778)
                .with_ids_and_classes({
                    pub(in super) const IDS_AND_CLASSES_14596186059031365806: &[IdOrClass] = &[
                                        Class(AzString::from_const_str("text-rechts")),

                    ];
                    IdOrClassVec::from_const_slice(IDS_AND_CLASSES_14596186059031365806)
                })
            ])),
        ])),

        div::render()
        .with_children(vec![
            TabHeader::new(vec![
                format!("Grundbuchblätter ({})", ausgewaehltes_verfahren.grundbuchblaetter.len()),
                format!("Abt. 2 Rechte ({})", ausgewaehltes_verfahren.abt2_rechte.len()),
                format!("Abt. 3 Rechte ({})", ausgewaehltes_verfahren.abt3_rechte.len()),
            ].into())
            .dom(),
        ].into()),

        div::render().with_children(vec![
            match &ausgewaehltes_verfahren.lefis_geladen {
                None => match &ausgewaehltes_verfahren.auftragsstatus {
                    None => div::render(),
                    Some(Auftragsstatus::AuftragWirdFortgefuehrt { prozent }) =>  {
                        div::render()
                        .with_inline_css_props(CSS_MATCH_16439713609851736080)
                        .with_ids_and_classes({
                            pub(in super) const IDS_AND_CLASSES_16141427507998024750: &[IdOrClass] = &[
                                        Class(AzString::from_const_str("status-log")),

                            ];
                            IdOrClassVec::from_const_slice(IDS_AND_CLASSES_16141427507998024750)
                        })
                        .with_children(DomVec::from_vec(vec![
                            p::render(AzString::from_const_str("Auftrag wird fortgeführt..."))
                        ]))
                    },
                    Some(Auftragsstatus::Fehler { text }) => {

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
                            .with_ids_and_classes({
                                pub(in super) const IDS_AND_CLASSES_5083963822852818370: &[IdOrClass] = &[
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
                                pub(in super) const IDS_AND_CLASSES_9850780023879010224: &[IdOrClass] = &[
                                                Class(AzString::from_const_str("detail")),

                                ];
                                IdOrClassVec::from_const_slice(IDS_AND_CLASSES_9850780023879010224)
                            })
                            .with_children(DomVec::from_vec(vec![
                                p::render(text.clone().into())
                            ]))
                        ]))

                    },
                    Some(Auftragsstatus::ErfolgreichFortgefuehrt) => {  
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
                    }
                },
                Some(v) => {
                    div::render()
                    .with_inline_css_props(CSS_MATCH_1550058906051548789)
                    .with_children(DomVec::from_vec(vec![
                        div::render().with_children(vec![
                            p::render(format!("Ok: {} Grundbuchblätter bereit für Fortführung in DHK", v.len()).into()),
                        ].into())
                        .with_inline_style("display: flex; flex-grow: 1;".into()),
                        div::render().with_children(vec![
                            Button::new("Fortführen".into())
                            .with_on_click(app_data.clone(), lefis_datei_fortfuehren)
                            .dom(),
                        ].into())
                        .with_inline_style("display: flex; flex-grow: 0;".into()),
                    ]))
                }
            },
        ].into())
    ]))
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

    let aktives_verfahren = match data_mut.ausgewaehltes_verfahren.clone().and_then(|d| data_mut.geladene_verfahren.get_mut(d)) {
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

    let aktives_verfahren = match data_mut.ausgewaehltes_verfahren.clone().and_then(|d| data_mut.geladene_verfahren.get_mut(d)) {
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
        verfahren: aktives_verfahren.clone()
    });

    let thread_id = match info.start_thread(init_data, data_clone, ffa_background_thread).into_option() {
        Some(s) => s,
        None => return Update::DoNothing, // thread creation failed
    };

    aktives_verfahren.lefis_geladen = None;

    Update::DoNothing
}

#[derive(Debug)]
struct BackgroundThreadInit {
    konfiguration: LefisUploadKonfiguration,
    verfahren: VerfahrenGeladen,
}

#[derive(Debug, Clone)]
enum BackgroundThreadReturn {
    Fortgefuehrt { verfahrens_uuid: String },
    FortschrittUpdate { verfahrens_uuid: String, prozent: usize },
    Fehler { verfahrens_uuid: String, text: String, },
}

extern "C" fn ffa_background_thread(
    mut initial_data: RefAny,
    mut sender: ThreadSender,
    mut recv: ThreadReceiver,
) {
    use crate::wsdl::{AuftragsManager, RequestFailure};
    use azul::task::*;
    use azul::callbacks::WriteBackCallback;

    let initial_data = match initial_data.downcast_ref::<BackgroundThreadInit>() {
        Some(s) => s,
        None => return, // error
    };

    let am = AuftragsManager::new(
        &initial_data.konfiguration.lefis.webservice_url, 
        initial_data.konfiguration.lefis.benutzer.clone(), 
        initial_data.konfiguration.lefis.passwort.clone()
    );
    
    sender.send(ThreadReceiveMsg::WriteBack(ThreadWriteBackMsg {
        data: RefAny::new(BackgroundThreadReturn::FortschrittUpdate { 
            verfahrens_uuid: initial_data.verfahren.uuid.clone(), 
            prozent: 0,
        }),
        callback: WriteBackCallback { cb: writeback_callback }
    }));

    let e: Result<(), RequestFailure> = exec_sync(async {
        let version = am.get_version().await?;
        let login = am.login().await?;
        am.logout(login.session_id).await?;
        Ok(())
    });

    let data = match e {
        Err(e) => {
            BackgroundThreadReturn::Fehler { 
                verfahrens_uuid: initial_data.verfahren.uuid.clone(), 
                text: format!("{:?}", e),
            }
        },
        Ok(_) => {
            BackgroundThreadReturn::Fortgefuehrt { 
                verfahrens_uuid: initial_data.verfahren.uuid.clone(), 
            }  
        }
    };

    sender.send(ThreadReceiveMsg::WriteBack(ThreadWriteBackMsg {
        data: RefAny::new(data),
        callback: WriteBackCallback { cb: writeback_callback }
    }));
}

extern "C" fn writeback_callback(app_data: &mut RefAny, incoming_data: &mut RefAny, _: &mut CallbackInfo) -> Update {

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

    let mut aktives_verfahren = match data_mut.geladene_verfahren.iter_mut().find(|d| d.uuid == verfahrens_uuid) {
        Some(s) => s,
        None => return Update::DoNothing,
    };

    aktives_verfahren.auftragsstatus = match &mut *incoming_data {
        Fortgefuehrt { .. } => Some(Auftragsstatus::ErfolgreichFortgefuehrt),
        FortschrittUpdate { prozent, .. } => Some(Auftragsstatus::AuftragWirdFortgefuehrt { prozent: *prozent }),
        Fehler { text, .. } => Some(Auftragsstatus::Fehler { text: text.clone() }),
    };

    Update::RefreshDom
}


pub fn render(app_data: RefAny, data: &AppData) -> Dom {
    use azul::menu::*;

    Dom::body()
    .with_menu_bar(Menu::new({

        let mut menu = vec![
            MenuItem::String(StringMenuItem::new("Verfahren".into()).with_children(vec![
                MenuItem::String(StringMenuItem::new("Liste neu laden".into()))
            ].into()))
        ];

        if data.ausgewaehltes_verfahren.is_some() {
            menu.push(MenuItem::String(StringMenuItem::new("Ausgewähltes Verfahren".into()).with_children(vec![
                MenuItem::String(StringMenuItem::new("Neu laden".into()))
            ].into())))
        }

        menu
    }.into()))
    .with_inline_css_props(CSS_MATCH_13650487208571438689)
    .with_children(DomVec::from_vec(vec![
        render_verfahrensuebersicht(app_data.clone(), &data.geladene_verfahren, data.ausgewaehltes_verfahren),
        match data.ausgewaehltes_verfahren.and_then(|s| data.geladene_verfahren.get(s)) {
            Some(s) => render_verfahren_info(app_data.clone(), data, s),
            None => div::render()
        }    
    ]))
}
