# frozen_string_literal: true

class Mado < Formula
  desc "Fast Markdown linter written in Rust"
  homepage "https://github.com/akiomik/mado"
  version "0.1.5"
  license "Apache-2.0"

  on_macos do
    on_arm do
      url "https://github.com/akiomik/mado/releases/download/v#{version}/mado-macOS-arm64.tar.gz"
      sha256 "a0191c43c4c2ca81fb02c614c6d2d56b5f4fb003bd8aa7a37a58fbc1ffe98f2e"
    end

    on_intel do
      url "https://github.com/akiomik/mado/releases/download/v#{version}/mado-macOS-x86_64.tar.gz"
      sha256 "117e8c732758206b96c714bca0edb93c9b3fb6fb0a3dd4638bc771bf22b3eae7"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/akiomik/mado/releases/download/v#{version}/mado-Linux-gnu-arm64.tar.gz"
      sha256 "a79da53e10bd5f3aa7f45b6548acc9eda5927b5503d743aecba6267365efd99c"
    end

    on_intel do
      url "https://github.com/akiomik/mado/releases/download/v#{version}/mado-Linux-gnu-x86_64.tar.gz"
      sha256 "7d8bfc8d977a21827de30fc7e51e5c52b09d3aeb15ed107577ce9c76d9c0fb7c"
    end
  end

  def install
    bin.install "mado"
  end

  test do
    assert_equal "mado #{version}", shell_output("#{bin}/mado --version").strip
  end
end
