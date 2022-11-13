use headless_chrome::{protocol::cdp::Page, Browser};
use std::{fs, thread};

fn main() -> anyhow::Result<()> {
    let browser = Browser::default()?;
    let tab = browser.wait_for_initial_tab()?;

    tab.navigate_to("http://abehiroshi.la.coocan.jp/")?
        .wait_for_xpath("/html/frameset/frame[1]")?
        .focus()?
        .wait_for_xpath("html/body/table/tbody/tr[10]/td[3]/p/a")?
        .click()?
        .wait_for_xpath("/html/frameset/frame[2]")?
        .focus()?
        .wait_for_xpath("/html/body/table/tbody/tr[8]/td[2]/strong/a")?
        .click()?;

    thread::sleep(std::time::Duration::from_millis(3000));

    let cap_tab =
        tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Jpeg, None, None, true)?;
    fs::write("cap_tab.jpeg", &cap_tab)?;

    Ok(())
}
