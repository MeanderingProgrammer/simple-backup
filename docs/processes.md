## 1. Getting Backup Ahead Locally

* Delete state: `rm data/.state.bin`
* Clear `file-backup` directory
* Build, run sync, should get everything up to date
* Save `.state.bin` file in `file-backup` to some temporary location
* Update `file-local/file.txt` with some text
* Run sync, should get everything up to date
* Overwrite `.state.bin` file in `file-backup` with the one we oreviously saved
* Comment out `final_state.save` & `directory.backup_config.save_backup_state` to be extra careful
* Build, run sync, should now be in scenario where `backup` is the only different state
