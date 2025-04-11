# frozen_string_literal: true

class Mado < Formula
  desc "Fast Markdown linter written in Rust"
  homepage "https://github.com/akiomik/mado"
  version "0.3.0"
  license "Apache-2.0"

  on_macos do
    on_arm do
      url "https://github.com/akiomik/mado/releases/download/v#{version}/mado-macOS-arm64.tar.gz"
      sha256 "4000955c41799c839dbf1b4c3011ff1688bd31d0af88be282eec622a07ad9743"
    end

    on_intel do
      url "https://github.com/akiomik/mado/releases/download/v#{version}/mado-macOS-x86_64.tar.gz"
      sha256 "b8a3fb5cf3e84747c12a848fb87ce08c1c09dc5a957c1f74733bea6cb7f9d560"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/akiomik/mado/releases/download/v#{version}/mado-Linux-gnu-arm64.tar.gz"
      sha256 "ffcdd4845329a69bc729c0242abb6163f23495c0b04dbbaf608cbed43a2f4976"
    end

    on_intel do
      url "https://github.com/akiomik/mado/releases/download/v#{version}/mado-Linux-gnu-x86_64.tar.gz"
      sha256 "aad845cd23c8c0417cdf87b8376b75e131c38e1cb890124790567735306de6d7"
    end
  end

  def install
    bin.install "mado"
  end

  test do
    assert_equal "mado #{version}", shell_output("#{bin}/mado --version").strip
  end
end
