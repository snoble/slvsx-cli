class Slvsx < Formula
  desc "Geometric constraint solver using SolveSpace"
  homepage "https://github.com/snoble/slvsx-cli"
  url "https://github.com/snoble/slvsx-cli/releases/download/v0.1.0/slvsx-macos"
  sha256 "PLACEHOLDER_SHA256"
  license "GPL-3.0-or-later"
  version "0.1.0"

  def install
    bin.install "slvsx-macos" => "slvsx"
  end

  test do
    output = shell_output("#{bin}/slvsx --version")
    assert_match "slvsx", output
  end
end

# To install:
# brew tap snoble/slvsx
# brew install slvsx