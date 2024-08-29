# TMPRS

TMPRS is a **T**erminal-based **M**usic **P**layer made using **R**u**s**t

See what I did there?

I'd like to stress that while TMPRS is functional, it may have bugs and it is not very aestheticly pleasing.
If you encounter any errors, please create an issue (and do the same if you have any feature requests!).

## Installation

TMPRS 0.1.1 is currently avaliable for Fedora (Versions 39, 40 and rawhide for architectures aarch64 and x86_64) and OpenSUSE (Versions Leap-15.5, Leap-15.6 and Tumbleweed for architectures x86_64 for Leap and both x86_64 and i586 for Tumbleweed)

### Fedora

TMPRS is avaliable for Fedora through a Copr repo:
https://copr.fedorainfracloud.org/coprs/aurapy/tmprs/

#### Quick installation instructions:

Enable the Copr repo:
`dnf copr enable aurapy/tmprs`
Install TMPRS:
`dnf install tmprs`

### OpenSUSE

TMPRS is avaliable for OpenSUSE though OpenSUSE Build Service:
https://build.opensuse.org/package/show/home:Zayan/tmprs

#### Quick installation instructions:

Enable the OBS repo:
`zypper addrepo https://download.opensuse.org/repositories/home:/Zayan/openSUSE_{version}/home:Zayan.repo`
Install TMPRS:
`zypper install tmprs`
