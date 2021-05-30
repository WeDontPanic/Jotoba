#!/bin/bash

for i in `ls ./locales | grep \.po`;do msgfmt ./locales/$i -o /tmp/${i%.po}.mo;done
mv /tmp/*.mo ./locales
