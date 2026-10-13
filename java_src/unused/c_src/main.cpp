#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include "time.h"
#include <unistd.h>
#include <vector>
#include "nnet.cpp"
#include "kuhnStates.cpp"

int main(int argc, char *argv[]) {

    struct Model model = {};
    init(model);

    struct dataset dataset;

    // TODO API 
    dataset.training_inputs.reserve(128);
    dataset.training_outputs.reserve(128);

    // 1. King as P1 (Opening move): Optimal strategy bets ~1/3 of the time, check remainder
    dataset.training_inputs.push_back({3.0, 1.0, 0.0, 0.0, 2.0, 0.0});
    dataset.training_outputs.push_back({0.33});

    // 2. King as P2 after P1 checks: Always bet for value
    dataset.training_inputs.push_back({3.0, 0.0, 1.0, 0.0, 2.0, 0.0});
    dataset.training_outputs.push_back({1.0});

    // 3. King as P2 facing a bet from P1: Always call (nut hand)
    dataset.training_inputs.push_back({3.0, 0.0, 2.0, 0.0, 3.0, 1.0});
    dataset.training_outputs.push_back({1.0});

    // 4. Jack as P1 (Opening move): Mixed bluffing strategy (~1/3 chance to bet/bluff)
    dataset.training_inputs.push_back({1.0, 1.0, 0.0, 0.0, 2.0, 0.0});
    dataset.training_outputs.push_back({0.33});

    // 5. Jack as P1 facing a bet after checking: Always fold
    dataset.training_inputs.push_back({1.0, 1.0, 1.0, 2.0, 3.0, 1.0});
    dataset.training_outputs.push_back({0.0});

    // 6. Jack as P2 facing a bet from P1: Always fold
    dataset.training_inputs.push_back({1.0, 0.0, 2.0, 0.0, 3.0, 1.0});
    dataset.training_outputs.push_back({0.0});

    // 7. Queen as P1 (Opening move): Always check
    dataset.training_inputs.push_back({2.0, 1.0, 0.0, 0.0, 2.0, 0.0});
    dataset.training_outputs.push_back({0.0});

    // 8. Queen as P2 facing a bet from P1: Fold (pure call with Queen loses long-term)
    dataset.training_inputs.push_back({2.0, 0.0, 2.0, 0.0, 3.0, 1.0});
    dataset.training_outputs.push_back({0.0});

    // 9. Queen as P2 after P1 checks: Check back (showdown)
    dataset.training_inputs.push_back({2.0, 0.0, 1.0, 0.0, 2.0, 0.0});
    dataset.training_outputs.push_back({0.0});

    // 10. Queen as P1 after checking and P2 bets: Call roughly 1/3 of the time (indifference)
    dataset.training_inputs.push_back({2.0, 1.0, 1.0, 2.0, 3.0, 1.0});
    dataset.training_outputs.push_back({0.33});

    train(10000, 0.1, model, dataset);

    double output[1];
    forwardPass(model, dataset.training_inputs[0].data(), output);

    if (argc == 2) {
        printf("Raw debugging mode enabled. Input first State as a number: \n");

        while(1) {
            int input_s;
            if (scanf("%d", &input_s)) {
                printf("%d\n", node_tree[input_s]);
            } else {
                printf("Input failed to read!");
                break;
            }

        }

    } else {
        FILE* fptr;

        fptr = fopen("../temp.swp", "r");

        if (fptr == NULL) {
            printf("Error openning file!");
        } else {
            fclose(fptr);
        }


        printf("The C interface is running, press control+c to kill it.\n");
        while (1) {
            usleep(1000000);
            break; //TODO remove break, and communicate
        }
    }

    return 0;
}
