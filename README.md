# gh-ownership

This is a simple Rust implementation of GitHub's CODEOWNERS feature. It takes
the `git diff` result and returns the owners of each file touched by the changes.

## Usage

```
$ git diff main | gh-ownership

app/models/fee.rb  @orgname/teamname1
app/controllers/cookie_policies_controller.rb   @orgname/teamname2
app/controllers/home_controller.rb  @orgname/teamname3
config/environment.rb  @orgname/teamname2
app/models/application_record.rb @orgname/teamname4
app/models/user.rb  @orgname/teamname4
db/schema.rb @orgname/teamname4
```

## Requirements

```
$ rustc --version
rustc 1.67.0 (fc594f156 2023-01-24)
```

## How to build

```
cargo build --release
```

## Known limitations

If there are more than one rule can be applied for the file both results will be
returned:

```
# CODEOWNERS
db/ @orgname/teamname1
db/schema.rb @orgname/teamname2

$ git diff main | gh-ownership
db/schema.rb @orgname/teamname1
db/schema.rb @orgname/teamname2
```
