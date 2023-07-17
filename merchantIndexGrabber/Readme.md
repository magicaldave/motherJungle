# Usage: #

  * Place merchantIndexGrabber into your Data Files folder.

  * Double-Click it. Make sure you are using Mash, or running merchantIndexGrabber inside of MO2. It does not support the OpenMW VFS, and it's not going to.

    merchantIndexGrabber will dump the contents of every restocking merchant's inventory to a file called merchantIndexDatabase.json.

    The output *is no longer* in a format compatible with SkoomaBreath's customMerchantRestock, merchantIndexGrabber requires my forked version to read the external file. You can find it here: [here](https://github.com/magicaldave/motherJungle/releases/tag/merchantIndexGrabber)

  * Place the newly created database file into `Server/data/custom`, and restart the server.

  * ???

  * Profit!
