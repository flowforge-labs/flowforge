module.exports = {
  extends: ["@commitlint/config-conventional"],
  rules: {
    "scope-enum": [2, "always", [
      "backend",
      "frontend",
      "models",
      "ingest",
      "watcher",
      "storage",
      "api",
      "docs",
      "repo",
      "ci"
    ]]
  }
};
/*"build",
				"chore",
				"ci",
				"docs",
				"feat",
				"fix",
				"perf",
				"refactor",
				"revert",
				"style",
				"test",*/