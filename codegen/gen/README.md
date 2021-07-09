### Getting system definitions

    cp /System/Library/PrivateFrameworks/HomeKitDaemon.framework/Resources/plain-metadata.config system.json
    # remove `LegacyCloud` & `LegacyIDS` entries
    plutil -convert json system.json
