from numpy import random
import click

@click.command()
@click.option("--min", help="min number for random", required=True, type=click.INT)
@click.option("--max", help="max number for random", required=True, type=click.INT)
@click.option("--amount", help="repeat amount to random", default=1, type=click.INT)
@click.option("--output", help="if value is std print it, else write it to [output].txt", default="std")
def main(min: int, max: int, amount: int, output: str):
    """random some numbers."""
    if min >= max:
        click.echo("can't be min >= max")
    elif output == "std":
        for a in getRandomNumber(min, max, amount):
            click.echo(str(a))
    else:
        with open(f"{output}.txt", "w") as f:
            for a in getRandomNumber(min, max, amount):
                f.write(f"{str(a)}\n")

def getRandomNumber(min: int, max: int, size: int):
    return random.randint(min, max, size=size)

if __name__ == "__main__":
    main()