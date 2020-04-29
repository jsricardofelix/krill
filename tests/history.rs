extern crate krill;

use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use krill::commons::api::CommandHistoryCriteria;
use krill::commons::util::file;
use krill::commons::util::softsigner::OpenSslSigner;
use krill::daemon::ca::{ta_handle, CaServer};
use krill::daemon::mq::EventQueueListener;
use krill::test::*;

/// This tests regressions for the Command history and details as exposed through the
/// Krill API.
///
/// This uses commands and history as generated by other functional tests defined
/// here.
#[test]
fn history() {
    test_under_tmp(|d| {
        let roas_source =
            PathBuf::from("test-resources/api/regressions/v0_6_0/history/ca_roas/cas");

        let mut cas_test = d.clone();
        cas_test.push("cas");
        file::backup_dir(&roas_source, &cas_test).unwrap();

        let server = {
            let signer = OpenSslSigner::build(&d).unwrap();
            let signer = Arc::new(RwLock::new(signer));

            let event_queue = Arc::new(EventQueueListener::in_mem());

            CaServer::<OpenSslSigner>::build(&d, None, None, event_queue, signer).unwrap()
        };

        let ta = ta_handle();
        let crit_dflt = CommandHistoryCriteria::default();

        let ta_history = server.get_ca_history(&ta, crit_dflt).unwrap();
        let expected_ta_history_json_str = include_str!(
            "../test-resources/api/regressions/v0_6_0/history/ca_roas/expected/ta_history.json"
        );

        let expected_ta_history = serde_json::from_str(expected_ta_history_json_str).unwrap();
        assert_eq!(ta_history, expected_ta_history);

        let ta_repo_add = ta_history.commands().first().unwrap();

        let ta_repo_add_details = server
            .get_ca_command_details(&ta, ta_repo_add.command_key().unwrap())
            .unwrap()
            .unwrap();

        let expected_ta_repo_add_json_str = include_str!("../test-resources/api/regressions/v0_6_0/history/ca_roas/expected/ta_add_repo_cmd_details.json");
        let expected_ta_repo_add = serde_json::from_str(expected_ta_repo_add_json_str).unwrap();

        assert_eq!(ta_repo_add_details, expected_ta_repo_add);

        // let pretty_json = serde_json::to_string_pretty(&ta_history).unwrap();
        // println!("{}", pretty_json);
    })
}
