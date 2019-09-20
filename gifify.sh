#!/bin/bash

SIZE="1024x1024"
DELAY=10

echo "Resizing to $SIZE ..."

rm -f $1/*.small.png;
for i in $1/*.png; do
    echo " $i"
    gm convert -size $SIZE $i -resize $SIZE $i.small.png
done

echo
echo "Creating slice GIF ..."
gm convert $1/slice*.small.png -delay $DELAY $1/animated-slices.gif

echo
echo "Creating edge GIF ..."
gm convert $1/edge*.small.png -delay $DELAY $1/animated-edges.gif

echo
echo "Done!"
