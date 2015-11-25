
# list_sync

The main function list_sync() deals with synchronizing two lists of related types.
The first list is considered the `original` and the second list is the proposed `changed`
or updated list.  The function determines when items on the `changed` list have
been Inserted, Updated or Deleted, and subsequently runs the `on_insert`, `on_update`
or `on_delete` functions.

The caller must supply the two lists, a context object (which generally is how
output occurs), and functions for determining how to match objects (M),
how to insert when one is found only in the second list (I), how to update (U)
when an object is in both lists, and how to delete (D) when an object is missing
from the second list.

A second function vec_categorize() is supplied which divides the elements of a Vec
into two Vecs based upon a given predicate.
