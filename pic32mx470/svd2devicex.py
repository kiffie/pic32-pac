#!/usr/bin/python3

"""
svd2devicex.py

Create a device.x linker script containing symbols for ISR functions for
PIC32 MCUs. The device.x file can be included by a main linker script.
"""

import argparse
import xml.etree.ElementTree as ET
import sys

def parseargs():
    parser = argparse.ArgumentParser()
    parser.add_argument("svd", help="Path to SVD file to load")
    parser.add_argument("-o", "--output", help="path to output file")
    args = parser.parse_args()
    return args


def main():
    args = parseargs()
    svd = args.svd

    svdxml = ET.parse(svd)

    if args.output:
        out = open(args.output, 'w')
    else:
        out = sys.stdout

    intr = svdxml.getroot().findall('.//peripheral[name="INT"]/interrupt')
    irq_vectors = {}
    for i in intr:
        name = i.find('./name').text
        vect = int(i.find('./value').text)
        irq_vectors[vect] = name

    print('/* ISR symbols for MIPS; generated by svd2devicex */\n', file=out)

    for vector in range(0, 64):
        if vector in irq_vectors:
            name = irq_vectors[vector]
            print('/* IRQ vector #{} */'.format(vector), file=out)
            print('PROVIDE({} = _default_isr_fn);'.format(name), file=out)
            print('PROVIDE({}_CONTEXT = _isr_context);'.format(name), file=out)
            print('_vector_{}_fn = {};'.format(vector, name), file=out)
            print('_vector_{}_context = {}_CONTEXT;'.format(vector, name), file=out)
            print('', file=out)
        else:
            print('/* IRQ vector #{} (unused) */'.format(vector), file=out)
            print('PROVIDE(_vector_{}_fn = _default_isr_fn);'.format(vector), file = out)
            print('PROVIDE(_vector_{}_context = _isr_context);'.format(vector), file = out)
            print('', file=out)

if __name__ == "__main__":
    main()