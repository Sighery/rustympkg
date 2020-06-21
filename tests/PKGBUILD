# Maintainer: Sighery
pkgname=terraform-provider-njalla
pkgver=0.7.0
pkgrel=1
pkgdesc="Unofficial Terraform Njalla provider plugin"
url='https://github.com/Sighery/terraform-provider-njalla'
arch=('x86_64')
license=('MIT')
makedepends=('go')
source=(
	"$pkgname-$pkgver.tar.gz::$url/archive/v$pkgver.tar.gz"
)
sha256sums=('29d5b4c94dcfe2260e0d217392e2aa935a6b81e7388f72305fde87f0b680189a')

build() {
	export CGO_LDFLAGS="${LDFLAGS}"
	export GOFLAGS="-trimpath"

	cd $pkgname-$pkgver
	go build -o "${pkgname}_v${pkgver}"
}

package() {
	cd $pkgname-$pkgver

	install -Dm755 "${pkgname}_v${pkgver}" "$pkgdir/usr/bin/${pkgname}_v${pkgver}"

	# MIT license needs to be installed separately
	install -Dm644 LICENSE $pkgdir/usr/share/licenses/$pkgname/LICENSE
}
