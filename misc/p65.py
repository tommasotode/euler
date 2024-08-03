import math

def lcm(a, b):
    return abs(a * b) // math.gcd(a, b)

def fraction_sum(f1, f2):
    res = [0, 0]

    res[1] = lcm(f1[1], f2[1])
    res[0] = (res[1] // f1[1]) * f1[0] + (res[1] // f2[1]) * f2[0]

    gcd = math.gcd(res[0], res[1])
    res[1] //= gcd
    res[0] //= gcd

    return tuple(res)

def inverted_sum(f1, f2):
    res = fraction_sum(f1, f2)
    res = (res[1], res[0])
    return res

def get_k_convergent(k, count):
    val = [1, 1]
    if count % 3 == 0:
        val[0] = 2 * (count // 3)

    if k == 0:
        return (1, 1)

    conv = get_k_convergent(k - 1, count + 1)
    a = inverted_sum(tuple(val), conv)

    return a

def main():
  s = fraction_sum((2,1), get_k_convergent(98, 2));
  
  sum = 0
  for c in str(s[0]):
    sum += int(c)
  print(sum)

if __name__ == "__main__":
    main()
