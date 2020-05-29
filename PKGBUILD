# Maintainer: Mr. Fawkes <poisonimy@protonmail.com>
pkgname=musicio-git
pkgver=v0.1.0.alpha.r0.b03eb2e
pkgrel=1
pkgdesc="A program that records from multiple audio sources and plays it back to an output (e.g. a sink)."
arch=(any)
url="https://github.com/poisonimy/musicio"
license=(MIT)
depends=(libpulse)
makedepends=(git pkgconf)
provides=("${pkgname%-VCS}")
conflicts=("${pkgname%-VCS}")
source=("${pkgname}::git+https://github.com/poisonimy/musicio")
noextract=()
md5sums=('SKIP')

pkgver() {
	cd "$srcdir/${pkgname}"
	printf "%s" "$(git describe --long --tags | sed 's/\([^-]*-\)g/r\1/;s/-/./g')"
}

build() {
	cd "$srcdir/${pkgname}"
    cargo build --release --locked
}

package() {
	cd "$srcdir/${pkgname}"
	install -Dm 755 target/release/musicio -t "${pkgdir}/usr/bin"
}