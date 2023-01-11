## 1) Figure out how to store 'state' of a directory

We'll eventually need to know what was the state of a directory last we looked at it.

This is so we can later compare it to future scans to determine when something changed.

This will then allow us to find `new`, `modified`, & `deleted` files and take appropriate actions.

## 2) Create a framework for handling changes

Immediately there are 2 things we would like to be able to do when files change.

1. Upload them to S3
2. Copy them to another location (likely an external hard drive)

Which action we take will likely depend on some concept of `user profile`.

## 3) Figure out the User Profile

Will likely consist of:

* Which directories we should be paying attention to
* Where to backup these directories

Along with some either metadata, perhaps a sync schedule and credentials.

We'll need to store this information across executions, likely a similar mechanism to directory state.

## 4) Create a simple UI

Want some simple UI to be able to add directories to be tracked, and any other configurations.

## 5) Restore Framework

If an existing User wants to restore state and has all the needed devices, i.e. AWS credentials / external hard drive,
we should be able to restore the local system and start tracking from there.

The main use case from this is getting a new computer, so we assume this does not happen frequently.

Ideally we could also use this to keep multiple systems in sync, but this will require some notion of drift detection.

## 6) Drift Detection / Resolution

If the user wants to essentially use this to keep two systems in sync, say by relying completely on AWS,
we should be able to do periodic partial restores by looking at the state of the current backup, and the
last one that was successfully performed by the current machine.

Will be very similar to using the state of directories to figure out when to do a backup, but inversed and
based on the state of the backup.
