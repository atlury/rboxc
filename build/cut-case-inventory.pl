no warnings 'redefine';
*main::run_tests = sub ($$$$$) {
    for my $case (@{$_[2]}) { print $case->[0], "\n"; }
    return 0;
};
do $ARGV[0]; die $@ if $@;
