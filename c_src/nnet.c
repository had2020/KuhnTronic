#include <math.h>
#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>

double sigmoid(double x) {return 1 / (1 + exp(-x)); }
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

struct dataset {
    double training_inputs[numTrainingSets][numInputs];
    double training_outputs[numTrainingSets][numInputs];
};

struct Model {
    double hiddenLayer[numHiddenNodes];
    double outputLayer[numOuputs];

    double hiddenLayerBias[numHiddenNodes];
    double outputLayerBias[numOuputs];

    double hiddenWeights[numInputs][numHiddenNodes];
    double outputWeights[numHiddenNodes][numOuputs];
};

void init(struct Model model) {
    for (int i = 0; i < numInputs; i++) {
        for (int j = 0; j < numHiddenNodes; j++) {
            model.hiddenWeights[i][j] = init_weights();
        }
    }

    for (int i = 0; i < numHiddenNodes; i++) {
        for (int j = 0; j < numOuputs; j++) {
            model.outputWeights[i][j] = init_weights();
        }
    }

   for (int i = 0; i < numOuputs; i++) {
       model.outputLayerBias[i] = init_weights();
   }
}

// learningRate works at 0.1
void train(uint64_t numEpochs, double learningRate, struct Model model, struct dataset dataset) {
    int trainingSetOrder[] = {0,1,2,3};

    for (int epoch = 0; epoch < numEpochs; epoch++) {

        shuffle(trainingSetOrder, numTrainingSets);

        for (int x = 0; x < numTrainingSets; x++) {
            int i = trainingSetOrder[x];

            // Forward pass

            // Compute hidden Layer Activation
            for (int j = 0; j < numHiddenNodes; j++){
                double activation = model.hiddenLayerBias[j];

                for (int k = 0; k < numInputs; k++) {
                    activation += dataset.training_inputs[i][k] * model.hiddenWeights[k][j];
                }
                model.hiddenLayer[j] = sigmoid(activation);
            }

            // Compute output layer activation
            for (int j = 0; j < numOuputs; j++){
                double activation = model.outputLayerBias[j];

                for (int k = 0; k < numHiddenNodes; k++) {
                    activation += model.hiddenLayer[k] * model.outputWeights[k][j];
                }
                model.outputLayer[j] = sigmoid(activation);
            }

            printf("Input: %g   Output: %g   Predicted Output: %g \n",
                dataset.training_inputs[i][0],
                model.outputLayer[0],
                dataset.training_outputs[i][0]
            );

            // Backprop

            // Compute change in output weights

            double deltaOutput[numOuputs];

            for (int j = 0; j < numOuputs; j++) {
                double error = (dataset.training_outputs[i][j] - model.outputLayer[j]);
                deltaOutput[j] = error * dSigmoid(model.outputLayer[j]);
            }

            // Compute change in hidden weights
            double deltaHidden[numHiddenNodes];
            for (int j = 0; j < numHiddenNodes; j++) {
                double error = 0.0f;
                for (int k = 0; k < numOuputs; k++) {
                    error += deltaOutput[k] * model.outputWeights[j][k];
                }
                deltaHidden[j] = error * dSigmoid(model.hiddenLayer[j]);
            }

            // Apply change in output weights
            for (int j = 0; j < numOuputs; j++) {
                model.outputLayerBias[j] += deltaOutput[j] * learningRate;
                for (int k = 0; k < numHiddenNodes; k++) {
                    model.outputWeights[k][j] += model.hiddenLayer[k] * deltaOutput[j] * learningRate;
                }
            }

            // Apply change in hidden weights
            for (int j = 0; j < numHiddenNodes; j++) {
                model.hiddenLayerBias[j] += deltaHidden[j] * learningRate;
                for (int k = 0; k < numInputs; k++) {
                    model.hiddenWeights[k][j] += dataset.training_inputs[i][k] * deltaHidden[j] * learningRate;
                }
            }



        }
    }
}

uintptr_t forwardPass(struct Model model, double inputs[numInputs]) {
    // Compute hidden Layer Activation
    for (int j = 0; j < numHiddenNodes; j++){
        double activation = model.hiddenLayerBias[j];

        for (int k = 0; k < numInputs; k++) {
            activation += inputs[k] * model.hiddenWeights[k][j];
        }
        model.hiddenLayer[j] = sigmoid(activation);
    }

    // Compute output layer activation
    for (int j = 0; j < numOuputs; j++){
        double activation = model.outputLayerBias[j];

        for (int k = 0; k < numHiddenNodes; k++) {
            activation += model.hiddenLayer[k] * model.outputWeights[k][j];
        }
        model.outputLayer[j] = sigmoid(activation);
    }

    return model.outputLayer[0];
}
