import argparse
import typing as t

from newp import templates


def options_from_args() -> t.Optional[templates.Options]:
    parser = argparse.ArgumentParser(description="Creates a new project")
    subparsers = parser.add_subparsers(help="command")
    list_p = subparsers.add_parser("list", help="List project types")
    list_p.set_defaults(list="list")
    create_p = subparsers.add_parser("create", help="Create a new project")
    create_p.add_argument("type", type=str, help="type of project")
    create_p.add_argument("name", type=str, help="name of new project")
    create_p.add_argument("description", type=str, help="Description")
    create_p.add_argument(
        "--dir",
        type=str,
        help="directory (defaults to a new directory named after project)",
    )

    p_args = parser.parse_args()

    if hasattr(p_args, "list"):
        return None
    else:
        return templates.Options(
            p_args.type,
            p_args.name,
            p_args.directory or p_args.name,
            p_args.description,
        )


def cli() -> None:
    options = options_from_args()
    if not options:
        for name, desc in templates.get_list():
            print(name)
            print("\t" + desc)
            print()
    else:
        print(f"{options.name} or {options.type} at {options.directory}")


if __name__ == "__main__":
    cli()
