#!/bin/sh -ex
: ${1:?no argument: spec file should be given}

throw(){ echo $*  >&2; exit 1;  }
[ -f "$1" ] || throw no such file "$1"

get_version(){
	cat  $1 | jq .info.version | { read v; v=${v#\"}; echo ${v%\"}; }
}

{
	get_version $1
	get_version $1
	get_version $1
} | {
	read orig_version
	read major_and_minor
	read subminor
	major_and_minor=${major_and_minor%\.*}
	subminor=${subminor##*\.}
	v=${major_and_minor:?no major ver}-${subminor:?no subminor ver}
	while read -r line ; do 
		if [ "$line" = "version = \"${orig_version:?no orig}\"" ]; then
			echo version = \"$v\"
		elif [ "$line" != "${line#authors =}" ]; then
			echo homepage = \"https://github.com/g1eng/jpid-rs\"
			echo repository = \"https://github.com/g1eng/jpid-rs\"
			echo documentation = \"https://github.com/g1eng/jpid-rs\"
		else
			echo "$line"
		fi
	done < Cargo.toml > Cargo.toml.next
	mv Cargo.toml.next Cargo.toml
}
