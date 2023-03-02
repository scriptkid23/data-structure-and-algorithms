package com.spirity.recursion;

import java.util.ArrayList;

public class PrimeFactorization {
    ArrayList<Integer> storage = new ArrayList<>();

    public int calculate(int n) {

        for (int i = 2; i <= Math.sqrt(n); i++) {
            if (n % i == 0) {
                return calculate(i) * calculate(n / i);
            }
        }
        this.storage.add(n);
        if (n == 2)
            return 2;
        return n;
    }

    public void result() {
        System.out.println(storage.toString());
    }

    public static void main(String[] args) {
        PrimeFactorization pf = new PrimeFactorization();
        pf.calculate(305656);
        pf.result();
    }
}
