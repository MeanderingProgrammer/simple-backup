## 1. Getting Backup Ahead Locally

* Update `local/folder-1/file-1.txt` with some more text
* Comment out saving `final_state` as part of `sync`
* Build and run sync, will put us in `scenario d` as `global` and `current` will match (this should never happen in prod)
* Modify the same local file again with some text
* Comment out the call to `copy_file` & `save_global_state`
* Add back the call to save `final_state` as part of `sync`
* Build and run sync, will put us in `scenario c` as `previous` and `current` will match
* Again comment out the call to `final_state` as part of `sync`
* Now we can experiment with how current state will change based on our solution to `scenario c`
