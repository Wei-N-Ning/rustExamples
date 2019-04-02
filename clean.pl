#!/usr/bin/env perl
use warnings;
use strict;
use 5.010;

use File::Basename;

sub main {
    foreach(split(/\n/, `find . -name "Cargo.toml" -type f`)) {
        my $dirname = dirname($_);
        `cd $dirname && cargo clean`;
    }
}

main();
