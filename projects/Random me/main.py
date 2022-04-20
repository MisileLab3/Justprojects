from secrets import SystemRandom
import click

@click.command()
@click.option("--min", help="min number for random", type=click.INT)
@click.option("--max", help="max number for random", type=click.INT)
@click.option("--amount", help="repeat amount to random", default=1, type=click.INT)
@click.option("--output", help="if value is std print it, else write it to [output].txt", default="std")
def main(min: int, max: int, amount: int, output: str):
    """random some numbers."""
    if output == "std":
        for _ in range(amount):
            click.echo(str(SystemRandom().randint(min, max)))
    else:
        with open(f"{output}.txt", "w") as f:
            for _ in range(amount):
                f.write(str(SystemRandom().randint(min, max)))

if __name__ == "__main__":
    main()