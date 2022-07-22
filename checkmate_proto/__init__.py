
def main():
    from .cli import CLI, create_commands

    CLI(create_commands()).run()
