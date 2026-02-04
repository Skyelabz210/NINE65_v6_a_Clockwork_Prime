# QMNF One-Shot Learning Architecture

## 1. Overview

This document details the novel one-shot learning architecture implemented within the Quantum-Modular Numerical Framework (QMNF) System. This architecture represents a significant departure from traditional deep learning paradigms that rely on iterative, gradient-based optimization.

The core philosophy is to learn a robust model for a class from a **single, high-quality example (an "exemplar")**. This is achieved through a deterministic, integer-only process of **systematic perturbation** and **consensus-based classification**. The entire framework is designed to be data-efficient, reproducible, and free from the complexities and potential errors of floating-point arithmetic.

## 2. Core Components

The architecture is built around two primary components: the `OneShotLearner` and the `ConsensusClassifier`.

### `OneShotLearner`

The `OneShotLearner` is the main entry point for the "training" process. Its responsibility is to take a set of exemplars (one for each class) and produce a `ConsensusClassifier` that can distinguish between them.

### `ConsensusClassifier`

The `ConsensusClassifier` is the final "model". It stores a "template" for each class and uses a consensus mechanism to classify new inputs. It does not have any "weights" in the traditional sense; its knowledge is stored entirely within the class templates.

## 3. The "Training" Process: Template Generation

In this architecture, "training" is not an iterative process of adjusting weights. Instead, it is a one-shot procedure to generate a robust template for each class. This process has two main steps:

### Step 1: Systematic Perturbation

-   **Goal:** To create a rich, synthetic dataset from a single exemplar.
-   **Process:** The `perturbation_variants` function takes a single exemplar (represented as a vector in residue space) and generates hundreds or thousands of **synthetic variants**.
-   **Mechanism:** It adds small, deterministic "noise" to each channel of the exemplar's residue representation. This creates a "cloud" of data points in the residue space that are all similar to the original exemplar. The size of this cloud is controlled by the `perturbation_radius` parameter.
-   **Benefit:** This step allows the system to learn a generalized representation of the class that is robust to small variations and noise, without requiring a large dataset of real examples.

### Step 2: Template Extraction

-   **Goal:** To find the central point of the synthetic data cloud.
-   **Process:** The `extract_template` function takes the set of synthetic variants and computes a single **class template**.
-   **Mechanism:** It does this by calculating the **modular median** for each channel across all the variants. The median is used because it is a robust statistical measure that is less sensitive to outliers than the mean.
-   **Result:** The final template is a single vector in residue space that represents the "ideal" or "average" version of the class, as derived from the perturbed variants.

## 4. The Classification Process: Consensus

Once a template for each class has been generated, the `ConsensusClassifier` can be used to classify new, unseen inputs.

-   **Goal:** To determine which class template an input is most similar to.
-   **Process:** The `classify` method takes a new input and computes a **consensus score** between the input and each stored class template.
-   **Mechanism:** The consensus score is a measure of agreement, or similarity, in the residue space. It is calculated by summing the modular distance between the input and the template for each corresponding channel. A smaller distance results in a higher consensus score.
-   **Result:** The class whose template has the highest consensus score with the input is chosen as the predicted class.

## 5. Mathematical Foundations and Key Benefits

-   **Integer-Only Arithmetic:** All calculations—perturbation, median, and distance—are performed using exact modular arithmetic. This adheres to the core principle of the QMNF System and avoids any potential for floating-point contamination.
-   **Data Efficiency:** The architecture is designed to learn from a minimal amount of data (a single exemplar per class), making it suitable for scenarios where data is scarce.
-   **Determinism and Reproducibility:** The entire process is deterministic. Given the same exemplars and parameters, the system will always produce the exact same classifier and the same predictions. This is a critical advantage for applications that require verifiable and reproducible results.
-   **Robustness:** The systematic perturbation process builds a degree of noise and variation tolerance into the class templates, making the classifier robust to minor variations in the input.

## 6. Limitations and Future Work

The current implementation of the one-shot learning architecture has some known limitations, which represent opportunities for future research and development.

### Learning Abstract Mathematical Properties

The `ConsensusClassifier`, in its current form, relies on a simple modular distance metric to measure the similarity between an input and a class template. While this is effective for many classification tasks, it has been shown to be insufficient for learning abstract mathematical properties like **parity** (even vs. odd).

In tests, the classifier was unable to reliably distinguish between even and odd numbers, even with carefully crafted exemplars. This is because the classifier is learning a "region" in the residue space, rather than an abstract rule. An input vector of odd numbers may be "closer" in modular distance to the "even" template than to the "odd" template, leading to misclassification.

This limitation highlights the need for a more sophisticated consensus algorithm that can capture abstract relationships in the data, rather than just measuring point-wise similarity.

### Future Work

-   **Improve the consensus algorithm:** Research and implement more advanced consensus algorithms that can capture abstract properties. This could involve looking at the *pattern* of distances, the differences between values, or incorporating other mathematical properties into the calculation.
-   **Explore feature extraction:** For tasks that require abstract reasoning, a pre-processing step could be added to extract relevant features from the input data. For example, a feature extraction step could explicitly calculate the parity of each number in a vector.
-   **Develop hierarchical classification:** For a large number of classes, a hierarchical classification scheme could be implemented to improve performance and scalability.
-   **Adaptive Perturbation:** The perturbation radius is currently fixed. Researching and implementing a method to adapt the radius based on the data could improve the robustness of the templates.
