use crate::PenareParams;
use std::sync::Arc;
use nih_plug::prelude::*;
use nih_plug_vizia::{
    vizia::prelude::*,
    widgets::*,
    ViziaState,
    assets,
    create_vizia_editor,
};

#[derive(Lens)]
struct Data {
    params: Arc<PenareParams>,
}

impl Model for Data {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (400, 550))
}

pub(crate) fn create(
    params: Arc<PenareParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, nih_plug_vizia::ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        Data {
            params: params.clone(),
        }.build(cx);

        PopupData::default().build(cx);

        ResizeHandle::new(cx);

        VStack::new(cx, |cx| {
            // TODO: Implement waveform display

            ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                macro_rules! hstack {
                    ($cx:ident, $f:expr) => {
                        HStack::new($cx, $f)
                        .child_top(Stretch(1.0))
                        .child_bottom(Stretch(1.0))
                        .col_between(Pixels(10.0))
                        .left(Pixels(10.0))
                    };
                }
                macro_rules! slider {
                    ($cx:ident, $label:expr, $param:ident) => {
                        hstack!($cx, |cx| {
                            ParamSlider::new(cx, Data::params, |p| &p.$param);
                            Label::new(cx, $label);
                        })
                    };
                }
                macro_rules! button {
                    ($cx:ident, $label:expr, $param:ident) => {
                        hstack!($cx, |cx| {
                            ParamButton::new(cx, Data::params, |p| &p.$param);
                            Label::new(cx, $label);
                        })
                    };
                }
                macro_rules! label {
                    ($cx:ident, $label:expr) => {
                        HStack::new($cx, |cx| {
                            Label::new(cx, $label)
                            .font_size(24.0);
                        })
                        .child_space(Stretch(1.0));
                    };
                }

                label!(cx, "Mix");
                slider!(cx, "Mix", mix);
                button!(cx, "Hard Clip Output", output_clip);
                slider!(cx, "Output Clip Threshold", output_clip_threshold);

                label!(cx, "Waveshaper");
                slider!(cx, "Pre Gain", pre_gain);
                slider!(cx, "Function Mix", function_mix);
                slider!(cx, "Function Type", function_type);
                slider!(cx, "Function Parameter", function_param);
                slider!(cx, "Post Gain", post_gain);
                button!(cx, "Flip Phase", flip);

                label!(cx, "Rectifier");
                button!(cx, "Rectify", rectify);
                slider!(cx, "Rectify Mix", rectify_mix);
                slider!(cx, "Rectified Signal Mix In", rectify_mix_in);
                slider!(cx, "Rectify Type", rectify_type);
                button!(cx, "Flip Rectified Signal", rectify_flip);

                label!(cx, "Floorer");
                button!(cx, "Floor", floor);
                slider!(cx, "Floor Mix", floor_mix);
                slider!(cx, "Floor Mix In", floor_mix_in);
                slider!(cx, "Floor Step", floor_step);

                label!(cx, "Filter");
                slider!(cx, "Excess Mix", excess_mix);
                slider!(cx, "Low Pass", low_pass);
                slider!(cx, "Low Pass Q", low_pass_q);
                slider!(cx, "High Pass", high_pass);
                slider!(cx, "High Pass Q", high_pass_q);
                button!(cx, "Excess Signal Bypass", excess_bypass);
            })
            .width(Percentage(100.0));

            Label::new(cx, &format!(
                "{} by {} v{}",
                crate::Penare::NAME,
                crate::Penare::VENDOR,
                crate::Penare::VERSION,
            ))
            .width(Percentage(100.0))
            .height(Pixels(20.0))
            .font_size(12.0)
            .background_color(Color::rgb(200, 200, 200))
            .child_top(Stretch(1.0))
            .child_bottom(Stretch(1.0));
        })
        .row_between(Pixels(0.0))
        .child_left(Stretch(1.0))
        .child_right(Stretch(1.0));
    })
}