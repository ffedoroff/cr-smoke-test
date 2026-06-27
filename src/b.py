from src.c import work_c
def work_b(): return work_c() + 1

def extra(x, y, z):
    if x > 0:
        if y > 0:
            return x + y + z
    return 0
