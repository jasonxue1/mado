# frozen_string_literal: true

class Mado < Formula
  desc "Fast Markdown linter written in Rust"
  version "0.1.3"
  homepage "https://github.com/akiomik/mado"
  license "Apache-2.0"

  on_macos do
    on_arm do
      url "https://github.com/akiomik/mado/releases/download/v#{version}/mado-macOS-arm64.tar.gz"
      sha256 "457845e71ebbfec6fd966176c25098770492976e6f466bbe59d396616d1c8ff3"
    end

    on_intel do
      url "https://github.com/akiomik/mado/releases/download/v#{version}/mado-macOS-x86_64.tar.gz"
      sha256 "db1515c8010fab6b3522014271b9e7e65effc6d254cced95ddd07e6192e6ee74"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/akiomik/mado/releases/download/v#{version}/mado-Linux-gnu-arm64.tar.gz"
      sha256 "6d57a766dcb2fbf86a9db8ef85511ad1d4ed72ebf19f12ec4701e4775dd9369b"
    end

    on_intel do
      url "https://github.com/akiomik/mado/releases/download/v#{version}/mado-Linux-gnu-x86_64.tar.gz"
      sha256 "944bf94d1715a8bba01395c179c4cfc8b9f5a468d13f614cc3dd58f25dd05d51"
    end
  end

  def install
    bin.install "mado"
  end

  test do
    assert_equal "mado #{version}", shell_output("#{bin}/mado --version").strip
  end
end
