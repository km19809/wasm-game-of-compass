#[cfg(not(feature = "Korean"))]
#[macro_export]
macro_rules! TEMPLATE_TEXT_GIVEN_UP {
    () => {
        "{} (Given Up)"
    };
}
#[cfg(not(feature = "Korean"))]
#[macro_export]
macro_rules! TEMPLATE_TEXT_WINNER {
    () => {
        "The winner is {}!"
    };
}
#[cfg(not(feature = "Korean"))]
#[macro_export]
macro_rules! TEMPLATE_TEXT_RANKING {
    () => {
        "{}. {} - {}pt."
    };
}
#[cfg(feature = "Korean")]
#[macro_export]
macro_rules! TEMPLATE_TEXT_GIVEN_UP {
    () => {
        "{} (포기)"
    };
}
#[cfg(feature = "Korean")]
#[macro_export]
macro_rules! TEMPLATE_TEXT_WINNER {
    () => {
        "우승자는 {}입니다!"
    };
}
#[cfg(feature = "Korean")]
#[macro_export]
macro_rules! TEMPLATE_TEXT_RANKING {
    () => {
        "{}. {} - {}점"
    };
}
