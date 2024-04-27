scop: all

run:
	cargo run --release -- resources/objs/42.obj

all:
	cargo build --release

clean:
	cargo clean

fclean: clean

re: fclean all

.PHONY: scop run all clean fclean re