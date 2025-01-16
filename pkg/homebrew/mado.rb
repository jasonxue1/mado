# frozen_string_literal: true

class Mado < Formula
  desc "Fast Markdown linter written in Rust"
  homepage "https://github.com/akiomik/mado"
  version "0.1.4"
  license "Apache-2.0"

  on_macos do
    on_arm do
      url "https://github.com/akiomik/mado/releases/download/v#{version}/mado-macOS-arm64.tar.gz"
      sha256 "701547b6def685c3a771ab4537e8b22c4b346d836cf2cbd96456b984ddc04360"
    end

    on_intel do
      url "https://github.com/akiomik/mado/releases/download/v#{version}/mado-macOS-x86_64.tar.gz"
      sha256 "fca3110954e2aa0eb3bac76ccdded961a22c56c33d2cafef79c5c83fd7a115dc"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/akiomik/mado/releases/download/v#{version}/mado-Linux-gnu-arm64.tar.gz"
      sha256 "1a311f58f5222c32cc4181f50097fd3571d5524713c20c7bfb945f085cbe8ec2"
    end

    on_intel do
      url "https://github.com/akiomik/mado/releases/download/v#{version}/mado-Linux-gnu-x86_64.tar.gz"
      sha256 "0581969c538bdac9720ef6ce104a812f0b81d6dbfa9e8112d1da37dfb16b80a4"
    end
  end

  def install
    bin.install "mado"
  end

  test do
    assert_equal "mado #{version}", shell_output("#{bin}/mado --version").strip
  end
end
