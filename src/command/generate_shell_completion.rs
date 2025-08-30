use std::io;

use clap::Command;
use clap_complete::{generate, shells, Generator};

use crate::command::CompletionShell;

#[derive(Debug, Clone)]
pub struct ShellCompletionGenerator {
    cmd: Command,
}

impl ShellCompletionGenerator {
    #[inline]
    #[must_use]
    pub const fn new(cmd: Command) -> Self {
        Self { cmd }
    }

    #[inline]
    pub fn generate<G: Generator>(&mut self, gen: G) {
        let name = self.cmd.get_name().to_owned();
        generate(gen, &mut self.cmd, name, &mut io::stdout());
    }

    #[inline]
    pub fn generate_for(&mut self, shell: CompletionShell) {
        match shell {
            CompletionShell::Bash => self.generate(shells::Bash),
            CompletionShell::Elvish => self.generate(shells::Elvish),
            CompletionShell::Fish => self.generate(shells::Fish),
            CompletionShell::PowerShell => self.generate(shells::PowerShell),
            CompletionShell::Zsh => self.generate(shells::Zsh),
            CompletionShell::Nushell => self.generate(clap_complete_nushell::Nushell),
            CompletionShell::Fig => self.generate(clap_complete_fig::Fig),
        }
    }
}
