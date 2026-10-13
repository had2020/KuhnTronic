#include <stdlib.h>
#include <stdio.h>
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

double sigmoid(double x) {return 1 / (1 + fast_exp(-x)); }
double dSigmoid(double x) {return x * (1 - x); } // Derivative

double init_weights() { return ((double)rand()) / ((double)RAND_MAX); }

void shuffle(int *array, size_t n) {
    if (n > 1) {
        size_t i;
        for (i = 0; i < n-1; i++) {
            size_t j = i + rand() / (RAND_MAX / (n - i) + 1);
            int t = array[j];
            array[j] = array[i];
            array[i] = t;
        }
    }
}

#define numInputs 2
#define numHiddenNodes 2
#define numOuputs 1
#define numTrainingSets 4

int main(void) {

    const double learningRate = 0.1f;

    double hiddenLayer[numHiddenNodes];
    double outputLayer[numOuputs];

    double hiddenLayerBias[numHiddenNodes];
    double outputLayerBias[numOuputs];

    double hiddenWeights[numInputs][numHiddenNodes];
    double outputWeights[numHiddenNodes][numOuputs];


    // Training for XOR
    double training_inputs[numTrainingSets][numInputs] = {
        {0.0f, 0.0f},
        {1.0f, 0.0f},
        {0.0f, 1.0f},
        {1.0f, 1.0f}
    };

    double training_outputs[numTrainingSets][numOuputs] = {
        {0.0f},
        {1.0f},
        {1.0f},
        {0.0f}
    };

    for (int i = 0; i < numInputs; i++) {
        for (int j = 0; j < numHiddenNodes; j++) {
            hiddenWeights[i][j] = init_weights();
        }
    }

    for (int i = 0; i < numHiddenNodes; i++) {
        for (int j = 0; j < numOuputs; j++) {
            outputWeights[i][j] = init_weights();
        }
    }

   for (int i = 0; i < numOuputs; i++) {
       outputLayerBias[i] = init_weights();
   }

   int trainingSetOrder[] = {0,1,2,3};

   int numEpochs = 10000;

   // Train
   for (int epoch = 0; epoch < numEpochs; epoch++) {

       shuffle(trainingSetOrder, numTrainingSets);

       for (int x = 0; x < numTrainingSets; x++) {
           int i = trainingSetOrder[x];

           // Forward pass

           // Compute hidden Layer Activation
           for (int j = 0; j < numHiddenNodes; j++){
               double activation = hiddenLayerBias[j];

               for (int k = 0; k < numInputs; k++) {
                   activation += training_inputs[i][k] * hiddenWeights[k][j];
               }
               hiddenLayer[j] = sigmoid(activation);
           }

           // Compute output layer activation
           for (int j = 0; j < numOuputs; j++){
               double activation = outputLayerBias[j];

               for (int k = 0; k < numHiddenNodes; k++) {
                   activation += hiddenLayer[k] * outputWeights[k][j];
               }
               outputLayer[j] = sigmoid(activation);
           }

           printf("Input: %g   Output: %g   Predicted Output: %g \n",
               training_inputs[i][0],
               outputLayer[0],
               training_outputs[i][0]
           );

           // Backprop

           // Compute change in output weights

           double deltaOutput[numOuputs];

           for (int j = 0; j < numOuputs; j++) {
               double error = (training_outputs[i][j] - outputLayer[j]);
               deltaOutput[j] = error * dSigmoid(outputLayer[j]);
           }

           // Compute change in hidden weights
           double deltaHidden[numHiddenNodes];
           for (int j = 0; j < numHiddenNodes; j++) {
               double error = 0.0f;
               for (int k = 0; k < numOuputs; k++) {
                   error += deltaOutput[k] * outputWeights[j][k];
               }
               deltaHidden[j] = error * dSigmoid(hiddenLayer[j]);
           }

           // Apply change in output weights
           for (int j = 0; j < numOuputs; j++) {
               outputLayerBias[j] += deltaOutput[j] * learningRate;
               for (int k = 0; k < numHiddenNodes; k++) {
                   outputWeights[k][j] += hiddenLayer[k] * deltaOutput[j] * learningRate;
               }
           }

           // Apply change in hidden weights
           for (int j = 0; j < numHiddenNodes; j++) {
               hiddenLayerBias[j] += deltaHidden[j] * learningRate;
               for (int k = 0; k < numInputs; k++) {
                   hiddenWeights[k][j] += training_inputs[i][k] * deltaHidden[j] * learningRate;
               }
           }



       }
   }
}
