use backoff_macros::retry;

extern crate backoff;
use backoff::ExponentialBackoff;

// #[retryyy(ExponentialBackoff)]
// pub fn do_it() -> Result<(), backoff::Error<&'static str>> {
//     println!("trying");
//     Err(backoff::Error::Permanent("address not found"))
//     //Err(backoff::Error::Transient("next server"))
// }

#[retry(ExponentialBackoff)]
fn count_to_5(num: &mut u32) -> Result<(), backoff::Error<&'static str>> {
    println!("Trying A");

    // If not yet 5, add one an fail try again
    match num {
        0..=4 => {
            *num += 1;
            Err(backoff::Error::Transient("not yet 5"))
        }
        5.. => Ok(()),
    }
}

#[test]
fn retry_to_success() {
    let mut counter: u32 = 2;
    let counter_ref = &mut counter;
    count_to_5(counter_ref).unwrap()
}

#[retry(ExponentialBackoff)]
fn count_to_failure(num: &mut u32) -> Result<(), backoff::Error<&'static str>> {
    println!("Trying B");

    // If not yet 5, add one an fail try again
    match num {
        0..=4 => {
            *num += 1;
            Err(backoff::Error::Transient("not yet 5"))
        }
        5.. => Err(backoff::Error::Permanent("give up")),
    }
}

#[test]
fn retry_to_failure() {
    let mut counter: u32 = 2;
    let counter_ref = &mut counter;
    assert!(count_to_failure(counter_ref).is_err())
}
