{ lib, fetchFromGitLab, rustPlatform }:

rustPlatform.buildRustPackage rec {
  pname = "kile";
  version = "unstable-2021-04-19";

  src = fetchFromGitLab {
    owner = "snakedye";
    repo = pname;
    rev = "f302612c43d51a405b46104750f1d4f12f59fbb5";
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
