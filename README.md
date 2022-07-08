# Dev Template

![dtemplate](https://user-images.githubusercontent.com/72868196/160446275-1c6983f9-284a-4067-b28f-da4f80be3a37.jpg)

```sh
dt new -t Hello-World
```

No more installing and configuring all basic dependencies manually every time you start a new project. the dev template can install and configure everything with a single command. A CLI that runs templates with a sequence of commands to launch your projects. Anyone (including you) can create a template that will install and run the commands needed to start your projects.

## Install

for the installation of all versions it is necessary to have [unzip](http://stahlworks.com/dev/index.php?tool=zipunzip) installed

Shell (Mac, Linux):

```sh
curl -fsSL https://raw.githubusercontent.com/dtemplate/dt/master/install.sh | sh
```

PowerShell (Windows):

```powershell
iwr https://raw.githubusercontent.com/dtemplate/dt/master/install.ps1 -useb | iex
```

## Example

Create and open a new empty folder and run our hello-world as an example.

```sh
mkdir example-dt
cd example-dt
dt new -t Hello-World
```

## How to use

To get started you need to keep in mind one of the community created templates for the dev template, see the [list](https://dtemplate.org/templates) if you don't know one.

Now run follow the instructions in template documentation or only run:

```sh
dt new -t Hello_World
```

You can get the template name into the [list](https://dtemplate.org/templates).

## DOCS

Documentation is available at: [https://dtemplate.org/docs](https://dtemplate.org/docs)

Documentation repository is at: [https://github.com/dtemplate/documentation](https://github.com/dtemplate/documentation)
