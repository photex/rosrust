use rosrust;
use std::process::Command;
use std::time;

mod util;

mod msg {
    rosrust::rosmsg_include!(roscpp_tutorials / TwoInts);
}

fn test_request(client: &rosrust::Client<msg::roscpp_tutorials::TwoInts>, a: i64, b: i64) {
    let sum = client
        .req(&msg::roscpp_tutorials::TwoIntsReq { a, b })
        .unwrap()
        .unwrap()
        .sum;
    assert_eq!(a + b, sum);
}

#[test]
fn client_to_roscpp_service() {
    let _roscore = util::run_roscore(11312);
    let _service = util::ChildProcessTerminator::spawn(
        Command::new("rosrun")
            .arg("roscpp_tutorials")
            .arg("add_two_ints_server"),
    );

    rosrust::init("add_two_ints_client");
    rosrust::wait_for_service("add_two_ints", Some(time::Duration::from_secs(10))).unwrap();
    let client = rosrust::client::<msg::roscpp_tutorials::TwoInts>("add_two_ints").unwrap();

    test_request(&client, 0, 10);
    test_request(&client, 10, 0);
    test_request(&client, 100, -200);
}
