// Copyright © 2014 - 2015 by Optimal Computing Limited (of New Zealand)
// This code is licensed under the MIT license (see LICENSE-MIT for details)

#![feature(drain)]

/// This function deals with synchronizing two lists of related types.  The first
/// list is considered the `original` and the second list is the proposed `changed`
/// or updated list.  The function determines when items on the `changed` list have
/// been Inserted, Updated or Deleted, and subsequently runs the `on_insert`, `on_update`
/// or `on_delete` functions.
///
/// The caller must supply the two lists, a context object (which generally is how
/// output occurs), and functions for determining how to match objects (M),
/// how to insert when one is found only in the second list (I), how to update (U)
/// when an object is in both lists, and how to delete (D) when an object is missing
/// from the second list.
pub fn list_sync<A,B,C,M,I,U,D,E>(
    original: &Vec<A>,
    changes: &Vec<B>,
    context: &mut C,
    matches: M,
    on_insert: I,
    on_update: U,
    on_delete: D)
    -> Result<(),E>
    where M: Fn(&A, &B) -> bool,
          I: Fn(&mut C, &B) -> Result<(),E>,
          U: Fn(&mut C, &A, &B) -> Result<(),E>,
          D: Fn(&mut C, &A) -> Result<(),E>
{
    for a in original {
        let mut matched = false;
        for b in changes {
            if matches(a,b) {
                matched = true;
                try!(on_update(context, a,b));
            }
        }
        if ! matched {
            try!(on_delete(context, a));
        }
    }

    for b in changes {
        let mut matched = false;
        for a in original {
            if matches(a,b) {
                matched = true;
                // on_update taken care of in previous loop
            }
        }
        if ! matched {
            try!(on_insert(context, b))
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_list_sync() {
        use ::list_sync;

        let original: Vec<usize> = vec![1, 2, 3, 4];
        let changes: Vec<usize> = vec![1, 2, 5];

        let mut output: Vec<usize> = original.clone();

        let _: Result<(),()> =
            list_sync(&original, &changes, &mut output,
                      |a,b| { *a==*b }, // matches function
                      |c, b| { c.push(*b); Ok(()) }, // insert function
                      |_, _, _| { Ok(()) }, // update function
                      |c, a| { // delete function
                          match c.iter().position(|x| { x==a }) {
                              Some(p) => { c.remove(p); },
                              None => { },
                          }
                          Ok(())
                      });

        assert_eq!( output, vec![1, 2, 5] );
    }
}
