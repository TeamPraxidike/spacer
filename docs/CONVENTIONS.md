<!-- CONVENTIONS AND RULES WHEN DEVELOPING IN THIS REPOSITORY -->

# Rules and conventions

This document will describe rules that MUST be followed by contributors to this project during development and coding. The document [Code of Conduct](CODE_OF_CONDUCT.md) focuses more on rules regarding the behavior of Contributors.

## Code contribution workflow

Contributions should follow the correct order of steps. This way the project development will remain of consistent structure and workflow.

1. Locate the issue (missing feature, bug, etc.) and open a well-documented issue about it.
2. Branch out from the **correct branch** and use the branch naming convention specified below.
3. Commit your changes in a controlled and well-manner. Use the convention for naming your commits.
4. Open a **Merge Request** and if possible request reviews from other developers.

### Issues conventions

Issues are a great way to contribute to the project and to keep track of its future updates.

1. The issues must follow the naming convention `<TYPE>: <Title>`:
   
   * The `<Title>` should begin with an uppercase letter and briefly describe the issue

2. The body of an issue must follow the provided templates or should adhere to their structure iff there is no template available for this type of issue. 

Spamming issues or in any other way breaking the Code of Conduct may result in a ban or mute.

### Branch conventions

Most of the branch conventions are taken from the [branching standards & conventions gist](https://gist.github.com/digitaljhelms/4287848).

1. **Branches follow the below-specified logic:**
   
   | Instance          | Branch    | Branches from | Accepts from    | Description                                                                                                                                             |
   | ----------------- | --------- | ------------- | --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
   | Working           | `trunk`   | -             | Features/Issues | The main working branch that encapsulated current development. Accepts `features` merges.  Issues on this branch must be solved before publishing tags. |
   | Features / Issues | `topic-*` | trunk         | -               | Used when developing a new feature or an enhancement OR a bug fix that could be implemented on a later stage.                                           |

2. **Branches must follow the following naming conventions:**
   
   * `trunk` - always reflects a state with the latest delivered development changes for the next release
     * a developer branches and merges from `trunk`
   * `<issue number>-feature-<name>` - logically isolated changes related to a specific feature or user story
   * `<issue number>-bugfix-<name>` - changes related to fixing a specific bug

### Commit conventions

**Commit messages must follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) standards**.

```markdown
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

 Here are some additional rules that abide to the original ones but are focused on this repository:

* The `<type>` keyword could be one of the following **nouns**:
  * `feat` - a new feature is introduced in the changes
  * `fix` - a bug fix has occurred
  * `chore` - changes that do not relate to a fix or feature and do not modify files
  * `refactor` - the changes refactor code (neither fixes a bug nor adds a feature)
  * `docs` - changes the documentation (both in .md files and inline code documentation)
  * `style` - changes that do not affect the meaning or behavior of the code (formatting)
  * `test` - including new or correcting old tests
  * `perf` - performance enhancement
* The `[optional scope]` can be one of the following (or if reasonable a new one). Keep in mind that points to what the commit is focused on and not all changes must be in this specific scope (you may also do small changes in other files too)
* The `<description>` must be in lower-case and in imperative

### Merge request conventions

1. Merge requests MUST adhere to the branch merging conventions specified above.
2. Merge requests MUST follow the `pull_request_template.md` and provide descriptive change-log

## Comments & Reviews

### Comments

1. **Comments must follow the rules written in the [Code of Conduct](CODE_OF_CONDUCT.md)**.
2. Comments must not be spam messages.
3. Comments may include meme references.

### Reviews

1. Merge requests must be reviewed by at least one developer before being merged
2. Reviews MUST be descriptive and **MUST NOT criticize the developer in any nonconstructive or derogatory way.**
3. Critics should be well-argued and provoke discussion rather than insulting the developer.
