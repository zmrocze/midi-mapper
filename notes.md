
# midi mapper

care for aftertouch: it keeps sending many noteOn's after the first noteOn 
first iteration: ignore or turn off on device aftertouch
think how to use

## todo

 - simplest config format for rust. all the various interfaces as dhall lib
 - consider trie if hashmap is slow (there may be some 135000 keys)

## config format

yaml or dhall (to easily specify mapping between all 127 notes)
but probably doesnt work with current config