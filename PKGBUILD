# Maintainer: Alessio Deiana <adeiana@gmail.com>
pkgname=git-merge-deps
pkgver=0.1.0
pkgrel=1
pkgdesc="Git merge driver for pip requirements files"
arch=('x86_64')
url="https://github.com/adeiana/git-merge-deps"
license=('MIT')
depends=('gcc-libs')
makedepends=('cargo')

build() {
    cd "$startdir"
    cargo build --release --locked
}

package() {
    cd "$startdir"
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
}
