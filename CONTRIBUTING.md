# Contributing to Edge Hive

Thank you for your interest in contributing to Edge Hive! 🐝

## 🤝 How to Contribute

Edge Hive follows the **Git-Core Protocol** for development workflow. Please read [AGENTS.md](./AGENTS.md) before contributing.

### Quick Start

1. **Find an issue**: Check [open issues](../../issues) or create a new one
2. **Claim it**: Comment on the issue or assign yourself
3. **Create branch**: `git checkout -b feat/issue-<number>`
4. **Make changes**: Follow our code standards (see below)
5. **Test**: Ensure all tests pass (`cargo test`)
6. **Commit**: Use Conventional Commits (see below)
7. **Create PR**: Reference the issue in your PR description

### Commit Message Format

We use **Conventional Commits** with AI-Context extension:

```
<type>(<scope>): <description> #<issue-number>

[optional body]

[optional AI-Context footer]
```

**Types:**

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Maintenance tasks
- `ci`: CI/CD changes

**Example:**

```
feat(discovery): implement Kademlia DHT #42

Adds global node discovery via libp2p Kademlia DHT.
Nodes can now find each other across different networks.

AI-Context: chose Kademlia over centralized discovery
for censorship resistance and zero server costs
```

See [docs/reference/COMMIT_STANDARD.md](./docs/reference/COMMIT_STANDARD.md) for full details.

## 📋 Code Standards

### Rust

- **Format**: Run `cargo fmt` before committing
- **Lints**: Run `cargo clippy` and fix all warnings
- **Tests**: Add tests for new features
- **Docs**: Add rustdoc comments for public APIs

### TypeScript/Svelte (App)

- **Format**: Run `npm run format` (Prettier)
- **Lint**: Run `npm run lint` (ESLint)
- **Types**: Ensure no TypeScript errors

## 🧪 Testing

```bash
# Rust workspace
cargo test --workspace

# Specific crate
cargo test -p edge-hive-core

# Tauri app
cd app
npm test
```

## 📝 Documentation

- **Code**: Add rustdoc/JSDoc comments
- **Architecture**: Update `.✨/ARCHITECTURE.md` for major changes
- **User docs**: Add to `docs/` following Diátaxis framework

## 🔒 Security

Found a vulnerability? **DO NOT** open a public issue.

Email: <security@edge-hive.io> (TODO: setup)

## 📜 License

By contributing, you agree that your contributions will be licensed under the BSL 1.1 license (see [LICENSE](./LICENSE)).

---

**Questions?** Open a discussion or ask in an issue.
