#!/bin/sh
cd `dirname $0`/..
root=`pwd`
tmp=/tmp/prepare-cadical-sr2018-submission.log
VERSION=`cat VERSION`
rm -f $tmp
##########################################################################
cd $root
./scripts/make-src-release.sh | tee $tmp
tar=`awk '{print $2}' $tmp |sed -e "s,',,g"`
##########################################################################
prepare () {
option=$1
cd $root
base=cadical-${VERSION}-starexec-$option
dir=/tmp/$base
rm -rf $dir
mkdir $dir
mkdir $dir/bin
mkdir $dir/build
mkdir $dir/archives
cp -a $tar $dir/archives
cat <<EOF >$dir/build/build.sh
#!/bin/sh
tar xf ../archives/cadical*
mv cadical* cadical
cd cadical
./configure
make test
install -s build/cadical ../../bin/
EOF
chmod 755 $dir/build/build.sh
cat <<EOF >$dir/starexec_build
#!/bin/sh
cd build
exec ./build.sh
EOF
chmod 755 $dir/starexec_build
cat <<EOF >$dir/bin/starexec_run_default
#!/bin/sh
exec ./cadical --$option \$1 \$2/proof.out
EOF
chmod 755 $dir/bin/starexec_run_default
archive=/tmp/$base.zip
rm -f $archive
cd $dir
zip -r $archive .
cd /tmp/
ls -l $archive
#rm -f $tmp
#rm -rf $dir/
}
##########################################################################
prepare default
#prepare unsat
#prepare sat
