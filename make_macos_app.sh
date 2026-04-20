#!/bin/bash
rm -rf reBOrN.app
rm reBOrN_deobfuscator.dmg

mkdir -p reBOrN.app/Contents/MacOS
mkdir -p reBOrN.app/Contents/Resources

cp ./resources/Info.plist.template ./reBOrN.app/Contents/Info.plist
cp ./resources/icon.icns ./reBOrN.app/Contents/Resources/icon.icns

if [ -z "$CI" ]; then
  echo "Creating macOS app locally"
  cp ./target/release/reborn ./reBOrN.app/Contents/MacOS/reborn
else
  echo "Creating macOS app in Github runner"
  cp ./target/${{ matrix.platform.target }}/release/reborn ./reBOrN.app/Contents/MacOS/reborn
fi

hdiutil create -fs HFS+ -volname "reBOrN deobfuscator" -srcfolder reBOrN.app "reBOrN_deobfuscator.dmg"