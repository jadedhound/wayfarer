use leptos::*;

fn format_funds(mut gp: u32) -> (u32, u32, u32) {
    let cp = gp % 10;
    gp /= 10;
    let sp = gp % 100;
    gp /= 100;
    (gp, sp, cp)
}

pub fn funds(sup: u32) -> impl IntoView {
    let (gp, sp, cp) = format_funds(sup);
    format!("{gp}gp {sp}sp {cp}cp")
}

pub fn short_funds<F>(sup: F) -> impl IntoView
where
    F: Fn() -> u32 + 'static,
{
    let (gp, sp, cp) = format_funds(sup());
    [(gp, "gp"), (sp, "sp"), (cp, "cp")]
        .into_iter()
        .filter(|x| x.0 != 0)
        .map(|(num, ico)| format!("{num}{ico}"))
        .fold(String::new(), |mut acc, e| {
            acc.push(' ');
            acc.push_str(&e);
            acc
        })
}
