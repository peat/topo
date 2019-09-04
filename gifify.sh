#!/bin/bash

SIZE="640x640"
DELAY=10

echo "Resizing to $SIZE ..."

rm $1/*.frame.png
for i in $1/*.png; do
    echo " $i"
    gm convert -size $SIZE $i -resize $SIZE $i.frame.png
done

echo
echo "Converting frames to GIF ..."
gm convert $1/*.frame.png -delay $DELAY $1/animated.gif

echo
echo "Done! => $1/animated.gif"
