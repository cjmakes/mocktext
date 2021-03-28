# Maintainer: conjones <cj@cjmakes.com>
pkgname=mocktext
pkgver=0
pkgrel=1
pkgdesc="make MoCk TeXt"
arch=(x86_64)
url="https://github.com/cjmakes/mocktext"
license=(MIT)
depends=(glibc gcc-libs)
makedepends=(rust git)
provides=(mocktext)
conflicts=(mocktext)
replaces=()
source=("${pkgname}::git+${url}")
sha512sums=(SKIP)


build() {
  cargo build --release --locked --all-features --target-dir=target
}

check() {
  cargo test --release --locked --target-dir=target
}

package() {
  install -Dm 755 target/release/${pkgname} -t "${pkgdir}/usr/bin"
}
