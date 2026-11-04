use dioxus::dioxus_core::DynamicNode;
use dioxus::prelude::*;
use guide::geo::Bearing;
use newtype::Newtype;
use newtype::proptest::strategy::Strategy;

#[derive(Newtype, newtype::Arbitrary, Debug)]
#[newtype(strategy = ArrowDrawing::drawn())]
pub struct ArrowDrawing {
    turned: Bearing,
}

impl ArrowDrawing {
    const FULL_TURN: f64 = 360.0;

    fn drawn() -> impl Strategy<Value = Bearing> {
        (-Self::FULL_TURN..Self::FULL_TURN).prop_map(Bearing::new)
    }
}

impl IntoDynNode for ArrowDrawing {
    fn into_dyn_node(self) -> DynamicNode {
        let turned = self.turned.degrees();
        rsx! {
            svg { view_box: "-20 -20 40 40",
                path { class: "arrow-shape", transform: "rotate({turned:.0})", d: "M0 -17L11 10L0 4L-11 10Z" }
            }
        }
        .into_dyn_node()
    }
}
