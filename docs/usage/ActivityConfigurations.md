# ActivityConfigurations

Note: the contents of this file are planned (not yet implemented)

``dsdk`` supports per-Activity Configurations. An activity is something you,
the developer, do (e.g. developing, testing, testing on the newer Java,
building for deployment), and ``dsdk`` allows you to configure your versions
per each activity and project.

## Projects

We expect each project to have a unique identifier. This is the minimum 
requirement for using ACs.

The unique identifier is stored in the ``.dsdk.json`` file, and is a 
simple string literal, as follows:

```jsonc
{
  // by the way, we support comments. 
  "projectId": "my-awesome-project"
}
```

It's not **absolutely** required to keep this identifier unique among your 
project, however reusing it will likely lead to unexpected results.

## Activities

An Activity is also identified by a simple string literal.

Activities is how you specify a project's environmental dependencies.
Hence, Activity Configurations. You can configure said dependencies in one 
of the following ways:

## Global configuration

Any ``.json`` files in ``$DROIDSDK_WORK_DIR/config/activities`` will be parsed
whenever you choose to switch activities. The format of these files is
as follows:

```json
{
    "java7": {
        "projectDeps": {
            "API gateway": {
              "kotlin": "1.3.60",
              "java": "11"
            },
            "payment system": {
              "java": "7"
            }
        }
    },
    "java8": {
        "projectDeps": {
            "API gateway": {
              "kotlin": "1.4.10",
              "java": "11"
            },
            "payment system": {
              "java": "8"
            }
        }
    }
} 
```

## Project configuration

If you dislike storing all of your configuration in a single place, or
if you want to quickly automate a project without delving into the jungle
of files in your ``/config/activities``, or if you want to store a project's
dependency information with it, you can of course include the configuration
within your project's root directory.

The format in this case is:


```json
{
    "projectId": "API gateway",
    "activities": {
        "java7": {
            "kotlin": "1.3.60",
            "java": "11"
        },
        "java8": {
            "kotlin": "1.4.10",
            "java": "11"
        }
    }
} 
```