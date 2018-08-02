use event::{Event, Mutate, Phrase, Render};
use instrument::oscillator::{Oscillator, StereoWaveform};
use ratios::{Pan, R};
use settings::get_default_app_settings;

pub fn generate_composition() -> StereoWaveform {
    let settings = get_default_app_settings();
    let r = r![
        (25, 4, 0.0, 0.0, 0.0),
        (28, 4, 0.0, 0.0, 0.0),
        (20, 4, 0.0, 0.0, 0.0),
        (17, 4, 0.0, 0.0, 0.0),
        (10, 4, 0.0, 0.0, 0.0),
        (12, 4, 0.0, 0.0, 0.0),
        (7, 4, 0.0, 0.0, 0.0),
        (1, 1, 0.0, 0.0, 0.0),
        (1, 1, 0.0, 0.0, 0.0),
        (5, 8, 0.0, 0.0, 0.0),
        (2, 1, 0.0, 0.0, 0.0),
        (2, 1, 0.0, 0.0, 0.0)
    ];
    let mut oscillator = Oscillator::init(r.clone(), &settings);
    let freq = 230.0;
    let e = Event::new(freq, r.clone(), 1.5, 0.8);
    let phrase1 = Phrase {
        events: vec![
            e.clone().mut_ratios(r![
                (28, 4, 0.0, 0.05, 0.4),
                (28, 4, 0.0, 0.05, 0.7),
                (20, 4, 0.0, 0.2, -0.4),
                (17, 4, 0.0, 0.2, -0.7),
                (10, 4, 0.0, 0.2, 0.5),
                (12, 4, 0.0, 0.2, 0.6),
                (7, 4, 0.0, 0.5, 1.0),
                (1, 1, 0.0, 0.5, -1.0),
                (1, 1, 3.0, 0.5, -1.0),
                (5, 8, 0.0, 0.5, -1.0),
                (2, 1, 0.0, 0.25, 1.0),
                (2, 1, 7.0, 0.25, 1.0),
            ]),
            e.clone().mut_ratios(r![
                (28, 4, 0.0, 0.1, -0.4),
                (28, 4, 11.0, 0.1, -0.7),
                (20, 4, 0.0, 0.2, 0.4),
                (17, 4, 0.0, 0.2, 0.7),
                (10, 4, 0.0, 0.2, -0.2),
                (12, 4, 0.0, 0.2, -0.3),
                (7, 4, 0.0, 0.5, -1.0),
                (1, 1, 0.0, 0.5, 1.0),
                (1, 1, 2.0, 0.5, 1.0),
                (5, 8, 0.0, 0.5, 0.25),
                (2, 1, 0.0, 0.15, -0.75),
                (2, 1, 5.0, 0.15, -0.75),
            ]),
            e.clone().mut_ratios(r![
                (28, 4, 0.0, 0.1, -0.4),
                (28, 4, 9.0, 0.1, -0.7),
                (17, 4, 10.0, 0.2, -0.9),
                (17, 4, 0.0, 0.2, -0.5),
                (10, 4, 0.0, 0.2, 0.2),
                (12, 4, 0.0, 0.2, 0.3),
                (7, 4, 0.0, 0.5, 1.0),
                (1, 1, 0.0, 0.5, -1.0),
                (1, 1, 1.0, 0.5, -1.0),
                (5, 8, 0.0, 0.5, -1.0),
                (2, 1, 0.0, 0.25, 1.0),
                (2, 1, 6.0, 0.25, 1.0)
            ]),
            e.clone().mut_ratios(r![
                (28, 4, 0.0, 0.1, 0.4),
                (28, 4, -10.0, 0.1, 0.7),
                (17, 4, 7.0, 0.2, -0.9),
                (17, 4, 0.0, 0.2, -0.5),
                (10, 4, 0.0, 0.2, -0.1),
                (12, 4, 0.0, 0.2, -0.2),
                (7, 4, 0.0, 0.5, 1.0),
                (1, 1, 0.0, 0.5, -1.0),
                (1, 1, -3.0, 0.5, -1.0),
                (5, 8, 0.0, 0.5, 1.0),
                (2, 1, 0.0, 0.25, -0.75),
                (2, 1, 7.0, 0.25, -0.75),
            ]),
            e.clone().mut_ratios(r![
                (28, 4, 0.0, 0.0, -0.4),
                (28, 4, -8.0, 0.0, -0.7),
                (17, 4, 14.0, 0.25, 1.0),
                (17, 4, 0.0, 0.25, 0.5),
                (10, 4, 0.0, 0.25, 0.1),
                (12, 4, 0.0, 0.25, 0.2),
                (7, 4, 0.0, 0.55, -1.0),
                (1, 1, 0.0, 0.55, 1.0),
                (1, 1, -5.0, 0.55, 1.0),
                (5, 8, 0.0, 0.55, -1.0),
                (2, 1, 0.0, 0.30, 0.75),
                (2, 1, 8.0, 0.30, 0.75)
            ]),
            e.clone().mut_length(3.5, 0.0).mut_ratios(r![
                (30, 4, 11.0, 0.0, 0.4),
                (30, 4, 0.0, 0.0, 0.7),
                (17, 4, 20.0, 0.3, -1.0),
                (17, 4, 0.0, 0.3, -0.9),
                (10, 4, 0.0, 0.3, -0.5),
                (12, 4, 0.0, 0.3, -0.7),
                (7, 4, 0.0, 0.6, 1.0),
                (1, 1, 0.0, 0.6, -1.0),
                (1, 1, -10.0, 0.6, -1.0),
                (5, 8, 0.0, 0.6, 1.0),
                (2, 1, 0.0, 0.45, 1.0),
                (2, 1, 10.0, 0.45, 1.0),
            ]),
        ],
    };

    let end = Phrase {
        events: vec![Event::new(0.0, r.clone(), 3.0, 0.0)],
    };

    vec![
        phrase1.clone(),
        phrase1.clone(),
        phrase1.clone(),
        phrase1.clone(),
        phrase1.clone(),
        phrase1.clone(),
        phrase1.clone(),
        phrase1.clone(),
        phrase1.clone(),
        phrase1.clone(),
        end,
    ].render(&mut oscillator)
}
