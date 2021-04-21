{ lib, fetchFromGitLab, rustPlatform }:

rustPlatform.buildRustPackage rec {
  pname = "kile";
  version = "unstable-2021-04-19";

  src = fetchFromGitLab {
    owner = "snakedye";
    repo = kile;
    rev = "b97b9f1e5b33862b33918efaf23fd1c0c5d7058a";
    sha256 = lib.fakeSha256;
  };

  cargoSha256 = lib.fakeSha256;

  meta = with lib; {
    description = "A tiling layout generator for river";
    homepage = "https://gitlab.com/snakedye/kile";
    license = licenses.mit;
    maintainers = with maintainers; [ fortuneteller2k ];
  };
}
