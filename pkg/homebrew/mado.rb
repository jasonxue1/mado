# frozen_string_literal: true

class Mado < Formula
  desc "Fast Markdown linter written in Rust"
  homepage "https://github.com/akiomik/mado"
  version "0.2.0"
  license "Apache-2.0"

  on_macos do
    on_arm do
      url "https://github.com/akiomik/mado/releases/download/v#{version}/mado-macOS-arm64.tar.gz"
      sha256 "3900c1a5a599ae07e3e1baf3efe20a3b8e3c3cddcb1f660eb2f025ae0b92a164"
    end

    on_intel do
      url "https://github.com/akiomik/mado/releases/download/v#{version}/mado-macOS-x86_64.tar.gz"
      sha256 "af3a80784be80c24e09ff313d92fd40a9165f6435570b00da33d259a6c8f90cf"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/akiomik/mado/releases/download/v#{version}/mado-Linux-gnu-arm64.tar.gz"
      sha256 "dfae957b710c2d492b4c83503f66b8cf19aea6287cc69d95815bd7f945d7173e"
    end

    on_intel do
      url "https://github.com/akiomik/mado/releases/download/v#{version}/mado-Linux-gnu-x86_64.tar.gz"
      sha256 "49fc78169fc60f8484931323525a5cfc72da89b6cb4f70774f93dca6d300f69c"
    end
  end

  def install
    bin.install "mado"
  end

  test do
    assert_equal "mado #{version}", shell_output("#{bin}/mado --version").strip
  end
end
