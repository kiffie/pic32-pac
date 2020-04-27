#!/usr/bin/python3

"""
pic2irqsources.py

Create a Rust enum of interrupt sources (as opposed to interrupt vectors) for
PIC32 MCUs. The generated enum can be appended to interrupt.rs of the PAC.
"""

import argparse
import xml.etree.ElementTree as ET
import sys

def parseargs():
    parser = argparse.ArgumentParser()
    parser.add_argument("pic", help="Path to EDC/PIC file to load")
    parser.add_argument("-o", "--output", help="path to output file")
    args = parser.parse_args()
    return args


def main():
    args = parseargs()
    pic = args.pic

    xml = ET.parse(pic)

    if args.output:
        out = open(args.output, 'w')
    else:
        out = sys.stdout

    irqsrc = {}
    ns = {'edc': 'http://crownking/edc'}
    intlist = xml.getroot().findall('.//edc:InterruptList/edc:InterruptRequest', ns)
    for i in intlist:
        name = i.attrib['{http://crownking/edc}cname']
        num = i.attrib['{http://crownking/edc}irq']
        desc = i.attrib['{http://crownking/edc}desc']
        if num in irqsrc:
            raise ValueError("number {} not unique".format(num))
        irqsrc[num] = (name, desc)

    print(file=out)
    print('#[doc = r"Enumeration of all the interrupt sources"]', file=out)
    print('#[derive(Copy, Clone, Debug)]', file=out)
    print('#[repr(u8)]', file=out)
    print('pub enum InterruptSource {', file=out)
    for num in irqsrc.keys():
        (name, desc) = irqsrc[num]
        if desc:
            print('    #[doc = "{}"]'.format(desc), file=out)
        print('    {} = {},'.format(name, num), file=out)
    print('}', file=out)

if __name__ == "__main__":
    main()
