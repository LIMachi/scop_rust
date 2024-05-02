scop: all

run:
	cargo run --release -- 42

test1:
	cargo run --release -- teapot

test2:
	cargo run --release -- dragon

test3:
	cargo run --release -- 42 dragon 42 dragon

all:
	cargo build --release

clean:
	cargo clean

fclean: clean

re: fclean all

.PHONY: scop run all clean fclean re test1 test2 test3