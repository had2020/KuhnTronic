#include <stdint.h>

double fast_exp(double x) {
    const double LN2_HI = 0.6931471805599453;
    const double LN2_LO = 2.3190468138462996e-17;
    const double INV_LN2 = 1.4426950408889634;

    if (x > 709.782712893384) return 1.0 / 0.0;
    if (x < -708.396418532264) return 0.0;

    double k_real = x * INV_LN2;
    int64_t k = (int64_t)(k_real + (k_real >= 0.0 ? 0.5 : -0.5));

    double r = x - (double)k * LN2_HI - (double)k * LN2_LO;

    const double c0 = 1.0;
    const double c1 = 1.0;
    const double c2 = 0.5;
    const double c3 = 0.16666666666666666;
    const double c4 = 0.041666666666666664;
    const double c5 = 0.008333333333333333;
    const double c6 = 0.0013888888888888889;

    double r2 = r * r;
    double r4 = r2 * r2;

    double p01 = c0 + r * c1;
    double p23 = c2 + r * c3;
    double p45 = c4 + r * c5;

    double p03 = p01 + r2 * p23;
    double p46 = p45 + r2 * c6;

    double poly = p03 + r4 * p46;

    union {
        double d;
        uint64_t i;
    } scale;
    scale.i = (uint64_t)(k + 1023) << 52;
    return poly * scale.d;
}
