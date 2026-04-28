# Maintainer: fibsussy <noahlykins@gmail.com>
pkgname=reaction-time-test
pkgver=0.1.0
pkgrel=1
pkgdesc="A CLI/TUI reaction time test inspired by Human Benchmark"
arch=('x86_64' 'aarch64')
url="https://github.com/fibsussy/reaction-time-test"
license=('MIT')
makedepends=('cargo')
options=('!debug')

build() {
    cd "$startdir"
    cargo build --release
}

package() {
    cd "$startdir"

    install -Dm755 target/release/reaction-time-test "$pkgdir/usr/bin/reaction-time-test"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
