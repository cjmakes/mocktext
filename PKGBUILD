# Maintainer: conjones <cj@cjmakes.com>
pkgname=mocktext
pkgver=0
pkgrel=1
epoch=
pkgdesc="make MoCk TeXt"
arch=(x86_64)
url="https://github.com/cjmakes/mocktext"
license=('MIT')
groups=()
depends=()
makedepends=()
checkdepends=()
optdepends=()
provides=()
conflicts=()
replaces=()
backup=()
options=()
install=
changelog=
source=()
noextract=()
md5sums=()
validpgpkeys=()

build() {
  cargo build --release --locked --all-features --target-dir=target
}

check() {
  cargo test --release --locked --target-dir=target
}

package() {
  install -Dm 755 target/release/${pkgname} -t "${pkgdir}/usr/bin"
}
