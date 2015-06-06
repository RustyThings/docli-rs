# Maintainer: Kevin K. <kbknapp@gmail.com>

_pkgname=docli
pkgname=${_pkgname}-bin
pkgrel=1
pkgver=0.1.0-beta
pkgdesc="A command line utility for managing DigitalOcean infrastructure"
url="https://kbknapp.github.io/docli-rs"
provides=('docli')
arch=('x86_64')
license=('MIT')
makedepends=()
conflicts=('docli-git')
replaces=('docli-git')
backup=()
install=''
source=("http://wod.twentyfives.net/bin/${_pkgname}/${_pkgname}-${pkgver}-${CARCH}-linux.tar.gz")
sha256sums=('SKIP')

package() {
  cd "${srcdir}/${_pkgname}-${pkgver}-${CARCH}-linux/"

    install -D -m644 LICENSE-MIT "${pkgdir}/usr/share/licenses/${_pkgname}/LICENSE-MIT"
    install -Dm 0755 docli/bin/docli "${pkgdir}/usr/bin/docli"
}

