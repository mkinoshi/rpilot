# Rpilot

Rpilot is a CLI tool for managing env files. You can use multiple env files for one project with one command. It is similar to how you can have multiple profiles for AWS CLI tool.

## Before using this

This CLI will generate two files. Please add them in `gitignore` of the project.

```
.env
.rpilot
```

You also need administrative permissions for `apply` and `edit` commands.

## Installation

### Download from releases page

Download the rpilot binary from [the GitHub Releases tab](https://github.com/mkinoshi/rpilot/releases).

## Usage Examples

- Initialize Rpilot for your project

```
rpilot init
```

- Add a new env profile

```
rpilot add --name default
```

- Apply the specific profile

```
rpilot apply --name default
```

- List all of the available profiles

```
rpilot list
```

- Edit a specific profile

```
rpilot edit --name default
```

- Remove a specific profile

```
rpilot remove --name default
```

- Show the env variables for a specific profile

```
rpilot show --name default
```

- Get the current profile

```
rpilot current
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate. Enjoy!

## License

[MIT](https://choosealicense.com/licenses/mit/)
