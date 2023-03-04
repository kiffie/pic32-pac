#!/bin/bash

build_module() {
        local dir=$1
        local target_name=$2
        local EDC=$target_name.PIC
        local SVD=$target_name.svd

        pushd $dir || exit -1
        edc2svd $EDC $SVD
        ../svdpatch.py svdpatch.yaml
        svd2rust --target mips -g -m -i $SVD.patched
        form -i mod.rs -o src/ && rm mod.rs
        mv src/lib.rs src/mod.rs
#        ../pic2irqsources.py $EDC >> src/interrupt.rs
        ../svd2devicex.py -o device.x $SVD.patched
        #rm -f $SVD.patched
        popd
        mkdir src/$dir
        mv $dir/device.x src/$dir
        mv $dir/src/* src/$dir
        rmdir $dir/src
}

if [ -e src ]; then
        echo -e "src directory already exists"
        exit -1
fi

SVD2RUST_VERSION=$(svd2rust --version)

mkdir src
sed "{s/\$SVD2RUST_VERSION/$SVD2RUST_VERSION/}" lib.rs.template > src/lib.rs

build_module pic32mx695fxxxl PIC32MX695F512L

cp pic32mx695fxxxl/generic.rs src
rm -r pic32*/generic.rs

cargo fmt
